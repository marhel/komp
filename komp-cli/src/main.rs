use komp_core::*;
use std::env;
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
mod play;

fn main() {
    println!("komp");

    let current_chord: Arc<Mutex<Option<Chord>>> = Arc::new(Mutex::new(None));
    let read_current_chord = Arc::clone(&current_chord);

    let source_index = get_source_index();
    let destination_index = get_destination_index();

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

    let mut playing: Vec<(u8, u8)> = vec![];
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
        let d = Duration::from_millis(3000);
        let mut last_key = None;
        let mut timestamp;
        loop {
            let current_key = *read_current_chord.lock().unwrap();
            if last_key != current_key {
                println!("T: {:?}", current_key);
                if last_key.is_none() {
                    unsafe {
                        timestamp = Some(AudioConvertHostTimeToNanos(AudioGetCurrentHostTime()));
                    }
                    coremidi::flush().expect("cannot flush coremidi queued events");
                    let packet_buf =
                        play::schedule_music(timestamp.unwrap(), *current_key.unwrap().key());
                    output_port
                        .send(&destination, &packet_buf)
                        .expect("cannot send MIDI packet");
                }
                last_key = current_key;
            } else {
                print!(".")
            }
            thread::sleep(d);
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

fn detect_chord(
    was_playing: &mut Vec<(u8, u8)>,
    playing: &mut Vec<(u8, u8)>,
    current_chord_mutex: &Arc<Mutex<Option<Chord>>>,
) {
    was_playing.sort();
    was_playing.dedup();
    playing.sort();
    playing.dedup();
    let was_modified = was_playing.len() != playing.len()
        || was_playing
            .iter()
            .zip(playing.clone())
            .any(|(a, b)| *a != b);

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

fn process_midi<'a>(data: &[u8], playing: &'a mut Vec<(u8, u8)>) -> &'a Vec<(u8, u8)> {
    if data.len() == 1 && data[0] == ACTIVE_SENSE {
        return playing;
    }

    if data.len() != 3 {
        println!("packet not length 3; {:?})", data);
        return playing;
    }

    let channel = data[0] & CHANNEL_MASK;
    let command = data[0] & COMMAND_MASK;
    let note = data[1];
    let velocity = data[2];
    match command {
        CONTROLLER => (), // controller
        NOTE_ON if velocity > 0 => {
            playing.push((channel, note));
        }
        NOTE_ON | NOTE_OFF if command != NOTE_ON || velocity == 0 => {
            playing.retain(|(ch, n)| channel != *ch || note != *n);
        }
        _ => println!("Unknown command {} in packet {:?}", command, data),
    };
    playing
}

fn get_source_index() -> usize {
    let mut args_iter = env::args();
    let tool_name = args_iter
        .next()
        .and_then(|path| {
            path.split(std::path::MAIN_SEPARATOR)
                .last()
                .map(|v| v.to_string())
        })
        .unwrap_or("komp".to_string());

    match args_iter.next() {
        Some(arg) => match arg.parse::<usize>() {
            Ok(index) => {
                if index >= coremidi::Sources::count() {
                    println!("Source index out of range: {}", index);
                    std::process::exit(-1);
                }
                index
            }
            Err(_) => {
                println!("Wrong source index: {}", arg);
                std::process::exit(-1);
            }
        },
        None => {
            println!("Usage: {} <source-index>", tool_name);
            println!("");
            println!("Available Sources:");
            print_sources();
            std::process::exit(-1);
        }
    }
}

fn print_sources() {
    for (i, source) in coremidi::Sources.into_iter().enumerate() {
        match source.display_name() {
            Some(display_name) => println!("[{}] {}", i, display_name),
            None => (),
        }
    }
}

fn get_destination_index() -> usize {
    let mut args_iter = env::args();
    let tool_name = args_iter
        .next()
        .and_then(|path| {
            path.split(std::path::MAIN_SEPARATOR)
                .last()
                .map(|v| v.to_string())
        })
        .unwrap_or("send".to_string());

    match args_iter.next() {
        Some(arg) => match arg.parse::<usize>() {
            Ok(index) => {
                if index >= coremidi::Destinations::count() {
                    println!("Destination index out of range: {}", index);
                    std::process::exit(-1);
                }
                index
            }
            Err(_) => {
                println!("Wrong destination index: {}", arg);
                std::process::exit(-1);
            }
        },
        None => {
            println!("Usage: {} <destination-index>", tool_name);
            println!("");
            println!("Available Destinations:");
            print_destinations();
            std::process::exit(-1);
        }
    }
}

fn print_destinations() {
    for (i, destination) in coremidi::Destinations.into_iter().enumerate() {
        match destination.display_name() {
            Some(display_name) => println!("[{}] {}", i, display_name),
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_on() {
        let data = vec![0x93, 0x3c, 0x40];
        let mut playing = vec![];
        process_midi(&data, &mut playing);
        assert_eq!(playing, vec![(0x03, 0x3c)]);
    }
    #[test]
    fn test_note_off() {
        let data = vec![0x83, 0x3c, 0x40];
        let mut playing = vec![(0x03, 0x3c), (0x03, 0x40), (0x04, 0x3c)];
        process_midi(&data, &mut playing);
        assert_eq!(playing, vec![(0x03, 0x40), (0x04, 0x3c)]);
    }
    #[test]
    fn test_note_on_velocity_0() {
        let data = vec![0x93, 0x3c, 0x00];
        let mut playing = vec![(0x03, 0x3c), (0x03, 0x40), (0x04, 0x3c)];
        process_midi(&data, &mut playing);
        assert_eq!(playing, vec![(0x03, 0x40), (0x04, 0x3c)]);
    }
}
