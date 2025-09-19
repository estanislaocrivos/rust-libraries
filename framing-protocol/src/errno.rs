#[derive(Debug)]
pub enum FramingProtError {
    Overflow,
    InvalidChecksum,
    UnexpectedByte,
    TruncatedFrame,
    PayloadTooLong,
    InvalidSize,
    OutputFrameTooShort,
    EscapeSequence,
    ConfigError,
    PhyBusy,
    PhyTimeout,
    InvalidFrame,
    NoPayload,
    ChecksumMismatch,
}
