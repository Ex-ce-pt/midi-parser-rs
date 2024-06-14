//! A module defining the `KeyName` type and implementing its functionality.

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
// use std::result;

use super::key_signature::KeySignature;
use super::super::err;

/// Total number of keys in 12 tone equal temperament.
pub const NUMBER_OF_KEYS: u8 = 12;

/// An enum representing the 12 keys of the 12 tone equal temperament.
/// The black keys are named as sharps (e.g. CSharp) and not as flats (e.g. DFlat).
#[derive(Clone, Copy, PartialEq)]
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
    
    /// Attempts to convert a `u8` into `KeyName` based on the order of the keys.
    ///
    /// For an opposite operation, see `KeyName::into_index`.
    pub fn try_from_index(idx: u8) -> Option<Self> {
        match idx {
            0  => Some(Self::C),
            1  => Some(Self::CSharp),
            2  => Some(Self::D),
            3  => Some(Self::DSharp),
            4  => Some(Self::E),
            5  => Some(Self::F),
            6  => Some(Self::FSharp),
            7  => Some(Self::G),
            8  => Some(Self::GSharp),
            9  => Some(Self::A),
            10 => Some(Self::ASharp),
            11 => Some(Self::B),
            _  => None
        }
    }
    
    /// Converts the `KeyName` into `u8` based on the order of the keys.
    /// 
    /// For an opposite operation, see `KeyName::try_from_index`.
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
    
    /// Attempts to convert a name of the key (e.g. G, F#, Ab) into `KeyName`.
    /// 
    /// For an opposite operation, see `KeyName::into_str`.
    pub fn try_from_name<T: AsRef<str>>(name: T) -> Option<Self> {
        match name.as_ref() {
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
    
    /// Converts the given `KeyName` into a `&'static str`.
    /// 
    /// For an opposite operation, see `KeyName::try_from_name`.
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

    /// Caution! unimplemented.
    /// 
    /// Converts the `KeyName` into `&'static str` based on the scale provided.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let scale = KeySignature { key: KeyName::G, minor: true };
    /// let name_in_scale = KeyName::ASharp.into_str_in_scale(scale);
    /// assert!(name_in_scale == "Eb");
    /// ```
    pub fn into_str_in_scale(&self, scale: &KeySignature) -> &'static str {
        todo!();
    }
    
}

// MIDI convertions

impl KeyName {

    /// Attempts to convert the MIDI representation of a key into a tuple `(KeyName, i8)`, where the first entry is the key name and the second entry is the octave.
    /// 
    /// For an opposite operation, see `KeyName::try_into_midi_keycode`.
    pub fn try_from_midi_keycode(value: u8) -> Result<(Self, i8), err::ConvertionError> {
        if value > 127 {
            return Err(err::ConvertionError(format!("Could not convert the value into KeyName: {} > 127", value)));
        }
        
        let keyname = KeyName::try_from_index(value % 12).unwrap();
        let octave = (value / NUMBER_OF_KEYS as u8) as i8 - 1;
        
        Ok((keyname, octave))
    }
    
    /// Attempts to convert the given `KeyName` into a MIDI representation using the octave the key is on.
    /// 
    /// For an opposite operation, see `KeyName::try_from_midi_keycode`.
    pub fn try_into_midi_keycode(&self, octave: i8) -> Result<u8, err::ConvertionError> {
        if octave < -1 || octave > 9 || (octave == 9 && self.into_index() > Self::G.into_index()) {
            return Err(err::ConvertionError(format!("Could not convert KeyName to the MIDI value: {}{} is out of bounds of possible values.", self, octave)));
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
