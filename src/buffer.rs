pub fn read_uint_be(buffer: &[u8], start_index: usize, size: usize) -> u16 {
    let mut result = 0;
    for i in 0..size {
        result = (result << 8) | buffer[start_index + i] as u16;
    }
    result
}

pub fn read_uint16_be(buffer: &[u8], start_index: usize) -> u16 {
    ((buffer[start_index] as u16) << 8) | (buffer[start_index + 1] as u16)
}
