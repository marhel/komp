use komp_core::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod external {
    #[link(name = "CoreAudio", kind = "framework")]
    extern "C" {
        pub fn AudioConvertHostTimeToNanos(inHostTime: u64) -> u64;
        pub fn AudioGetCurrentHostTime() -> u64;
    }
}
use crate::external::{AudioConvertHostTimeToNanos, AudioGetCurrentHostTime};
mod pattern;
mod play;
mod setup;

use crate::play::*;
use crate::setup::*;
use std::env;

fn main() {
    println!("komp");

    let current_chord: Arc<Mutex<Option<Chord>>> = Arc::new(Mutex::new(None));
    let read_current_chord = Arc::clone(&current_chord);

    let mut args_iter = env::args();
    let tool_name = tool_name(&mut args_iter);

    let source_index = get_source_index(&mut args_iter, &tool_name);
    let destination_index = get_destination_index(&mut args_iter, &tool_name);

    let source = coremidi::Source::from_index(source_index)
        .expect(&format!("cannot get coremidi source[{}]", source_index));
    let source_name = source
        .display_name()
        .expect("cannot get coremidi source name");
    println!("Using source[{}] <{}>", source_index, source_name);

    let destination = coremidi::Destination::from_index(destination_index).expect(&format!(
        "cannot get coremidi destination[{}]",
        destination_index
    ));
    let destination_name = destination
        .display_name()
        .expect("cannot get coremidi destination name");
    println!(
        "Using destination[{}] <{}>",
        destination_index, destination_name
    );

    let client = coremidi::Client::new("komp-client").expect("cannot create coremidi client");

    let mut playing: Playing = HashSet::new();
    let receive_midi = move |packet_list: &coremidi::PacketList| {
        let mut was_playing = playing.clone();

        for packet in packet_list.iter() {
            let data = packet.data();
            process_midi(data, &mut playing);
        }

        detect_chord(&mut was_playing, &mut playing, &current_chord);
    };

    let input_port = client
        .input_port("komp-port", receive_midi)
        .expect("cannot create input port");
    input_port
        .connect_source(&source)
        .expect("cannot connect input port to source");
    let output_port = client.output_port("komp-port").unwrap();

    let _handle = thread::spawn(move || {
        let ticks_per_quarter = 96;
        let us_per_quarter = 500_000;
        let mut last_key = None;
        let mut timestamp = now();
        let timed_events = pattern::create_bar(ticks_per_quarter, Chord::MajorMaj7(C_KEY));
        let slice_length = 200 * NS_PER_MS;
        let pattern_length = 4 * us_per_quarter as u64 * NS_PER_US;
        let scheduling_deadline_margin = 50 * NS_PER_MS;

        let mut scheduler = play::Scheduler::new(
            timestamp,
            slice_length,
            scheduling_deadline_margin,
            timed_events,
            pattern_length,
            us_per_quarter,
            ticks_per_quarter,
        );

        let mut slice_start = 0;
        loop {
            timestamp = now();
            let current_key = *read_current_chord.lock().unwrap();
            if last_key != current_key {
                println!("T: {:?}", current_key);
                last_key = current_key;
            } else {
                print!(".")
            }
            let (sleep_time, packet_buf) = scheduler.schedule_slice(
                timestamp,
                &mut slice_start,
                current_key.map_or_else(|| C_KEY, |ch| *ch.key()),
            );
            output_port
                .send(&destination, &packet_buf)
                .expect("cannot send MIDI packet");
            if sleep_time > 0 {
                thread::sleep(Duration::from_nanos(sleep_time as u64));
            } else {
                println!("unexpected delay {}", sleep_time);
            }
        }
    });

    let mut input_line = String::new();
    println!("Press [Enter] to finish ...");
    std::io::stdin()
        .read_line(&mut input_line)
        .ok()
        .expect("cannot read line");

    input_port
        .disconnect_source(&source)
        .expect("cannot disconnect input port from source");
    println!("disconnected from source <{}>", source_name);
}

fn now() -> u64 {
    unsafe { AudioConvertHostTimeToNanos(AudioGetCurrentHostTime()) }
}

fn detect_chord(
    was_playing: &mut Playing,
    playing: &mut Playing,
    current_chord_mutex: &Arc<Mutex<Option<Chord>>>,
) {
    let was_modified = was_playing != playing;

    let name = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    if was_modified && playing.len() > 0 {
        let notes = playing.iter().map(|(_ch, note)| *note).collect();
        match komp_core::detect_chord(&notes).first() {
            Some(chord) => {
                print!("{:?} <= [ ", chord);
                for note in notes {
                    print!("{}{} ", name[(note % 12) as usize], note / 12)
                }
                println!("]");
                let mut current_chord = current_chord_mutex.lock().unwrap();
                *current_chord = Some(*chord);
            }
            None => {
                print!("no chord recognized in [ ");
                for note in notes {
                    print!("{}{} ", name[(note % 12) as usize], note / 12)
                }
                println!("]");
                let mut current_chord = current_chord_mutex.lock().unwrap();
                *current_chord = None;
            }
        };
    }
}

const ACTIVE_SENSE: u8 = 0xFE;
const COMMAND_MASK: u8 = 0xF0;
const CHANNEL_MASK: u8 = 0x0F;
const CONTROLLER: u8 = 0xB0;
const NOTE_ON: u8 = 0x90;
const NOTE_OFF: u8 = 0x80;
use std::collections::HashSet;

type ChannelNote = (u8, u8);
type Playing = HashSet<ChannelNote>;

fn process_midi<'a>(data: &[u8], playing: &'a mut Playing) {
    extract_playing_notes(data, playing, false)
}

fn extract_playing_notes<'a>(data: &[u8], playing: &'a mut Playing, accumulate_notes: bool) {
    if data.len() == 1 && data[0] == ACTIVE_SENSE {
        return;
    }

    if data.len() != 3 {
        println!("packet not length 3; {:?})", data);
        return;
    }

    let channel = data[0] & CHANNEL_MASK;
    let command = data[0] & COMMAND_MASK;
    let note = data[1];
    let velocity = data[2];
    match command {
        CONTROLLER => (), // controller
        NOTE_ON if velocity > 0 => {
            playing.insert((channel, note));
        }
        NOTE_ON | NOTE_OFF if command != NOTE_ON || velocity == 0 => {
            if !accumulate_notes {
                playing.remove(&(channel, note));
            }
        }
        _ => println!("Unknown command {} in packet {:?}", command, data),
    };
}

#[macro_export]
macro_rules! hashset {
    ($($x:expr),*) => {
        {
            let mut p = HashSet::new();
            $(p.insert($x);)*
            p
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_on() {
        let data = vec![0x93, 0x3c, 0x40];
        let mut playing = HashSet::new();
        process_midi(&data, &mut playing);
        assert_eq!(playing, hashset![(0x03, 0x3c)]);
    }

    #[test]
    fn test_note_off() {
        let data = vec![0x83, 0x3c, 0x40];
        let mut playing = hashset![(0x03, 0x3c), (0x03, 0x40), (0x04, 0x3c)];
        process_midi(&data, &mut playing);
        assert_eq!(playing, hashset![(0x03, 0x40), (0x04, 0x3c)]);
    }
    #[test]
    fn test_note_on_velocity_0() {
        let data = vec![0x93, 0x3c, 0x00];
        let mut playing = hashset![(0x03, 0x3c), (0x03, 0x40), (0x04, 0x3c)];
        process_midi(&data, &mut playing);
        assert_eq!(playing, hashset![(0x03, 0x40), (0x04, 0x3c)]);
    }
}
