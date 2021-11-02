const MAX_PACKET_LENGTH: usize = 256;

pub struct Packet {
    content: [u8; MAX_PACKET_LENGTH],
    length: usize,
}

impl Packet {
    pub fn new(bytes: [u8; MAX_PACKET_LENGTH], length: usize) -> Self {
        Packet {
            content: bytes,
            length,
        }
    }

    pub fn set_length(&mut self, length: usize) {
        self.length = length;
    }

    pub fn content(&self) -> &[u8] {
        &self.content[0..self.length]
    }

    pub fn buf(&mut self) -> &mut [u8] {
        &mut self.content
    }
}

impl Default for Packet {
    fn default() -> Self {
        Packet {
            content: [0; MAX_PACKET_LENGTH],
            length: 0,
        }
    }
}
