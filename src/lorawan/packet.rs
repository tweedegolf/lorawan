const MAX_PAYLOAD_SIZE: usize = 222;

pub struct Packet {
    payload: [u8; MAX_PAYLOAD_SIZE],
    size: usize,
}

impl Packet {
    pub fn new(payload: [u8; MAX_PAYLOAD_SIZE], size: usize) -> Self {
        Packet {
            payload,
            size,
        }
    }

    pub fn set_length(&mut self, length: usize) {
        self.size = length;
    }

    pub fn content(&self) -> &[u8] {
        &self.payload[0..self.size]
    }

    pub fn buf(&mut self) -> &mut [u8] {
        &mut self.payload
    }
}

impl Default for Packet {
    fn default() -> Self {
        Packet {
            payload: [0; MAX_PAYLOAD_SIZE],
            size: 0,
        }
    }
}
