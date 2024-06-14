//! A simple MIDI file parser written in Rust.
//! 
//! By Exedice
//! 
//! GitHub repo: https://github.com/Ex-ce-pt/midi-parser-rs

mod util;
mod event_parser;
pub mod elements;

use util::*;
use event_parser::*;
use elements::chunk;

fn parse_header_at(data: &[u8], i: &mut usize) -> Result<chunk::Chunk, ParsingError> {
    
    // Format
    let midi_file_format_raw = match read_bytes_at(data, i, 2) {
        Ok(f) => f,
        Err(e) => return Err(ParsingError {
            position: *i,
            message: format!("Not enough data to read MThd.format\n{}", e)
        })
    };
    let midi_file_format_idx = u16::from_be_bytes(midi_file_format_raw.try_into().unwrap());
    let midi_file_format = match midi_file_format_idx {
        0 => chunk::MidiFileFormat::SingleTrack,
        1 => chunk::MidiFileFormat::SimultaneousTracks,
        2 => chunk::MidiFileFormat::SequentialTracks,
        _ => return Err(ParsingError {
            position: *i,
            message: format!("Undefined MIDI file format: {}\nThe file format can only be 0, 1 or 2", midi_file_format_idx)
        })
    };

    // Number of tracks
    let number_of_tracks_raw = match read_bytes_at(data, i, 2) {
        Ok(n) => n,
        Err(e) => return Err(ParsingError {
            position: *i,
            message: format!("Not enough data to read MThd.number_of_tracks\n{}", e)
        })
    };
    let number_of_tracks = u16::from_be_bytes(number_of_tracks_raw.try_into().unwrap());

    // Number of tracks
    let division_raw = match read_bytes_at(data, i, 2) {
        Ok(d) => d,
        Err(e) => return Err(ParsingError {
            position: *i,
            message: format!("Not enough data to read MThd.division\n{}", e)
        })
    };
    let division_word = u16::from_be_bytes(division_raw.try_into().unwrap());
    let division: chunk::Division;
    if (division_word & (1 << 15)) == 0 {
        division = chunk::Division::TicksPerQuarterNote(division_word);
    } else {
        // division = chunk::Division::SMPTE(((division_word & 0xFF00) >> 8) as i8, (division_word & 0xFF) as u8);
        unimplemented!();
    }

    Ok(chunk::Chunk::MThd {
        format: midi_file_format,
        number_of_tracks,
        division
    })
}

fn parse_track_event_at(data: &[u8], i: &mut usize) -> Result<chunk::TrackEvent, ParsingError> {
    let delta_time = match parse_variable_length_at(data, i) {
        Ok(dt) => dt,
        Err(e) => panic!("Not enough data to read TrackEvent.delta_time\n{}", e)
    };
    
    // Try parsing a meta-event

    let meta_event_maybe = try_parse_meta_event(data, i);

    if let Some(meta_event) = meta_event_maybe {
        return Ok(chunk::TrackEvent {
            delta_time,
            event: chunk::TrackEventType::Meta(meta_event)
        })
    }

    // Try parsing a midi-event
    
    match parse_midi_event_at(data, i) {
        Ok(event) => Ok(chunk::TrackEvent {
            delta_time,
            event: chunk::TrackEventType::Midi(event)
        }),
        Err(e) => Err(e)
    }
}

fn parse_track_at(data: &[u8], i: &mut usize, length: usize) -> Result<chunk::Chunk, ParsingError> {

    let i_at_chunk_data_start = *i;
    let mut events = Vec::<chunk::TrackEvent>::new();

    while *i < i_at_chunk_data_start + length {
        let event_maybe = parse_track_event_at(data, i);
        match event_maybe {
            Ok(event) => events.push(event),
            Err(e) => return Err(e)
        }
    }

    Ok(chunk::Chunk::MTrk(events))
}

/// A function for parsing the contents of a MIDI file into a `MidiFile` struct.
/// 
/// # Examples
/// 
/// ```
/// // Open file
/// let mut file = match File::open("Test MIDIs/Megalovania.mid").unwrap();
/// // Read the file into a buffer
/// let mut buf = Vec::<u8>::new();
/// let read_res = file.read_to_end(&mut buf);
/// if let Err(e) = read_res {
///     panic!("{}", e);
/// }
/// let midi_file = match parser::parse_midi_file(&buf) {
///     Ok(f) => f,
///     Err(e) => panic!("{}", e)
/// };
/// ```
pub fn parse_midi_file(data: &[u8]) -> Result<chunk::MidiFile, ParsingError> {
    // Iterator
    let mut i: usize = 0;
    
    let mut header: chunk::Chunk = chunk::Chunk::default_header();
    let mut tracks = Vec::<chunk::Chunk>::new();

    while i < data.len() {
        // Chunk type
        let chunk_type_raw = match read_bytes_at(data, &mut i, 4) {
            Ok(t) => t,
            Err(e) => return Err(ParsingError {
                position: i,
                message: format!("Not enough data to read MTrk.chunk_type\n{}", e)
            })
        };

        // Length
        let chunk_length_raw = match read_bytes_at(data, &mut i, 4) {
            Ok(l) => l,
            Err(e) => return Err(ParsingError {
                position: i,
                message: format!("Not enough data to read MTrk.length\n{}", e)
            })
        };
        let chunk_length = u32::from_be_bytes(chunk_length_raw.try_into().unwrap()) as usize;

        match chunk_type_raw {
            b"MThd" => {
                if chunk_length != chunk::MTHD_LENGTH {
                    return Err(ParsingError {
                        position: i,
                        message: format!("The length of the header was not equal the expected one\nExpected: {}B\nFound: {}B", chunk::MTHD_LENGTH, chunk_length)
                    });
                }
                header = match parse_header_at(data, &mut i) {
                    Ok(h) => h,
                    Err(e) => return Err(e)
                };
            },
            b"MTrk" => {
                let track = match parse_track_at(data, &mut i, chunk_length) {
                    Ok(trk) => trk,
                    Err(e) => return Err(e)
                };
                tracks.push(track);
            },
            _ => {
                // Ignore unknown chunks, following the documentation.
                i += chunk_length;
            }
        }
    }

    Ok(chunk::MidiFile {
        header,
        tracks
    })
}
