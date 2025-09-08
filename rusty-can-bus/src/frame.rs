use crate::errno;

struct CanFrame {
    id: u32,          // ID. Determines priority. 11 bits (standard) or 29 bits (extended)
    extended: bool,   // Extended ID. True: extended ID. False: standard ID
    rtr: bool, // Remote Transmission Request. True: frame contains request. False: frame contains payload
    dlc: u8,   // payload Length Code. payload length in bytes
    payload: [u8; 8], // payload array
}

impl CanFrame {
    pub fn new(
        id: u32,
        extended: bool,
        rtr: bool,
        dlc: u8,
        payload: &[u8; 8],
    ) -> Result<CanFrame, errno::CanError> {
        if extended {
            if id > (1 << 29) - 1 {
                return Err(errno::CanError::InvalidId);
            }
        } else {
            if id > (1 << 11) - 1 {
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
            payload: *payload,
        });
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }

    pub fn get_is_extended(&self) -> bool {
        return self.extended;
    }

    pub fn get_is_standard(&self) -> bool {
        return !self.extended;
    }

    pub fn get_rtr(&self) -> bool {
        return self.rtr;
    }

    pub fn get_dlc(&self) -> u8 {
        return self.dlc;
    }

    pub fn get_payload(&self) -> &[u8] {
        return &self.payload;
    }

    pub fn get_payload_mut(&mut self) -> &mut [u8] {
        return &mut self.payload;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_standard_can_frame() {
        let can_frame =
            CanFrame::new(0, false, false, 1, &[0; 8]).expect("Error: CanFrame creation failed.");
        assert_eq!(can_frame.get_id(), can_frame.id);
        assert_eq!(can_frame.get_is_extended(), can_frame.extended);
        assert_eq!(can_frame.get_rtr(), can_frame.rtr);
        assert_eq!(can_frame.get_dlc(), can_frame.dlc);
        assert_eq!(can_frame.get_payload(), can_frame.payload);
    }
}
