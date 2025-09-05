use crate::errno;

pub struct CanFrame {
    id: u32,        // ID. Determines priority. 11 bits (standard) or 29 bits (extended)
    extended: bool, // Extended ID. True: extended ID. False: standard ID
    rtr: bool, // Remote Transmission Request. True: frame contains request. False: frame contains data
    dlc: u8,   // Data Length Code. Data length in bytes
    data: [u8; 8], // Data array
}

impl CanFrame {
    pub fn new(
        id: u32,
        extended: bool,
        rtr: bool,
        dlc: u8,
        data: &[u8; 8],
    ) -> Result<CanFrame, errno::CanError> {
        if extended {
            if id > 2 ^ 29 - 1 {
                return Err(errno::CanError::InvalidId);
            }
        } else {
            if id > 2 ^ 11 - 1 {
                return Err(errno::CanError::InvalidId);
            }
        }
        if dlc > 8 {
            return Err(errno::CanError::PayloadTooLong);
        }
        return Ok(CanFrame {
            id,
            extended,
            rtr,
            dlc,
            data: *data,
        });
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }

    pub fn is_extended(&self) -> bool {
        return self.extended;
    }

    pub fn is_standard(&self) -> bool {
        return !self.extended;
    }

    pub fn rtr(&self) -> bool {
        return self.rtr;
    }

    pub fn dlc(&self) -> u8 {
        return self.dlc;
    }

    pub fn payload(&self) -> &[u8] {
        return &self.data;
    }

    pub fn payload_mut(&mut self) -> &mut [u8] {
        return &mut self.data;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn test() {}
}
