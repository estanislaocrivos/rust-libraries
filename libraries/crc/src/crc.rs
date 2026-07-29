struct Crc {
    crc8_polynomial: u8,
    crc8_initial_value: u8,
    crc8_final_xor_value: u8,
    reflect_input: bool,
    reflect_output: bool,
}

impl Crc {
    fn new(
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

    fn crc8_calculate(&self, data: &[u8]) -> u8 {
        let mut crc_value: u8 = self.crc8_initial_value;
        for &byte in data {
            let byte = if self.reflect_input {
                reverse_8bits(byte)
            } else {
                byte
            };
            crc_value ^= byte;
            for _ in 0..8 {
                if crc_value & 0x80 != 0 {
                    crc_value = (crc_value << 1) ^ self.crc8_polynomial;
                } else {
                    crc_value <<= 1;
                }
            }
        }
        if self.reflect_output {
            crc_value = reverse_8bits(crc_value);
        }
        crc_value ^= self.crc8_final_xor_value;
        return crc_value;
    }
}

fn reverse_8bits(mut byte: u8) -> u8 {
    byte = ((byte & 0xF0) >> 4) | ((byte & 0x0F) << 4);
    byte = ((byte & 0xCC) >> 2) | ((byte & 0x33) << 2);
    byte = ((byte & 0xAA) >> 1) | ((byte & 0x55) << 1);
    return byte;
}
