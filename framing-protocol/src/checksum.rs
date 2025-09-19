use crate::errno::FramingProtError;
use crate::framing::{CHECKSUM_SIZE, OVERHEAD_SIZE};
pub struct FramingProtocol;

pub trait Checksum {
    fn calculate_checksum(
        &self,
        frame: &[u8],
        payload_size: usize,
    ) -> Result<u16, FramingProtError> {
        if frame.len() < payload_size {
            return Err(FramingProtError::InvalidSize);
        }
        let mut sum: u16 = 0;
        let mut index: usize = 0;
        while index < payload_size + OVERHEAD_SIZE - CHECKSUM_SIZE as usize {
            sum += frame[index] as u16;
            index += 1;
        }
        return Ok(sum);
    }
}
