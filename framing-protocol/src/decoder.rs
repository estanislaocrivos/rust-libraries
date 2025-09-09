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
    frame_counter: usize,
    payload: [u8; MAX_FRAME_SIZE],
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

    pub fn push_byte(&mut self, byte: &u8) -> Result<usize, FramingProtError> {
        match self.state {
            DecodeState::WaitSTX => {
                self.state = DecodeState::ReceivingPayload;
            }
            DecodeState::ReceivingPayload => {
                self.payload[self.payload_counter] = *byte;
                self.payload_counter += 1;
                if self.payload_counter == self.cfg.max_payload_size {
                    self.state = DecodeState::WaitETX;
                }
            }
            DecodeState::WaitETX => {
                self.state = DecodeState::WaitChecksumHi;
            }
            DecodeState::WaitChecksumHi => {
                self.checksum |= (*byte << 8) as u16;
                self.state = DecodeState::WaitChecksumLo;
            }
            DecodeState::WaitChecksumLo => {
                self.checksum |= *byte as u16;
                self.state = DecodeState::WaitSTX;
            }
        }
        self.frame_counter += 1;
        Ok(self.frame_counter)
    }

    // pub fn try_pop_payload(&mut self) -> Result<&mut [u8], FramingProtError> {
    //     if self.payload_counter == self.cfg.max_payload_size {
    //         match checksum::Checksum::calculate_checksum(&self, self.pa, payload_size) {}
    //     }
    // }

    // pub fn reset(&mut self);
}
