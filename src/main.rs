use std::{fs::File, io::Read};

mod parser;

// MIDI Spec:
// https://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
// https://www.lim.di.unimi.it/IEEE/MIDI/META.HTM

fn main() {
    // let path = "Test MIDIs/Megalovania.mid";
    let path = "Test MIDIs/Nyan Cat.mid";

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

    let midi_file = match parser::parse_midi_file(&buf) {
        Ok(f) => f,
        Err(e) => panic!("{}", e)
    };

    // if let parser::chunk::Chunk::MTrk(trk) = &midi_file.tracks[1] {
    //     println!("{}", trk.len());
    // }

    println!("{:#?}", midi_file);
}
