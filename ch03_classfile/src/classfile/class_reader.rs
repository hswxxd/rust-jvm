

pub struct ClassReader {
    data: Vec<u8>,
}

impl ClassReader {
    pub fn new(data: Vec<u8>) -> Self {
        ClassReader{
            data
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let val = self.data[0];
        self.data = self.data[1..].to_vec();
        val
    }

    pub fn read_u16(&mut self) -> u16 {
        let val = u16::from_be_bytes(  (&self.data[..2]).try_into().unwrap() );
        self.data = self.data[2..].to_vec();
        val
    }

    pub fn read_u32(&mut self) -> u32 {
        let val = u32::from_be_bytes(  (&self.data[..4]).try_into().unwrap() );
        self.data = self.data[4..].to_vec();
        val
    }

    pub fn read_u64(&mut self) -> u64 {
        let val = u64::from_be_bytes(  (&self.data[..8]).try_into().unwrap() );
        self.data = self.data[8..].to_vec();
        val
    }

    pub fn read_u16s(&mut self) -> Vec<u16> {
        let n = self.read_u16();
        let mut result = vec![];
        for i in 0..n {
            result.push(self.read_u16());
        }
        result
    }

    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let bytes = self.data[..n].to_vec();
        self.data = self.data[n..].to_vec();
        bytes
    }


}

