//! A module defining various error types.

use err_derive::Error;

#[derive(Debug, Error)]
pub enum MIDIParsingError {
    #[error(display = "End Of File reached before the parsing ended - {} (Iterator position: {}, Tried to read: {}B, Total size of the MIDI file: {}B)", message, position, tried_to_read, buffer_size)]
    EOFError {
        position: usize,
        tried_to_read: usize,
        buffer_size: usize,
        message: String
    },

    #[error(display = "MIDI event with code {} ({:b} | {:X}) not defined; Iterator position: {}", code, code, code, position)]
    UndefinedMidiEvent {
        position: usize,
        code: u8
    },

    #[error(display = "Undefined MIDI file format: {}. The file format can only be 0, 1 or 2", found)]
    UndefinedMidiFileFormat {
        found: u16
    },

    #[error(display = "The length of the header chunk was not equal the expected one. Expected: {}B, Found: {}B", expected, found)]
    WrongHeaderSize {
        expected: usize,
        found: usize
    }
}

impl MIDIParsingError {

    pub fn with_msg<T: AsRef<str>>(self, msg: T) -> Self {
        match self {
            Self::EOFError { position, tried_to_read, buffer_size, message: _ } => Self::EOFError { position, tried_to_read, buffer_size, message: String::from(msg.as_ref()) },
            _ => self
        }
    }

}

#[derive(Debug)]
pub struct ConvertionError(pub String);

impl std::fmt::Display for ConvertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Convertion error: {}", self.0)
    }
}
