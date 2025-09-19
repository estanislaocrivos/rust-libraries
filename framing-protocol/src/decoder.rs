use crate::checksum::Checksum;
use crate::config::FramingConfig;
use crate::errno::FramingProtError;

const MAX_FRAME_SIZE: usize = 255;

pub enum DecodeState {
    WaitSTX,
    ReceivingPayload,
    WaitETX,
    WaitChecksumHi,
    WaitChecksumLo,
}

pub struct Decoder<C: Checksum> {
    state: DecodeState,
    payload: [u8; MAX_FRAME_SIZE],
    frame_counter: usize,
    payload_counter: usize,
    checksum: u16,
    cfg: FramingConfig<C>,
}

impl<C: Checksum> Decoder<C> {
    pub fn new(cfg: FramingConfig<C>) -> Self {
        return Decoder {
            state: DecodeState::WaitSTX,
            frame_counter: 0,
            payload: [0; MAX_FRAME_SIZE],
            payload_counter: 0,
            checksum: 0,
            cfg,
        };
    }

    fn check_frame(&self) -> Result<(), FramingProtError> {
        let calc = self
            .cfg
            .checksum
            .calculate_checksum(&self.payload[..self.payload_counter], self.payload_counter)?;
        if calc == self.checksum {
            Ok(())
        } else {
            Err(FramingProtError::ChecksumMismatch)
        }
    }

    pub fn push_byte(&mut self, byte: &u8) -> Result<usize, FramingProtError> {
        match self.state {
            DecodeState::WaitSTX => {
                if *byte == self.cfg.stx {
                    self.state = DecodeState::ReceivingPayload;
                    self.payload_counter = 0;
                    self.checksum = 0;
                }
            }
            DecodeState::ReceivingPayload => {
                self.payload[self.payload_counter] = *byte;
                self.payload_counter += 1;
                if self.payload_counter == self.cfg.max_payload_size {
                    self.state = DecodeState::WaitETX;
                }
            }
            DecodeState::WaitETX => {
                if *byte == self.cfg.etx {
                    self.state = DecodeState::WaitChecksumHi;
                } else {
                    self.reset();
                    return Err(FramingProtError::InvalidFrame);
                }
            }
            DecodeState::WaitChecksumHi => {
                self.checksum = (*byte as u16) << 8;
                self.state = DecodeState::WaitChecksumLo;
            }
            DecodeState::WaitChecksumLo => {
                self.checksum |= *byte as u16;
                // Validar frame
                let res = self.check_frame();
                self.state = DecodeState::WaitSTX;
                if res.is_err() {
                    self.payload_counter = 0;
                }
                return res.map(|_| self.frame_counter);
            }
        }
        self.frame_counter += 1;
        Ok(self.frame_counter)
    }

    pub fn try_pop_payload(&mut self) -> Result<&[u8], FramingProtError> {
        if self.payload_counter > 0 {
            let slice = &self.payload[..self.payload_counter];
            self.payload_counter = 0;
            Ok(slice)
        } else {
            Err(FramingProtError::NoPayload)
        }
    }

    pub fn reset(&mut self) {
        self.state = DecodeState::WaitSTX;
        self.payload_counter = 0;
        self.checksum = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::config;

    use super::*;

    #[test]
    fn test() {}
}
