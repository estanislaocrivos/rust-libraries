#[derive(Debug)]
pub enum CanError {
    InvalidId,
    InvalidDlc,
    LenMismatch,
    RtrNotSupported,
    PayloadTooLong,
}
