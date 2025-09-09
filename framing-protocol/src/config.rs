use crate::checksum::Checksum;

pub struct FramingConfig<C: Checksum> {
    pub start: u8,
    pub stop: u8,
    pub max_payload_size: usize,
    pub checksum: C,
}
