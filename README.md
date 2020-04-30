# komp

The `komp` is a utility for musicians. It consists of the command line tool `komp` and a Rust library `komp-core`.

## Features
* Chord detection (recognizes over 30 different chord types in all 12 keys, or over 400 chords in total), see the list below.
* [Planned] Auto-accompaniment (adding rhythm, bass and chord playback matching the recognized chord)
* [Planned] Auto-accompaniment created from Yamaha Style Files.

## Misfeatures
Due to being a work in progress, it currently isn't very useful and not even very fun to play along with. When connected
to a MIDI source and destination it detects the chords played and outputs this to the console. Then it starts to play a
Major Maj7 chord in the detected key each quarter over the notes you are playing.

Unfortunately the main drawback is that it is currently extremely twitchy, as it reacts to every individual key in the
chord, so while it does detects the correct chord when all keys are pressed, it also tries to detect after each key is
pressed. There's typically a delay of a couple of milliseconds between depression of each key, and 5 to 30ms between the
first and last key of the chord, so for the computer there's plenty of time to react to each individual key.

Later I'll add a delay for the detection of a new chord to make it less twitchy.

## Platform dependence
The chord-detection library is platform agnostic, but leans on the MIDI specification for the mapping from keys and notes to numbers.

The command-line tool is currently MacOS only as it relies getting MIDI data via [the coremidi crate](https://crates.io/crates/coremidi) which provides Rust bindings for the [CoreMIDI macOS framework](https://developer.apple.com/reference/coremidi) that provides macOS APIs for communicating with MIDI devices, such as hardware synthesizers or keyboards.

## Near realtime timing requirements
I considered using [the midir crate](https://github.com/Boddlnagg/midir) which provides cross-platform MIDI processing. However, this abstraction does not support scheduling sending MIDI events. This is needed as we have near realtime timing requirements.

Ideally, we want sub millisecond precision timing, because more than about 10 to 20 milliseconds offset from the expected timing is easily detected by ear, so we simply cannot rely on thread::sleep as it has no such precise guarantees.

However, CoreMIDI supports scheduling sending, which offloads the realtime aspect to macOS and we can just schedule the coming half second of notes or so ahead of time and then thread::sleep for a while.

## Recognized chords
I'm not well versed in musical theory, and there are certainly variations in how chords are written. The fingerings are mostly taken from the chord list of page 46 of [the Yamaha Tyros3 Reference Manual](https://uk.yamaha.com/files/download/other_assets/4/314194/tyros3_en_rm_v10a.pdf).

The played notes are normalized into a single octave, and de-duplicated - so it does not matter if you play C-E-G-Bb-D, or C-D-E-G-Bb, or even C-D-E-G-Bb-C, all will be recognized as a C7(9). As a simpler example of deduplication, C-E-G-C will be recognized as a C Major chord as the second C is ignored.

Examples and explanations are in the C key, but the chord recognition handles chords in any key, including all possible inversions (changing the fingering of the chord by playing some notes in an octave above or below, often for practical purposes during a chord progression). So for example C-F-A will be unambiguously recognized as a F major chord, even though the third note in the chord was voiced an octave lower then the uninverted F major of F-A-C.

These are the recognized chords, and their names, and (uninverted) fingerings:

* Major (C) C-E-G
* Minor (Cm) C-Eb-G
* Fifth (C5) C-G

### Augmented
* Augmented (Caug) C-E-G#
* Augmented with a minor seventh (Caug7) C-E-G#-Bb
* Augmented major with a major seventh (Cmaj7aug) C-E-G#-B

### Diminished
* Diminished (Cdim) C-Eb-Gb
* Diminished with seventh (Cdim7) C-Eb-Gb-A

### Suspended
* Suspended second (Csus2) C-D-G
* Suspended fourth (Csus4) C-F-G
* Seven with suspended fourth (C7sus4) C-F-G-Bb

### Sixths
* Major sixth (C6) C-E-G-A
* Minor sixth (Cm6) C-Eb-G-A
* Major sixth with a ninth (C6(9)) C-E-G-A-D
* Minor sixth with a ninth (Cm6(9)) C-Eb-G-A-D

### Minor sevenths (7)
* Major with a minor seventh (C7) C-E-G-Bb
* Major with a minor seventh and a flat ninth (C7b9) C-E-G-Bb-Db
* Major with a minor seventh and ninth (C7(9)) C-E-G-Bb-D
* Major with a minor seventh and a sharp ninth (C7#9) C-E-G-Bb-D#
* Major with a minor seventh and a sharp eleventh (C7#11) C-E-G-Bb-F#
* Major with a minor seventh and a flat thirteenth (C7b13) C-E-G-Bb-Ab
* Major with a minor seventh and a thirteenth (C7(13)) C-E-G-Bb-A
* Minor with a minor seventh C-Eb-G-Bb
* Minor with a minor seventh and a ninth (Cm7(9)) C-Eb-G-Bb-D
* Minor with a minor seventh and an eleventh (Cm7(11)) C-Eb-G-F
* Major with a minor seventh and a flat fifth (C7b5) C-E-Gb-Bb
* Minor with a minor seventh and a flat fifth (Cm7b5) C-Eb-Gb-Bb

### Major sevenths (maj7)
* Major with a major seventh (Cmaj7) C-E-G-B
* Major with a major seventh and a ninth (Cmaj7(9)) C-E-G-B-D
* Major with a major seventh and a sharp eleventh (Cmaj7#11) C-E-G-B-F#
* Minor with a major seventh (Cm maj7) C-Eb-G-B
* Minor with a major seventh and a ninth (Cm maj7(9)) C-Eb-G-B-D

### Added ninths (without a seven)
* Major with an added ninth (Cadd9) C-E-G-D
* Minor with an added ninth (Cm add9) C-Eb-G-D

See also [pianochord.org](https://www.pianochord.org) for more details on the chords and the theory behind it.

### Resolving alternate chord interpretations
Some chord inversions, however are identical to other chords (both non-inverted, or one of the inversions) in which case one must be selected over the other.

Some examples of this happening are:

* Am6 with the F# as the lowest key is identical to an uninverted F#m7b5 chord.
* A7b5 with a low D# is identical to D#7b5.
* Am7 with a low C is equal to a C6.
* Am7(11) with a low C is also a C6(9).
* Adim7 with a low C is the same as Cdim7 and also the same as a D#dim7 played with a low C which is the same as a F#dim7 with a low C.
* An ASus2 with a low E is equal to a ESus4.
* AAug with a low C# equals C#Aug equals FAug with a low C#.

The rules for chord resolution are basically the following:

* less inverted chords are preferred over more inverted ones.
* sevens are preferred over sixes
* sus4 is preferred over sus2

so in the example collisions above the following happens;

* any uninverted chord is recognized as-is.
* an inverted F#m7b5 is preferred over an inverted Am6 (but Am6 is recognized if played uninverted).
* an inverted D#7b5 is preferred over an inverted A7b5 (but A7b5 is recognized if played uninverted).
* an inverted Am7 is preferred over an inverted C6 (but C6 is recognized if played uninverted).
* an inverted Am7(11) is preferred over an inverted C6(9) (but C6(9) is recognized if played uninverted).
* an inverted ESus4 is preferred over an inverted ASus2 (but ASus2 is recognized if played uninverted).
* uninverted dim7 and aug chords are preferred over their inverted versions.