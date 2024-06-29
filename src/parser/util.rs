use super::err;

// Max length of a variable length value in MIDI files: 4B
pub const MAX_VARIABLE_LENGTH_VALUE_SIZE: usize = 4;

pub fn read_bytes_at<'a>(data: &'a [u8], i: &mut usize, count: usize) -> Result<&'a [u8], err::MIDIParsingError> {
    if *i + count > data.len() {
        return Err(err::MIDIParsingError::EOFError {
            position: *i,
            tried_to_read: count,
            buffer_size: data.len(),
            message: String::new()
        });
    }

    let res = &data[*i..*i+count];
    *i += count;

    return Ok(res);
}

pub fn parse_variable_length_at(data: &[u8], i: &mut usize) -> Result<u32, err::MIDIParsingError> {
    let mut res: u32 = 0;
    
    for (byte_idx, byte) in data[*i..].iter().enumerate() {

        if byte_idx == MAX_VARIABLE_LENGTH_VALUE_SIZE {
            break;
        }

        res = (res << 7) | ((byte & 0b01111111) as u32);
        
        if byte & 0b10000000 == 0 {
            *i += byte_idx + 1;
            return Ok(res);
        }
    }
    
    Err(err::MIDIParsingError::EOFError {
        position: *i,
        tried_to_read: 4,
        buffer_size: data.len(),
        message: String::new()
    })
}
