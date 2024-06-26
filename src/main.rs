use std::time::Duration;

use console::Term;
use midir::MidiInput;

const MIDI_NOTE_ON: u8 = 144;
const MIDI_NOTE_OFF: u8 = 128;

const CHORDS: [(&str, &[u8]); 29] = [
    ("M", &[0, 4, 7]),                 // Major
    ("maj7", &[0, 4, 11]),             // Major 7th
    ("maj7", &[0, 4, 7, 11]),          // Major 7th
    ("maj9", &[0, 4, 11, 14]),         // Major 9th
    ("maj9", &[0, 4, 7, 11, 14]),      // Major 9th
    ("maj11", &[0, 4, 11, 17]),        // Major 11th
    ("maj11", &[0, 4, 7, 11, 14, 17]), // Major 11th
    ("6", &[0, 4, 9]),                 // Major 6th
    ("6", &[0, 4, 7, 9]),              // Major 6th
    ("7", &[0, 4, 10]),                // Dominant 7th
    ("7", &[0, 4, 7, 10]),             // Dominant 7th
    ("sus4", &[0, 5]),                 // Suspended 4th
    ("sus4", &[0, 5, 7]),              // Suspended 4th
    ("sus2", &[0, 2]),                 // Suspended 2nd
    ("sus2", &[0, 2, 7]),              // Suspended 2nd
    ("m", &[0, 3, 7]),                 // Minor
    ("m7", &[0, 3, 10]),               // Minor 7th
    ("m7", &[0, 3, 7, 10]),            // Minor 7th
    ("m6", &[0, 3, 8]),                // Minor 6th
    ("m6", &[0, 3, 7, 8]),             // Minor 6th
    ("m9", &[0, 3, 10, 14]),           // Minor 9th
    ("m9", &[0, 3, 7, 10, 14]),        // Minor 9th
    ("dim", &[0, 3, 6]),               // Diminished
    ("dim7", &[0, 3, 6, 9]),           // Diminished 7th
    ("m7b5", &[0, 3, 6, 10]),          // Half-diminished 7th
    ("5", &[0, 7]),                    // Power chord
    ("aug", &[0, 4, 8]),               // Augmented
    ("aug7", &[0, 4, 8, 10]),          // Augmented 7th
    ("maj7#5", &[0, 4, 8, 11]),        // Augmented Major 7th
];

fn chord(notes: &Vec<u8>) -> Option<String> {
    let root = notes[0];
    let diffs: Vec<u8> = notes.iter().map(|&e| e - root).collect();

    for (name, intervals) in CHORDS {
        if diffs == intervals {
            return Some(format!("{}{}", note(root, false), name));
        }
    }
    None
}

fn note(midi_num: u8, include_octave: bool) -> String {
    let notes = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    let octave = (midi_num / 12) - 1;
    let note_index = (midi_num % 12) as usize;

    if include_octave {
        format!("{}{}", notes[note_index], octave)
    } else {
        format!("{}", notes[note_index])
    }
}

fn main() {
    let term = Term::stdout();

    let midi_in = MidiInput::new("midir").expect("midi init failed");
    let in_ports = midi_in.ports();
    if in_ports.len() == 0 {
        eprintln!("No midi devices found");
        std::process::exit(1);
    }

    let dev = &in_ports[0];
    let dev_name = midi_in.port_name(dev).expect("failed to get device name");

    let mut notes_on: Vec<u8> = Vec::new();

    let handle_event = move |_stamp: u64, msg: &[u8], _data: &mut ()| {
        let msg_type = msg[0];
        let midi_num = msg[1];
        match msg_type {
            MIDI_NOTE_ON => notes_on.push(midi_num),
            MIDI_NOTE_OFF => notes_on.retain(|&e| e != midi_num),
            _ => {}
        }

        if notes_on.len() > 1 {
            term.clear_last_lines(1).unwrap();

            notes_on.sort();
            println!(
                "Currently playing: {}",
                chord(&notes_on).unwrap_or("???".to_string())
            );
        }
    };

    let _conn = midi_in
        .connect(dev, "midir-read-input", handle_event, ())
        .expect(format!("could not connect to {}", dev_name).as_str());
    println!("Successfully connected to {}\n", dev_name);

    let poll_rate = Duration::from_millis(100);
    loop {
        std::thread::sleep(poll_rate);
    }
}
