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

impl Default for MidiFileFormat {
    fn default() -> Self {
        Self::SingleTrack
    }
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

impl Default for Division {
    fn default() -> Self {
        Self::TicksPerQuarterNote(96)
    }
}

/// An enum defining the type of an event in a track. A wrapper.
#[derive(Debug)]
pub enum TrackEventType {
    Midi(midi_event::MidiEvent),
    Meta(meta_event::MetaEvent)
}

/// A struct defining a track event.
/// 
/// Consists of a delta time and a `TrackEventType`.
#[derive(Debug)]
pub struct TrackEvent {
    pub delta_time: u32,
    pub event: TrackEventType
}

/// A struct defining the header chunk (MThd) of a MIDI file.
#[derive(Debug, Default)]
pub struct Header {
    pub format: MidiFileFormat,
    pub number_of_tracks: u16,
    pub division: Division
}

/// A tuple struct defining a track chunk (MTrk).
/// 
/// The only element of the tuple is a vec of all the events in this track in sequence.
#[derive(Debug, Default)]
pub struct Track(pub Vec<TrackEvent>);

/// A struct defining a MIDI file.
/// 
/// Consists of a header chunk and a vec of track chunks.
#[derive(Debug, Default)]
pub struct MidiFile {
    pub header: Header,
    pub tracks: Vec<Track>
}

impl std::fmt::Display for MidiFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl MidiFile {

    pub fn get_events_between(&self, t1: u8, t2: u8) {
        todo!();
    }

}
