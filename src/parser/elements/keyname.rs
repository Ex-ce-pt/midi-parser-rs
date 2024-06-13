use std::fmt;
use std::fmt::{Debug, Display, Formatter};
// use std::result;

use super::key_signature::KeySignature;

pub const NUMBER_OF_KEYS: u8 = 12;

#[derive(Clone, Copy)]
pub enum KeyName {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B
}

impl KeyName {
    
    pub fn try_from_index(idx: u8) -> Option<Self> {
        match idx {
            0 => Some(Self::C),
            1 => Some(Self::CSharp),
            2 => Some(Self::D),
            3 => Some(Self::DSharp),
            4 => Some(Self::E),
            5 => Some(Self::F),
            6 => Some(Self::FSharp),
            7 => Some(Self::G),
            8 => Some(Self::GSharp),
            9 => Some(Self::A),
            10 => Some(Self::ASharp),
            11 => Some(Self::B),
            _ => None
        }
    }
    
    pub fn into_index(&self) -> usize {
        *self as usize
    }
    
    // TODO: From<T>?
    // pub fn try_from_name(name: &str) -> Option<Self> {
    //     match name {
    //         "C" => Some(Self::C),
    //         "C#" => Some(Self::CSharp),
    //         "D" => Some(Self::D),
    //         "D#" => Some(Self::DSharp),
    //         "E" => Some(Self::E),
    //         "F" => Some(Self::F),
    //         "F#" => Some(Self::FSharp),
    //         "G" => Some(Self::G),
    //         "G#" => Some(Self::GSharp),
    //         "A" => Some(Self::A),
    //         "A#" => Some(Self::ASharp),
    //         "B" => Some(Self::B),
    //         _ => None,
    //     }
    // }
    
    pub fn try_from_name(name: &str) -> Option<Self> {
        match name {
            "B#" => Some(Self::C),
            "C"  => Some(Self::C),
            "C#" => Some(Self::CSharp),
            "Db" => Some(Self::CSharp),
            "D"  => Some(Self::D),
            "D#" => Some(Self::DSharp),
            "Eb" => Some(Self::DSharp),
            "E"  => Some(Self::E),
            "E#" => Some(Self::F),
            "Fb" => Some(Self::E),
            "F"  => Some(Self::F),
            "F#" => Some(Self::FSharp),
            "Gb" => Some(Self::FSharp),
            "G"  => Some(Self::G),
            "G#" => Some(Self::GSharp),
            "Ab" => Some(Self::GSharp),
            "A"  => Some(Self::A),
            "A#" => Some(Self::ASharp),
            "Bb" => Some(Self::ASharp),
            "B"  => Some(Self::B),
            _    => None,
        }
    }
    
    pub fn into_str(&self) -> &'static str {
        match self {
            Self::C      => "C",
            Self::CSharp => "C#",
            Self::D      => "D",
            Self::DSharp => "D#",
            Self::E      => "E",
            Self::F      => "F",
            Self::FSharp => "F#",
            Self::G      => "G",
            Self::GSharp => "G#",
            Self::A      => "A",
            Self::ASharp => "A#",
            Self::B      => "B",
        }
    }

    pub fn into_str_in_scale(&self, scale: &KeySignature) -> &'static str {
        todo!();
    }
    
}

// MIDI convertions

impl KeyName {

    pub fn try_from_midi_keycode(value: u8) -> Result<(Self, i8), ()> {
        if value > 127 {
            return Err(());
        }
        
        let keyname = KeyName::try_from_index(value % 12).unwrap();
        let octave = (value / NUMBER_OF_KEYS as u8) as i8 - 1;
        
        Ok((keyname, octave))
    }
    
    pub fn try_into_midi_keycode(&self, octave: i8) -> Result<u8, ()> {
        if octave < -1 || octave > 9 || (octave == 9 && self.into_index() > Self::G.into_index()) {
            return Err(());
        }
        
        Ok((octave + 1) as u8 * NUMBER_OF_KEYS + self.into_index() as u8)
    }

    
}

// Traits

impl Debug for KeyName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.into_str())
    }
}

impl Display for KeyName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
