#[derive(Debug)]
pub enum FrameError {
    Overflow,
    InvalidChecksum,
    UnexpectedByte,
    TruncatedFrame,
    PayloadTooLong,
    EscapeSequence,
    ConfigError,
    PhyBusy,
    PhyTimeout,
}
