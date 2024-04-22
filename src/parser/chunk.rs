pub const MTHD_LENGTH: u32 = 6;

#[derive(Debug)]
pub enum MidiFileFormat {
    SingleTrack,
    SimultaneousTracks,
    SequentialTracks
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Division {
    TicksPerQuarterNote(u16),
    SMPTE { // Not supported
        format: u8,
        ticks_per_frame: u8
    }
}

#[derive(Debug)]
pub enum TrackEventType {
    Midi(super::midi_event::MidiEvent),
    Meta(super::meta_event::MetaEvent)
}

#[derive(Debug)]
pub struct TrackEvent {
    pub delta_time: u32,
    pub event: TrackEventType
}

#[derive(Debug)]
pub enum Chunk {
    // Header
    MThd {
        format: MidiFileFormat,
        number_of_tracks: u16,
        division: Division
    },
    // Track
    MTrk(Vec<TrackEvent>)
}

#[derive(Debug)]
pub struct MidiFile {
    pub header: Chunk,
    pub tracks: Vec<Chunk>
}

impl std::fmt::Display for MidiFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
