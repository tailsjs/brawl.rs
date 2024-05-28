#[allow(dead_code)]
pub fn read_uint_be(vec: &Vec<u8>, start_index: usize, size: usize) -> u16 {
    let mut result = 0;
    for i in 0..size {
        result = (result << 8) | vec[start_index + i] as u16;
    }
    result
}

#[allow(dead_code)]
pub fn read_uint16_be(vec: &Vec<u8>, start_index: usize) -> u16 {
    ((vec[start_index] as u16) << 8) | (vec[start_index + 1] as u16)
}

#[allow(dead_code)]
pub fn write_uint_be(buffer: &mut Vec<u8>, value: u32, offset: usize, length: usize) {
    let mut value = value;
    
    let required_len = offset + length;
    if buffer.len() < required_len {
        buffer.resize(required_len, 0);
    }

    for i in (0..length).rev() {
        buffer[offset + i] = (value & 0xFF) as u8;
        value >>= 8;
    }
}

#[allow(dead_code)]
pub fn write_uint16_be(buffer: &mut Vec<u8>, offset: usize, value: u16) {
    buffer.resize(offset + 2, 0); 
    buffer[offset] = ((value >> 8) & 0xFF) as u8;
    buffer[offset + 1] = (value & 0xFF) as u8;
}
