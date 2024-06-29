use std::{fs::{File, OpenOptions}, io::{Read, Write}};

mod parser;

// MIDI Spec:
// https://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
// https://www.lim.di.unimi.it/IEEE/MIDI/

fn main() {
    let path = "Test MIDIs/Megalovania.mid";
    // let path = "Test MIDIs/Nyan Cat.mid";
    // let path = "Test MIDIs/U.N. Owen was her.mid";

    // Open file
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e)
    };

    // Read file into a buffer
    let mut buf = Vec::<u8>::new();
    let read_res = file.read_to_end(&mut buf);
    if let Err(e) = read_res {
        panic!("{}", e);
    }

    // API call
    let midi_file = match parser::parse_midi_file(&buf) {
        Ok(f) => f,
        Err(e) => panic!("{}", e)
    };

    let mut out_file = OpenOptions::new().write(true).open("out.txt").unwrap();
    out_file.write(format!("{midi_file}").as_bytes()).unwrap();
}
