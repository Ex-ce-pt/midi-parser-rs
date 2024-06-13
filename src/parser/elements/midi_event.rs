#[derive(Debug, PartialEq)]
pub enum MidiEvent {
    // Channel Voice Messages
    NoteOff {
        channel: u8,
        key: u8,
        velocity: u8
    },
    NoteOn {
        channel: u8,
        key: u8,
        velocity: u8
    },
    PolyphonicKeyPressure {
        channel: u8,
        key: u8,
        pressure_value: u8
    },
    ControlChange {
        channel: u8,
        controller_number: u8,
        new_value: u8
    },
    ProgramChange {
        channel: u8,
        new_program_number: u8
    },
    ChannelPressure {
        channel: u8,
        pressure_value: u8
    },
    PitchWheelChange {
        channel: u8,
        pitch_wheel_value: u16
    },

    // Channel Mode Messages
    LocalControlOff {
        channel: u8
    },
    LocalControlOn {
        channel: u8
    },
    AllNotesOff {
        channel: u8
    },
    OmniModeOff {
        channel: u8
    },
    OmniModeOn {
        channel: u8
    },
    MonoModeOn {
        channel: u8,
        number_of_channels: u8
    },
    PolyModeOn {
        channel: u8
    },

    // System Common Messages
    SystemExclusive {
        manufacturer_id: u8,
        data: Vec<u8>
    },
    SongPositionPointer {
        midi_beats_since_start: u16
    },
    SongSelect {
        song: u8
    },
    TuneRequest,

    // System Real-Time Messages
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset
}

impl std::fmt::Display for MidiEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
