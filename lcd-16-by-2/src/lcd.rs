use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{OutputPin, PinState};

pub struct Lcd<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7, DELAY>
where
    RS: OutputPin,
    EN: OutputPin,
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    DELAY: DelayNs,
{
    eight_bit_mode: bool,
    rs: RS,
    en: EN,
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
    delay: DELAY,
}

impl<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7, DELAY>
    Lcd<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7, DELAY>
where
    RS: OutputPin,
    EN: OutputPin,
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    DELAY: DelayNs,
{
    pub fn new(
        eight_bit_mode: bool,
        rs: RS,
        en: EN,
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        delay: DELAY,
    ) -> Self {
        Self {
            eight_bit_mode,
            rs,
            en,
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
            d7,
            delay,
        }
    }

    /* ------------------------------------------------------------------------------------------ */

    fn assert_command_sequence(&mut self) {
        self.rs.set_low().unwrap();
        self.en.set_high().unwrap();
        self.delay.delay_ms(10);
        self.en.set_low().unwrap();
    }

    /* ------------------------------------------------------------------------------------------ */

    fn assert_data_sequence(&mut self) {
        self.rs.set_high().unwrap();
        self.en.set_high().unwrap();
        self.delay.delay_ms(10);
        self.en.set_low().unwrap();
    }

    /* ------------------------------------------------------------------------------------------ */

    fn translate_state(state: bool) -> PinState {
        if state {
            return PinState::High;
        } else {
            return PinState::Low;
        }
    }

    /* ------------------------------------------------------------------------------------------ */

    fn write_byte(&mut self, byte: u8) {
        self.d0
            .set_state(Self::translate_state(((byte >> 0) & 0x01) != 0))
            .unwrap();
        self.d4
            .set_state(Self::translate_state(((byte >> 1) & 0x01) != 0))
            .unwrap();
        self.d4
            .set_state(Self::translate_state(((byte >> 2) & 0x01) != 0))
            .unwrap();
        self.d4
            .set_state(Self::translate_state(((byte >> 3) & 0x01) != 0))
            .unwrap();
        self.d4
            .set_state(Self::translate_state(((byte >> 4) & 0x01) != 0))
            .unwrap();
        self.d5
            .set_state(Self::translate_state(((byte >> 5) & 0x01) != 0))
            .unwrap();
        self.d6
            .set_state(Self::translate_state(((byte >> 6) & 0x01) != 0))
            .unwrap();
        self.d7
            .set_state(Self::translate_state(((byte >> 7) & 0x01) != 0))
            .unwrap();
    }

    /* ------------------------------------------------------------------------------------------ */

    fn write_nibble(&mut self, nibble: u8) {
        self.d4
            .set_state(Self::translate_state(((nibble >> 0) & 0x01) != 0))
            .unwrap();
        self.d5
            .set_state(Self::translate_state(((nibble >> 1) & 0x01) != 0))
            .unwrap();
        self.d6
            .set_state(Self::translate_state(((nibble >> 2) & 0x01) != 0))
            .unwrap();
        self.d7
            .set_state(Self::translate_state(((nibble >> 3) & 0x01) != 0))
            .unwrap();
    }

    /* ------------------------------------------------------------------------------------------ */

    fn write_command_4_bit(&mut self, command: u8) {
        self.write_nibble((command >> 4) & 0x0F);
        self.assert_command_sequence();
        self.write_nibble(command & 0x0F);
        self.assert_command_sequence();
    }

    fn write_command_8_bit(&mut self, command: u8) {
        self.write_nibble(command);
        self.assert_command_sequence();
    }

    /* ------------------------------------------------------------------------------------------ */

    fn write_char_4_bit(&mut self, char: u8) {
        self.write_nibble((char >> 4) & 0x0F);
        self.assert_data_sequence();
        self.write_nibble(char & 0x0F);
        self.assert_data_sequence();
    }

    fn write_char_8_bit(&mut self, char: u8) {
        self.write_byte(char);
        self.assert_data_sequence();
    }

    /* ------------------------------------------------------------------------------------------ */

    pub fn init(&mut self) {}
}
