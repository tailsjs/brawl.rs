pub struct ByteStream<'a> {
    buffer: &'a mut Vec<u8>,
    offset: usize,
    bit_offset: usize
}

impl ByteStream<'_> {
    pub fn new(buffer: &mut Vec<u8>) -> ByteStream {
        ByteStream {
            buffer,
            offset: 0,
            bit_offset: 0
        }
    }

    #[allow(unused_assignments, dead_code)]
    pub fn read_vint(&mut self) -> i32 {
        let mut result: i32 = 0;
        let mut shift: usize = 0;
        let mut s: u16 = 0;
        let mut a1: u16 = 0;
        let mut a2: u16 = 0;
    
        loop {
            let offset = self.add_to_offset(1);
            let mut byte: u16 = self.buffer[offset] as u16;
            
    
            if shift == 0 {
                a1 = (byte & 0x40) >> 6;
                a2 = (byte & 0x80) >> 7;
                s = (byte << 1) as u16 & !(0x181u16);
                byte = s | (a2 << 7) | a1;
            }
    
            result |= ((byte & 0x7F) as i32) << shift;
            shift += 7;
    
            if byte & 0x80 == 0 {
                break;
            }
        }
    
        (result >> 1) ^ ((result & 1) as i32)
    }

    pub fn read_int(&mut self) -> u32 {
        let offset = self.add_to_offset(1);
        let offset2 = self.add_to_offset(1);
        let offset3 = self.add_to_offset(1);
        let offset4 = self.add_to_offset(1);

        ((self.buffer[offset] as u32) << 24) | ((self.buffer[offset2] as u32) << 16) | ((self.buffer[offset3] as u32) << 8) | (self.buffer[offset4] as u32)
    }

    #[allow(dead_code)]
    pub fn read_short(&mut self) -> u32 {
        let offset = self.add_to_offset(1);
        let offset2 = self.add_to_offset(1);

        ((self.buffer[offset] as u32) << 8) | (self.buffer[offset2] as u32)
    }

    #[allow(dead_code)]
    pub fn read_boolean(&mut self) -> bool {
        let offset = self.add_to_offset(1); 
        self.buffer[offset] != 0
    }

    #[allow(dead_code)]
    pub fn read_string(&mut self) -> String {
        let len: usize = self.read_int() as usize;

        let string_bytes = &self.buffer[self.offset..self.offset + len];

        let string_bytes_len = string_bytes.len();

        self.offset += string_bytes_len;

        String::from_utf8_lossy(string_bytes).to_string()
    }

    #[allow(dead_code)]
    pub fn read_data_reference(&mut self) -> [ i32; 2 ] {
        let dataid = self.read_vint();

        if dataid == 0 {
            [ dataid, 0 ]
        } else {
            [ dataid, self.read_vint() ]
        }
    }

    #[allow(dead_code)]
    pub fn read_logic_long(&mut self) -> [ i32; 2 ] {
        [ self.read_vint(), self.read_vint() ]
    }

    #[allow(dead_code)]
    pub fn read_long_long(&mut self) -> [ u32; 2 ] {
        [ self.read_int(), self.read_int() ]
    }

    pub fn write_int(&mut self, value: u32) {
        self.bit_offset = 0;
        self.alloc_memory(4);
        self.add_to_offset(4);

        self.buffer.push((value >> 24) as u8);
        self.buffer.push((value >> 16) as u8);
        self.buffer.push((value >> 8) as u8);
        self.buffer.push(value as u8);
    }

    pub fn write_string(&mut self, value: &str) {
        self.alloc_memory(value.len());

        self.write_int(value.len() as u32);

        self.buffer.extend_from_slice(value.as_bytes());

        self.add_to_offset(value.len());
    }

    pub fn write_vint(&mut self, mut value: i32) {
        self.bit_offset = 0;

        let mut temp = (value >> 25) & 0x40;

        let mut flipped = value ^ (value >> 31);

        temp |= value & 0x3F;

        value >>= 6;
        flipped >>= 6;

        if flipped == 0 {
            self.write_byte(temp as u8);
        } else {
            self.write_byte((temp | 0x80) as u8);

            flipped >>= 7;

            let mut r = 0;

            if flipped != 0 {
                r = 0x80;
            }

            self.write_byte(((value & 0x7F) | r) as u8);

            value >>= 7;

            while flipped != 0 {
                flipped >>= 7;
                r = 0;

                if flipped != 0 {
                    r = 0x80;
                }

                self.write_byte(((value & 0x7F) | r) as u8);

                value >>= 7
            }
        }
    }

    pub fn write_boolean(&mut self, value: bool) {
        if self.bit_offset == 0 {
            self.write_byte(0);
        }

        if value {
            self.buffer[self.offset - 1] |= 1 << self.bit_offset;
        }

        self.bit_offset = (self.bit_offset + 1) & 7;
    }

    pub fn write_byte(&mut self, value: u8) {
        self.alloc_memory(1);
        self.add_to_offset(1);
        self.buffer.push(value);
    }
    

    fn add_to_offset(&mut self, amount: usize) -> usize {
        self.offset += amount;
        self.offset - 1
    }

    fn alloc_memory(&mut self, amount: usize) {
        self.buffer.reserve(amount);
    }

    pub fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }
}

