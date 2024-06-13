#[derive(Debug, PartialEq)]
pub enum MetaEvent {
    SequenceNumber {
        number: u16
    },
    TextEvent {
        text: String
    },
    CopyrightNotice {
        notice: String
    },
    TrackName {
        name: String
    },
    InstrumentName {
        name: String
    },
    Lyric {
        text: String
    },
    Marker {
        name: String
    },
    CuePoint {
        text: String
    },
    MIDIChannelPrefix {
        channel: u8
    },
    EndOfTrack,
    SetTempo {
        microseconds_per_midi_quarter_note: u64
    },
    SMPTEOffset {
        hour: u8,
        minute: u8,
        second: u8,
        frame: u8,              // don't exactly know what these 2 do, something SMPTE-related :)
        fractional_frames: u8
    },
    TimeSignature {
        numerator: u8,
        denominator: u8,
        midi_clocks_per_metronome_click: u8,
        thirty_second_notes_per_midi_quarter_note: u8
    },
    KeySignature {
        key_signature: super::key_signature::KeySignature
    },
    SequencerSpecific {
        data: Vec<u8>
    },

    // A meta-event not defined by the documentation.
    Alien {
        code: u8,
        data: Vec<u8>
    }
}

impl std::fmt::Display for MetaEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
