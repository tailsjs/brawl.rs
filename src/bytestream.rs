pub struct ByteStream<'a> {
    buffer: &'a [u8],
    offset: usize
}

impl<'a> ByteStream<'a> {
    pub fn new(buffer: &[u8]) -> ByteStream<'_> {
        ByteStream {
            buffer,
            offset: 0
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
            let mut byte: u16 = self.buffer[self.add_to_offset(1)] as u16;
    
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
        ((self.buffer[self.add_to_offset(1)] as u32) << 24) | ((self.buffer[self.add_to_offset(1)] as u32) << 16) | ((self.buffer[self.add_to_offset(1)] as u32) << 8) | (self.buffer[self.add_to_offset(1)] as u32)
    }

    #[allow(dead_code)]
    pub fn read_short(&mut self) -> u32 {
        ((self.buffer[self.add_to_offset(1)] as u32) << 8) | (self.buffer[self.add_to_offset(1)] as u32)
    }

    #[allow(dead_code)]
    pub fn read_boolean(&mut self) -> bool {
        self.buffer[self.add_to_offset(1)] != 0
    }

    #[allow(dead_code)]
    pub fn read_string(&mut self) -> String {
        let len: usize = self.read_int() as usize;

        let string_bytes = &self.buffer[self.offset + 1..self.offset + 1 + len];

        self.add_to_offset(string_bytes.len());
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

   /* pub fn write_int(&mut self, value: u32) {
        self.buffer[self.add_to_offset(4)] = (value >> 24) as u8;
        self.buffer[self.add_to_offset(3)] = (value >> 16) as u8;
        self.buffer[self.add_to_offset(2)] = (value >> 8) as u8;
        self.buffer[self.add_to_offset(1)] = value as u8;
    }
    */

    fn add_to_offset(&mut self, amount: usize) -> usize {
        self.offset += amount;

        self.offset
    }
}

