struct Crc {
    crc8_polynomial: u8,
    crc8_initial_value: u8,
    crc8_final_xor_value: u8,
    reflect_input: bool,
    reflect_output: bool,
}

impl Crc {
    fn new_crc8(
        crc8_polynomial: u8,
        crc8_initial_value: u8,
        crc8_final_xor_value: u8,
        reflect_input: bool,
        reflect_output: bool,
    ) -> Crc {
        return Crc {
            crc8_polynomial,
            crc8_initial_value,
            crc8_final_xor_value,
            reflect_input,
            reflect_output,
        };
    }

    fn crc8_calculate(&self, data: &[u8], length: usize) -> Result<u8, ()> {
        return Ok(0);
    }
}
