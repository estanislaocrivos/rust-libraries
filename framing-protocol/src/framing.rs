use crate::errno;

pub const STX: u8 = 0x02;
pub const ETX: u8 = 0x03;
pub const CHECKSUM_SIZE: usize = 2;
pub const OVERHEAD_SIZE: usize = 4;

pub fn build_frame(payload: &[u8], payload_size: u8, frame: &mut [u8]) {
    frame[0] = STX;
    let mut index: usize = 1;
    while index < payload_size as usize {
        frame[index] = payload[index - 1];
        index += 1;
    }
    frame[index] = ETX;
}
