use super::util::*;
use super::midi_event;
use super::meta_event;

fn try_construct_channel_mode_message(channel: u8, controller_number: u8, new_value: u8) -> Option<midi_event::MidiEvent> {

    if controller_number == 126 {
        return Some(midi_event::MidiEvent::MonoModeOn { channel, number_of_channels: new_value });
    }

    match (controller_number, new_value) {
        (122, 0) => Some(midi_event::MidiEvent::LocalControlOff { channel }),
        (122, 127) => Some(midi_event::MidiEvent::LocalControlOn { channel }),
        (123, 0) => Some(midi_event::MidiEvent::AllNotesOff { channel }),
        (124, 0) => Some(midi_event::MidiEvent::OmniModeOff { channel }),
        (125, 0) => Some(midi_event::MidiEvent::OmniModeOn { channel }),
        (127, 0) => Some(midi_event::MidiEvent::PolyModeOn { channel }),

        _ => None
    }
}

fn parse_midi_event_channel_voice_message_at(data: &[u8], i: &mut usize, event_code: u8) -> Result<midi_event::MidiEvent, ParsingError> {
    let channel = event_code & 0b1111;
    
    match event_code & 0b11110000 {

        0b10000000 => {
            let key = match read_bytes_at(data, i, 1) {
                Ok(k) => k[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[NoteOff].key\n{}", e)
                })
            };
            let velocity = match read_bytes_at(data, i, 1) {
                Ok(v) => v[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[NoteOff].velocity\n{}", e)
                })
            };

            Ok(midi_event::MidiEvent::NoteOff { channel, key, velocity })
        },

        0b10010000 => {
            let key = match read_bytes_at(data, i, 1) {
                Ok(k) => k[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[NoteOn].key\n{}", e)
                })
            };
            let velocity = match read_bytes_at(data, i, 1) {
                Ok(v) => v[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[NoteOn].velocity\n{}", e)
                })
            };

            Ok(midi_event::MidiEvent::NoteOn { channel, key, velocity })
        },

        0b10100000 => {
            let key = match read_bytes_at(data, i, 1) {
                Ok(k) => k[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[PolyphonicKeyPressure].key\n{}", e)
                })
            };
            let pressure_value = match read_bytes_at(data, i, 1) {
                Ok(p) => p[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[PolyphonicKeyPressure].pressure_value\n{}", e)
                })
            };

            Ok(midi_event::MidiEvent::PolyphonicKeyPressure { channel, key, pressure_value })
        },

        0b10110000 => {
            let controller_number = match read_bytes_at(data, i, 1) {
                Ok(c) => c[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[ControlChange].controller_number\n{}", e)
                })
            };
            let new_value = match read_bytes_at(data, i, 1) {
                Ok(v) => v[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[ControlChange].new_value\n{}", e)
                })
            };

            let channel_mode_msg_maybe = try_construct_channel_mode_message(channel, controller_number, new_value);

            if let Some(msg) = channel_mode_msg_maybe {
                return Ok(msg);
            }

            Ok(midi_event::MidiEvent::ControlChange { channel, controller_number, new_value })
        },

        0b11000000 => {
            let new_program_number = match read_bytes_at(data, i, 1) {
                Ok(p) => p[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[ProgramChange].new_program_number\n{}", e)
                })
            };

            Ok(midi_event::MidiEvent::ProgramChange { channel, new_program_number })
        },

        0b11010000 => {
            let pressure_value = match read_bytes_at(data, i, 1) {
                Ok(p) => p[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[ChannelPressure].pressure_value\n{}", e)
                })
            };

            Ok(midi_event::MidiEvent::ChannelPressure { channel, pressure_value })
        },

        0b11100000 => {
            let pitch_wheel_value = match read_bytes_at(data, i, 2) {
                Ok(w) => ((w[1] as u16) << 7) | (w[0] as u16),
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[PitchWheelChange].key\n{}", e)
                })
            };

            Ok(midi_event::MidiEvent::PitchWheelChange { channel, pitch_wheel_value })
        },

        code => Err(ParsingError {
            position: *i,
            message: format!("Midi event code not defined - {} ({:b} | {:X})", code, code, code)
        })
    }
}

fn parse_midi_event_system_common_or_real_time_message_at(data: &[u8], i: &mut usize, event_code: u8) -> Result<midi_event::MidiEvent, ParsingError> {
    match event_code {

        // System Common Messages

        0b11110000 => {
            let manufacturer_id = match read_bytes_at(data, i, 1) {
                Ok(id) => id[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[SystemExclusive].manufacrurer_id\n{}", e)
                })
            };
            let mut event_data = Vec::<u8>::new();
            loop {
                let byte = match read_bytes_at(data, i, 1) {
                    Ok(id) => id[0],
                    Err(e) => return Err(ParsingError {
                        position: *i,
                        message: format!("Not enough data to read MidiEvent.data[{}]\n{}", event_data.len(), e)
                    })
                };

                // "End of Exclusive" message
                if byte == 0b11110111 {
                    break;
                }

                event_data.push(byte);
            };

            Ok(midi_event::MidiEvent::SystemExclusive { manufacturer_id, data: event_data })
        },

        0b11110001 => unreachable!(), // Undefined event code

        0b11110010 => {
            let midi_beats_since_start = match read_bytes_at(data, i, 2) {
                Ok(b) => ((b[1] as u16) << 7) | (b[0] as u16),
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[SongPositionPointer].midi_beats_since_start\n{}", e)
                })
            };

            Ok(midi_event::MidiEvent::SongPositionPointer { midi_beats_since_start })
        },

        0b11110011 => {
            let song = match read_bytes_at(data, i, 1) {
                Ok(s) => s[0],
                Err(e) => return Err(ParsingError {
                    position: *i,
                    message: format!("Not enough data to read MidiEvent[SongSelect].song\n{}", e)
                })
            };

            Ok(midi_event::MidiEvent::SongSelect { song })
        }

        0b11110100 => unreachable!(), // Undefined event code

        0b11110101 => unreachable!(), // Undefined event code

        0b11110110 => {
            Ok(midi_event::MidiEvent::TuneRequest)
        },

        0b11110111 => unreachable!(), // End of Exclusive, should be handled in the SystemExclusive branch

        // System Real-Time Messages

        0b11111000 => {
            Ok(midi_event::MidiEvent::TimingClock)
        },
        
        0b11111001 => unreachable!(), // Undefined event code

        0b11111010 => {
            Ok(midi_event::MidiEvent::Start)
        },

        0b11111011 => {
            Ok(midi_event::MidiEvent::Continue)
        },

        0b11111100 => {
            Ok(midi_event::MidiEvent::Stop)
        },

        0b11111101 => unreachable!(), // Undefined event code

        0b11111110 => {
            Ok(midi_event::MidiEvent::ActiveSensing)
        },

        0b11111111 => {
            Ok(midi_event::MidiEvent::Reset)
        }

        code => return Err(ParsingError {
            position: *i,
            message: format!("Midi event code not defined - {} ({:b} | {:X})", code, code, code)
        })

    }
}

pub fn parse_midi_event_at(data: &[u8], i: &mut usize) -> Result<midi_event::MidiEvent, ParsingError> {
    let event_code = match read_bytes_at(data, i, 1) {
        Ok(c) => c[0],
        Err(e) => panic!("Not enough data to read MidiEvent.event_code\n{}", e)
    };

    if event_code & 0b11110000 != 0b11110000 { // Channel Voice Messages are only up to 0b1111nnnn
        return parse_midi_event_channel_voice_message_at(data, i, event_code);
    }
    
    parse_midi_event_system_common_or_real_time_message_at(data, i, event_code)
}

pub fn try_parse_meta_event(data: &[u8], i: &mut usize) -> Option<meta_event::MetaEvent> {
    // A copy of the iterator, the actual iterator is only moved forward when the return value has been determined.
    let mut i_copy = *i;
    
    // Return None if ANY errors occur during parsing.

    let event_code = match read_bytes_at(data, &mut i_copy, 2) {
        Ok(header) => {
            if header[0] != 0xFF {
                return None;
            }
            header[1]
        },
        Err(_) => return None
    };
    let data_length = match parse_variable_length_at(data, &mut i_copy) {
        Ok(v) => v,
        Err(_) => return None
    };
    let data = match read_bytes_at(data, &mut i_copy, data_length as usize) {
        Ok(d) => d,
        Err(_) => return None
    };

    match event_code {

        0x00 => {
            if data_length != 2 {
                return None;
            }

            let number = ((data[0] as u16) << 8) | (data[1] as u16);

            *i = i_copy;
            Some(meta_event::MetaEvent::SequenceNumber { number })
        },

        0x01 => {
            let text = String::from_utf8_lossy(data);

            *i = i_copy;
            Some(meta_event::MetaEvent::TextEvent { text: text.into() })
        },

        0x02 => {
            let notice = String::from_utf8_lossy(data);

            *i = i_copy;
            Some(meta_event::MetaEvent::CopyrightNotice { notice: notice.into() })
        },

        0x03 => {
            let name = String::from_utf8_lossy(data);

            *i = i_copy;
            Some(meta_event::MetaEvent::TrackName { name: name.into() })
        },

        0x04 => {
            let name = String::from_utf8_lossy(data);

            *i = i_copy;
            Some(meta_event::MetaEvent::InstrumentName { name: name.into() })
        },

        0x05 => {
            let text = String::from_utf8_lossy(data);

            *i = i_copy;
            Some(meta_event::MetaEvent::Lyric { text: text.into() })
        },

        0x06 => {
            let name = String::from_utf8_lossy(data);

            *i = i_copy;
            Some(meta_event::MetaEvent::Marker { name: name.into() })
        },

        0x07 => {
            let text = String::from_utf8_lossy(data);

            *i = i_copy;
            Some(meta_event::MetaEvent::CuePoint { text: text.into() })
        },

        0x20 => {
            if data_length != 1 {
                return None;
            }

            let channel = data[0];

            *i = i_copy;
            Some(meta_event::MetaEvent::MIDIChannelPrefix { channel })
        },

        0x2F => {
            if data_length != 0 {
                return None;
            }

            *i = i_copy;
            Some(meta_event::MetaEvent::EndOfTrack)
        },

        0x51 => {
            if data_length != 3 {
                return None;
            }

            let microseconds_per_midi_quarter_note = ((data[0] as u64) << 16) | ((data[1] as u64) << 8) | (data[2] as u64);

            *i = i_copy;
            Some(meta_event::MetaEvent::SetTempo { microseconds_per_midi_quarter_note })
        },

        0x54 => {
            if data_length != 5 {
                return None;
            }

            let hour = data[0];
            let minute = data[1];
            let second = data[2];
            let frame = data[3];
            let fractional_frames = data[4];

            *i = i_copy;
            Some(meta_event::MetaEvent::SMPTEOffset { hour, minute, second, frame, fractional_frames })
        },

        0x58 => {
            if data_length != 4 {
                return None;
            }

            let numerator = data[0];
            let denominator = data[1];
            let midi_clocks_per_metronome_click = data[2];
            let thirty_second_notes_per_midi_quarter_note = data[3];

            *i = i_copy;
            Some(meta_event::MetaEvent::TimeSignature { numerator, denominator, midi_clocks_per_metronome_click, thirty_second_notes_per_midi_quarter_note })
        },

        0x59 => {
            if data_length != 2 {
                return None;
            }

            let sf = data[0];
            let mi = data[1] == 1;

            *i = i_copy;
            Some(meta_event::MetaEvent::KeySignature { sf, mi })
        },

        0x7F => {
            
            *i = i_copy;
            Some(meta_event::MetaEvent::SequencerSpecific { data: data.to_vec() })
        },

        code => {
            *i = i_copy;
            Some(meta_event::MetaEvent::Alien { code, data: data.to_vec() })
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_note_off() {
        // 1000nnnn	0kkkkkkk 0vvvvvvv

        let mut i: usize = 0;

        let data1: [u8; 3] = [0b10000000, 0b00110000, 0b01111111];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::NoteOff { channel, key, velocity } = result1 {
            if !(channel == 0 && key == 48 && velocity == 127) {
                panic!("1. fail: wrong parameters");
            }
        } else {
            panic!("1. fail: wrong enum variant");
        }
        i = 0;

        let data2: [u8; 3] = [0b10000011, 0b00110100, 0b01111011];
        let result2 = match parse_midi_event_at(&data2, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::NoteOff { channel, key, velocity } = result2 {
            if !(channel == 3 && key == 52 && velocity == 123) {
                panic!("2. fail: wrong parameters");
            }
        } else {
            panic!("2. fail: wrong enum variant");
        }
        i = 0;

        let data3: [u8; 3] = [0b10001011, 0b00110110, 0b00000011];
        let result3 = match parse_midi_event_at(&data3, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::NoteOff { channel, key, velocity } = result3 {
            if !(channel == 11 && key == 54 && velocity == 3) {
                panic!("3. fail: wrong parameters");
            }
        } else {
            panic!("3. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_note_on() {
        // 1001nnnn	0kkkkkkk 0vvvvvvv

        let mut i: usize = 0;

        let data1: [u8; 3] = [0b10010000, 0b00110000, 0b01111111];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::NoteOn { channel, key, velocity } = result1 {
            if !(channel == 0 && key == 48 && velocity == 127) {
                panic!("1. fail: wrong parameters");
            }
        } else {
            panic!("1. fail: wrong enum variant");
        }
        i = 0;

        let data2: [u8; 3] = [0b10010011, 0b00110100, 0b01111011];
        let result2 = match parse_midi_event_at(&data2, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::NoteOn { channel, key, velocity } = result2 {
            if !(channel == 3 && key == 52 && velocity == 123) {
                panic!("2. fail: wrong parameters");
            }
        } else {
            panic!("2. fail: wrong enum variant");
        }
        i = 0;

        let data3: [u8; 3] = [0b10011011, 0b00110110, 0b00000011];
        let result3 = match parse_midi_event_at(&data3, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::NoteOn { channel, key, velocity } = result3 {
            if !(channel == 11 && key == 54 && velocity == 3) {
                panic!("3. fail: wrong parameters");
            }
        } else {
            panic!("3. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_polyphonic_pressure() {
        // 1010nnnn	0kkkkkkk 0vvvvvvv

        let mut i: usize = 0;

        let data1: [u8; 3] = [0b10100000, 0b00110000, 0b01111111];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::PolyphonicKeyPressure { channel, key, pressure_value } = result1 {
            if !(channel == 0 && key == 48 && pressure_value == 127) {
                panic!("1. fail: wrong parameters");
            }
        } else {
            panic!("1. fail: wrong enum variant");
        }
        i = 0;

        let data2: [u8; 3] = [0b10100011, 0b00110100, 0b01111011];
        let result2 = match parse_midi_event_at(&data2, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::PolyphonicKeyPressure { channel, key, pressure_value } = result2 {
            if !(channel == 3 && key == 52 && pressure_value == 123) {
                panic!("2. fail: wrong parameters");
            }
        } else {
            panic!("2. fail: wrong enum variant");
        }
        i = 0;

        let data3: [u8; 3] = [0b10101011, 0b00110110, 0b00000011];
        let result3 = match parse_midi_event_at(&data3, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::PolyphonicKeyPressure { channel, key, pressure_value } = result3 {
            if !(channel == 11 && key == 54 && pressure_value == 3) {
                panic!("3. fail: wrong parameters");
            }
        } else {
            panic!("3. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_control_change() {
        // 1011nnnn	0ccccccc 0vvvvvvv

        let mut i: usize = 0;

        let data1: [u8; 3] = [0b10110000, 0b00110000, 0b01111111];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::ControlChange { channel, controller_number, new_value } = result1 {
            if !(channel == 0 && controller_number == 48 && new_value == 127) {
                panic!("1. fail: wrong parameters");
            }
        } else {
            panic!("1. fail: wrong enum variant");
        }
        i = 0;

        let data2: [u8; 3] = [0b10110011, 0b00110100, 0b01111011];
        let result2 = match parse_midi_event_at(&data2, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::ControlChange { channel, controller_number, new_value } = result2 {
            if !(channel == 3 && controller_number == 52 && new_value == 123) {
                panic!("2. fail: wrong parameters");
            }
        } else {
            panic!("2. fail: wrong enum variant");
        }
        i = 0;

        let data3: [u8; 3] = [0b10111011, 0b00110110, 0b00000011];
        let result3 = match parse_midi_event_at(&data3, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::ControlChange { channel, controller_number, new_value } = result3 {
            if !(channel == 11 && controller_number == 54 && new_value == 3) {
                panic!("3. fail: wrong parameters");
            }
        } else {
            panic!("3. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_program_change() {
        // 1100nnnn	0ppppppp

        let mut i: usize = 0;

        let data1: [u8; 2] = [0b11000000, 0b00110000];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::ProgramChange { channel, new_program_number } = result1 {
            if !(channel == 0 && new_program_number == 48) {
                panic!("1. fail: wrong parameters");
            }
        } else {
            panic!("1. fail: wrong enum variant");
        }
        i = 0;

        let data2: [u8; 2] = [0b11000011, 0b00110100];
        let result2 = match parse_midi_event_at(&data2, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::ProgramChange { channel, new_program_number } = result2 {
            if !(channel == 3 && new_program_number == 52) {
                panic!("2. fail: wrong parameters");
            }
        } else {
            panic!("2. fail: wrong enum variant");
        }
        i = 0;

        let data3: [u8; 2] = [0b11001011, 0b00110110];
        let result3 = match parse_midi_event_at(&data3, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::ProgramChange { channel, new_program_number } = result3 {
            if !(channel == 11 && new_program_number == 54) {
                panic!("3. fail: wrong parameters");
            }
        } else {
            panic!("3. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_channel_pressure() {
        // 1100nnnn	0vvvvvvv

        let mut i: usize = 0;

        let data1: [u8; 2] = [0b11010000, 0b00110000];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::ChannelPressure { channel, pressure_value } = result1 {
            if !(channel == 0 && pressure_value == 48) {
                panic!("1. fail: wrong parameters");
            }
        } else {
            panic!("1. fail: wrong enum variant");
        }
        i = 0;

        let data2: [u8; 2] = [0b11010011, 0b00110100];
        let result2 = match parse_midi_event_at(&data2, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::ChannelPressure { channel, pressure_value } = result2 {
            if !(channel == 3 && pressure_value == 52) {
                panic!("2. fail: wrong parameters");
            }
        } else {
            panic!("2. fail: wrong enum variant");
        }
        i = 0;

        let data3: [u8; 2] = [0b11011011, 0b00110110];
        let result3 = match parse_midi_event_at(&data3, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::ChannelPressure { channel, pressure_value } = result3 {
            if !(channel == 11 && pressure_value == 54) {
                panic!("3. fail: wrong parameters");
            }
        } else {
            panic!("3. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_pitch_wheel_change() {
        // 1110nnnn	0lllllll 0mmmmmmm

        let mut i: usize = 0;

        let data1: [u8; 3] = [0b11100000, 0b00110000, 0b01111111];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::PitchWheelChange { channel, pitch_wheel_value } = result1 {
            if !(channel == 0 && pitch_wheel_value == 16304) {
                panic!("1. fail: wrong parameters");
            }
        } else {
            panic!("1. fail: wrong enum variant");
        }
        i = 0;

        let data2: [u8; 3] = [0b11100011, 0b00110100, 0b01111011];
        let result2 = match parse_midi_event_at(&data2, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::PitchWheelChange { channel, pitch_wheel_value } = result2 {
            if !(channel == 3 && pitch_wheel_value == 15796) {
                panic!("2. fail: wrong parameters");
            }
        } else {
            panic!("2. fail: wrong enum variant");
        }
        i = 0;

        let data3: [u8; 3] = [0b11101011, 0b00110110, 0b00000011];
        let result3 = match parse_midi_event_at(&data3, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::PitchWheelChange { channel, pitch_wheel_value } = result3 {
            if !(channel == 11 && pitch_wheel_value == 438) {
                panic!("3. fail: wrong parameters");
            }
        } else {
            panic!("3. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_song_position_pointer() {
        // 11110010	0lllllll 0mmmmmmm

        let mut i: usize = 0;

        let data1: [u8; 3] = [0b11110010, 0b00110000, 0b01111111];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::SongPositionPointer { midi_beats_since_start } = result1 {
            if !(midi_beats_since_start == 16304) {
                panic!("1. fail: wrong parameters");
            }
        } else {
            panic!("1. fail: wrong enum variant");
        }
        i = 0;

        let data2: [u8; 3] = [0b11110010, 0b00110100, 0b01111011];
        let result2 = match parse_midi_event_at(&data2, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::SongPositionPointer { midi_beats_since_start } = result2 {
            if !(midi_beats_since_start == 15796) {
                panic!("2. fail: wrong parameters");
            }
        } else {
            panic!("2. fail: wrong enum variant");
        }
        i = 0;

        let data3: [u8; 3] = [0b11110010, 0b00110110, 0b00000011];
        let result3 = match parse_midi_event_at(&data3, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::SongPositionPointer { midi_beats_since_start } = result3 {
            if !(midi_beats_since_start == 438) {
                panic!("3. fail: wrong parameters");
            }
        } else {
            panic!("3. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_song_select() {
        // 11110011	0sssssss

        let mut i: usize = 0;

        let data1: [u8; 2] = [0b11110011, 0b00110000];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::SongSelect { song } = result1 {
            if !(song == 48) {
                panic!("1. fail: wrong parameters");
            }
        } else {
            panic!("1. fail: wrong enum variant");
        }
        i = 0;

        let data2: [u8; 2] = [0b11110011, 0b00110100];
        let result2 = match parse_midi_event_at(&data2, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::SongSelect { song } = result2 {
            if !(song == 52) {
                panic!("2. fail: wrong parameters");
            }
        } else {
            panic!("2. fail: wrong enum variant");
        }
        i = 0;

        let data3: [u8; 2] = [0b11110011, 0b00110110];
        let result3 = match parse_midi_event_at(&data3, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::SongSelect { song } = result3 {
            if !(song == 54) {
                panic!("3. fail: wrong parameters");
            }
        } else {
            panic!("3. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_tune_request() {
        // 11110110

        let mut i: usize = 0;

        let data1: [u8; 1] = [0b11110110];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::TuneRequest = result1 {
            // Empty block
        } else {
            panic!("1. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_timing_clock() {
        // 11111000

        let mut i: usize = 0;

        let data1: [u8; 1] = [0b11111000];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::TimingClock = result1 {
            // Empty block
        } else {
            panic!("1. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_start() {
        // 11111010

        let mut i: usize = 0;

        let data1: [u8; 1] = [0b11111010];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::Start = result1 {
            // Empty block
        } else {
            panic!("1. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_stop() {
        // 11111100

        let mut i: usize = 0;

        let data1: [u8; 1] = [0b11111100];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::Stop = result1 {
            // Empty block
        } else {
            panic!("1. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_active_sensing() {
        // 11111110

        let mut i: usize = 0;

        let data1: [u8; 1] = [0b11111110];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::ActiveSensing = result1 {
            // Empty block
        } else {
            panic!("1. fail: wrong enum variant");
        }
    }

    #[test]
    fn test_reset() {
        // 11111111

        let mut i: usize = 0;

        let data1: [u8; 1] = [0b11111111];
        let result1 = match parse_midi_event_at(&data1, &mut i) {
            Ok(r) => r,
            Err(e) => panic!("{e}")
        };
        if let midi_event::MidiEvent::Reset = result1 {
            // Empty block
        } else {
            panic!("1. fail: wrong enum variant");
        }
    }

    // Meta-Events

    #[test]
    fn test_sequence_number() {
        // FF 00 02 ss ss
        let mut i = 0;

        let data1 = [0xFF, 0x00, 0x02, 0x00, 0x16];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::SequenceNumber { number }) = res1 {
            if number != 22 {
                panic!("test1 returned wrong data ({number})");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x00, 0x02, 0x16, 0x00];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::SequenceNumber { number }) = res2 {
            if number != 5632 {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x00, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_text_event() {
        // FF 01 len text
        let mut i = 0;

        let data1 = [0xFF, 0x01, 0x02, 'H' as u8, 'i' as u8];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::TextEvent { text }) = res1 {
            if text != "Hi" {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x01, 0x03, 'L' as u8, 'O' as u8, 'L' as u8];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::TextEvent { text }) = res2 {
            if text != "LOL" {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x01, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_copyright_notice() {
        // FF 02 len text
        let mut i = 0;

        let data1 = [0xFF, 0x02, 0x02, 'H' as u8, 'i' as u8];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::CopyrightNotice { notice }) = res1 {
            if notice != "Hi" {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x02, 0x03, 'L' as u8, 'O' as u8, 'L' as u8];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::CopyrightNotice { notice }) = res2 {
            if notice != "LOL" {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x02, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_track_name() {
        // FF 03 len text
        let mut i = 0;

        let data1 = [0xFF, 0x03, 0x02, 'H' as u8, 'i' as u8];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::TrackName { name }) = res1 {
            if name != "Hi" {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x03, 0x03, 'L' as u8, 'O' as u8, 'L' as u8];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::TrackName { name }) = res2 {
            if name != "LOL" {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x03, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_instrument_name() {
        // FF 04 len text
        let mut i = 0;

        let data1 = [0xFF, 0x04, 0x02, 'H' as u8, 'i' as u8];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::InstrumentName { name }) = res1 {
            if name != "Hi" {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x04, 0x03, 'L' as u8, 'O' as u8, 'L' as u8];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::InstrumentName { name }) = res2 {
            if name != "LOL" {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x04, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_lyric() {
        // FF 05 len text
        let mut i = 0;

        let data1 = [0xFF, 0x05, 0x02, 'H' as u8, 'i' as u8];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::Lyric { text }) = res1 {
            if text != "Hi" {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x05, 0x03, 'L' as u8, 'O' as u8, 'L' as u8];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::Lyric { text }) = res2 {
            if text != "LOL" {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x05, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_marker() {
        // FF 06 len text
        let mut i = 0;

        let data1 = [0xFF, 0x06, 0x02, 'H' as u8, 'i' as u8];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::Marker { name }) = res1 {
            if name != "Hi" {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x06, 0x03, 'L' as u8, 'O' as u8, 'L' as u8];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::Marker { name }) = res2 {
            if name != "LOL" {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x06, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_cue_point() {
        // FF 07 len text
        let mut i = 0;

        let data1 = [0xFF, 0x07, 0x02, 'H' as u8, 'i' as u8];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::CuePoint { text }) = res1 {
            if text != "Hi" {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x07, 0x03, 'L' as u8, 'O' as u8, 'L' as u8];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::CuePoint { text }) = res2 {
            if text != "LOL" {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x07, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_midi_channel_prefix() {
        // FF 20 01 cc
        let mut i = 0;

        let data1 = [0xFF, 0x20, 0x01, 0x03];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::MIDIChannelPrefix { channel }) = res1 {
            if channel != 3 {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x20, 0x01, 0x05];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::MIDIChannelPrefix { channel }) = res2 {
            if channel != 5 {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x20, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_end_of_track() {
        // FF 2F 00
        let mut i = 0;

        let data1 = [0xFF, 0x2F, 0x00];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::EndOfTrack) = res1 {
            // Empty block
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x2F, 0xFF];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if res2.is_some() || i != 0 {
            panic!("test2 returned Some");
        }
    }

    #[test]
    fn test_set_tempo() {
        // FF 51 03 tt tt tt
        let mut i = 0;

        let data1 = [0xFF, 0x51, 0x03, 0x00, 0x00, 0x01];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::SetTempo { microseconds_per_midi_quarter_note }) = res1 {
            if microseconds_per_midi_quarter_note != 1 {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x51, 0x03, 0x01, 0x02, 0x03];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::SetTempo { microseconds_per_midi_quarter_note }) = res2 {
            if microseconds_per_midi_quarter_note != 66051 {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x01, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_smpte_offset() {
        // FF 54 05 hr mn se fr ff
        let mut i = 0;

        let data1 = [0xFF, 0x54, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::SMPTEOffset { hour, minute, second, frame, fractional_frames }) = res1 {
            if hour != 1 || minute != 2 || second != 3 || frame != 4 || fractional_frames != 5 {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x54, 0x05, 0x02, 0x03, 0x04, 0x05, 0x06];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::SMPTEOffset { hour, minute, second, frame, fractional_frames }) = res2 {
            if hour != 2 || minute != 3 || second != 4 || frame != 5 || fractional_frames != 6 {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x54, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_time_signature() {
        // FF 58 04 nn dd cc bb
        let mut i = 0;

        let data1 = [0xFF, 0x58, 0x04, 0x02, 0x02, 0x09, 0x09];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::TimeSignature { numerator, denominator, midi_clocks_per_metronome_click, thirty_second_notes_per_midi_quarter_note }) = res1 {
            if numerator != 2 || denominator != 2 || midi_clocks_per_metronome_click != 9 || thirty_second_notes_per_midi_quarter_note != 9 {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x58, 0x04, 0x02, 0x03, 0x04, 0x05];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::TimeSignature { numerator, denominator, midi_clocks_per_metronome_click, thirty_second_notes_per_midi_quarter_note }) = res2 {
            if numerator != 2 || denominator != 3 || midi_clocks_per_metronome_click != 4 || thirty_second_notes_per_midi_quarter_note != 5 {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x58, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_key_signature() {
        // FF 59 02 sf mi
        let mut i = 0;

        let data1 = [0xFF, 0x59, 0x02, 0x02, 0x01];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::KeySignature { sf, mi }) = res1 {
            if sf != 0x02 || !mi {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x59, 0x02, 0x05, 0x00];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::KeySignature { sf, mi }) = res2 {
            if sf != 0x05 || mi {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x59, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

    #[test]
    fn test_sequencer_specific() {
        // FF 7F len data
        let mut i = 0;

        let data1 = [0xFF, 0x7F, 0x02, 0x01, 0x02];
        let res1 = try_parse_meta_event(&data1, &mut i);
        if let Some(meta_event::MetaEvent::SequencerSpecific { data }) = res1 {
            if data != vec![0x01, 0x02] {
                panic!("test1 returned wrong data");
            }
        } else {
            panic!("test1 failed");
        }
        i = 0;

        let data2 = [0xFF, 0x7F, 0x03, 0x02, 0x03, 0x04];
        let res2 = try_parse_meta_event(&data2, &mut i);
        if let Some(meta_event::MetaEvent::SequencerSpecific { data }) = res2 {
            if data != vec![0x02, 0x03, 0x04] {
                panic!("test2 returned wrong data");
            }
        } else {
            panic!("test2 failed");
        }
        i = 0;

        let data3 = [0xFF, 0x7F, 0xFF, 0x00, 0x00];
        let res3 = try_parse_meta_event(&data3, &mut i);
        if res3.is_some() || i != 0 {
            panic!("test3 returned Some");
        }
    }

}
