use super::{midi_event, meta_event};

/// The size of bytes of the MThd (header) chunk.
pub const MTHD_LENGTH: usize = 6;

/// An enum defining the file format of the midi file.
#[derive(Debug)]
pub enum MidiFileFormat {
    SingleTrack,
    SimultaneousTracks,
    SequentialTracks
}

/// An enum relating the time passed during the playback to the length of the musical notes.
/// 
/// Only `TicksPerQuarterNote` is supported.
#[derive(Debug)]
pub enum Division {
    /// The numerical value directly correlates to the one used as delta time for events.
    TicksPerQuarterNote(u16),

    /// Not supported
    #[allow(dead_code)]
    SMPTE {
        format: u8,
        ticks_per_frame: u8
    }
}

/// An enum defining the type of an event in a track. A wrapper.
#[derive(Debug)]
pub enum TrackEventType {
    Midi(midi_event::MidiEvent),
    Meta(meta_event::MetaEvent)
}

/// A struct defining a track event. Consists of a delta time and a `TrackEventType`.
#[derive(Debug)]
pub struct TrackEvent {
    pub delta_time: u32,
    pub event: TrackEventType
}

/// An enum defining a type of a chunk, a larger piece of information in a MIDI file.
/// There are 2 types of chunks: header chunks (MThd) and track chunks (MTrk).
#[derive(Debug)]
pub enum Chunk {
    /// A header chunk.
    MThd {
        format: MidiFileFormat,
        number_of_tracks: u16,
        division: Division
    },

    /// A track chunk.
    MTrk(Vec<TrackEvent>)
}

impl Chunk {
    /// Returns a defaukt configuration for a header chunk.
    pub fn default_header() -> Self {
        Self::MThd {
            format: MidiFileFormat::SingleTrack,
            number_of_tracks: 1,
            division: Division::TicksPerQuarterNote(96)
        }
    }
}

/// A struct defining a MIDI file.
/// Consists of a header chunk and a vec of track chunks.
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
