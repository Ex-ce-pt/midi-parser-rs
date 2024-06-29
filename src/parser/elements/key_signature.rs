//! A module defining the `KeySignature` type and implementing its functionality.

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::result;

use super::keyname::KeyName;
use super::super::err;

// TODO: add other scales

/// A struct representing the key signature (scale) of a song or a section of a song. Only major and minor scales are supported.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeySignature {
    /// The root key of the scale.
    pub key: KeyName,

    /// Whether the scale is minor.
    /// 
    /// `false` - major
    /// 
    /// `true` - minor
    pub minor: bool
}

impl KeySignature {
    
    /// Returns an array of notes that are included in the given scale. Only major and minor scales are supported.
    pub fn notes_in_scale(&self) -> [KeyName; 7] {
        todo!();
    }
    
}

// MIDI convertions

impl KeySignature {
    
    /// Attempts to convert a MIDI representation of a time signature into the `KeySignature` object.
    pub fn try_from_midi_msg(value: [u8; 2]) -> Result<Self, err::ConvertionError> {
        let sharps_flats = value[0] as i8;
        
        let minor = match value[1] {
            0 => false,
            1 => true,
            _ => return Err(err::ConvertionError(format!("The scale type is {}, only 0 (major) and 1 (minor) are permitted", value[1])))
        };
        
        // Using the circle of fifths found on the Internet
        let root = match (sharps_flats, minor) {
            (-7, false) => panic!("Although the key signature with 7 flats is allowed, it is yet undefined in this library."),
            (-7, true)  => panic!("Although the key signature with 7 flats is allowed, it is yet undefined in this library."),
            (-6, false) => KeyName::FSharp,
            (-6, true)  => KeyName::DSharp,
            (-5, false) => KeyName::CSharp,
            (-5, true)  => KeyName::ASharp,
            (-4, false) => KeyName::GSharp,
            (-4, true)  => KeyName::F,
            (-3, false) => KeyName::DSharp,
            (-3, true)  => KeyName::C,
            (-2, false) => KeyName::ASharp,
            (-2, true)  => KeyName::G,
            (-1, false) => KeyName::F,
            (-1, true)  => KeyName::D,
            (0, false)  => KeyName::C,
            (0, true)   => KeyName::A,
            (1, false)  => KeyName::G,
            (1, true)   => KeyName::E,
            (2, false)  => KeyName::D,
            (2, true)   => KeyName::B,
            (3, false)  => KeyName::A,
            (3, true)   => KeyName::FSharp,
            (4, false)  => KeyName::E,
            (4, true)   => KeyName::CSharp,
            (5, false)  => KeyName::B,
            (5, true)   => KeyName::GSharp,
            (6, false)  => KeyName::FSharp,
            (6, true)   => KeyName::DSharp,
            (7, false)  => panic!("Although the key signature with 7 sharps is allowed, it is yet undefined in this library."),
            (7, true)   => panic!("Although the key signature with 7 sharps is allowed, it is yet undefined in this library."),
            
            _ => return Err(err::ConvertionError(format!("No scale found with {} {}s", sharps_flats.abs(), if sharps_flats > 0 { "sharp" } else { "flat" }))),
        };
        
        Ok(Self {
            key: root,
            minor
        })
    }
    
}

// Traits

impl Display for KeySignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.key, if self.minor { "m" } else { "" })
    }
}

impl TryFrom<&str> for KeySignature {
    type Error = err::ConvertionError;
    
    fn try_from(value: &str) -> result::Result<Self, Self::Error> {
        let trimmed_name = value.trim();
        if trimmed_name.len() == 0 {
            return Err(err::ConvertionError(String::from("The provided string is either empty or consists only of whitespaces.")));
        }
        
        let last_char = trimmed_name.bytes().nth(trimmed_name.len() - 1).unwrap() as char;
        let mut minor: bool = false;
        let mut isolated_keyname: &str = trimmed_name;
        if last_char == 'm' {
            minor = true;
            isolated_keyname = &trimmed_name[0..(trimmed_name.len() - 1)];
        }
        if last_char == 'M' {
            isolated_keyname = &trimmed_name[0..(trimmed_name.len() - 1)];
        }
        
        let keyname: Option<KeyName> = KeyName::try_from_name(isolated_keyname);
        if keyname.is_none() {
            return Err(err::ConvertionError(format!("Could not determine the name of the key: {}", isolated_keyname)));
        }
        
        Ok(KeySignature {
            key: keyname.unwrap(),
            minor
        })
    }
}
