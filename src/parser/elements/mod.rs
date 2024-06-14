//! A module defining all the parts a MIDI file consists of.

// Data wrappers
pub mod keyname;
pub mod key_signature;

// Elements of the MIDI file format
pub mod midi_event;
pub mod meta_event;
pub mod chunk;
