#[derive(Debug)]
pub struct EOFError {
    pub position: usize,
    pub tried_to_read: usize,
    pub buffer_size: usize
}

impl std::fmt::Display for EOFError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EOFError {{\n\tPosition: {}B\n\tTried to read: {}B\n\tBuffer size: {}B\n\tUntil EOF: {}B\n}}", self.position, self.tried_to_read, self.buffer_size, self.buffer_size - self.position)
    }
}

#[derive(Debug)]
pub struct ParsingError {
    pub position: usize,
    pub message: String
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParsingError {{\n\tPosition: {}B\n\tMessage: \"{}\"\n}}", self.position, self.message)
    }
}

pub fn read_bytes_at<'a>(data: &'a [u8], i: &mut usize, count: usize) -> Result<&'a [u8], EOFError> {
    if *i + count > data.len() {
        return Err(EOFError {
            position: *i,
            tried_to_read: count,
            buffer_size: data.len()
        });
    }

    let res = &data[*i..*i+count];
    *i += count;

    return Ok(res);
}

pub fn parse_variable_length_at(data: &[u8], i: &mut usize) -> Result<u32, EOFError> {
    let mut res: u32 = 0;
    
    for (byte_idx, byte) in data[*i..].iter().enumerate() {

        if byte_idx == 4 {
            break;
        }

        res = (res << 7) | ((byte & 0b01111111) as u32);
        
        if byte & 0b10000000 == 0 {
            *i += byte_idx + 1;
            return Ok(res);
        }
    }
    
    Err(EOFError {
        position: *i,
        tried_to_read: 4,
        buffer_size: data.len()
    })
}