use crate::checksum::Checksum;

pub struct FramingConfig<C: Checksum> {
    pub stx: u8,
    pub etx: u8,
    pub max_payload_size: usize,
    pub checksum: C,
}
