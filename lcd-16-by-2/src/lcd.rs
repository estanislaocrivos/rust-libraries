use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

pub struct Lcd<RS, EN, D4, D5, D6, D7, DELAY>
where
    RS: OutputPin,
    EN: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    DELAY: DelayNs,
{
    rs: RS,
    en: EN,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
    delay: DELAY,
}

impl<RS, EN, D4, D5, D6, D7, DELAY> Lcd<RS, EN, D4, D5, D6, D7, DELAY>
where
    RS: OutputPin,
    EN: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    DELAY: DelayNs,
{
    pub fn new(rs: RS, en: EN, d4: D4, d5: D5, d6: D6, d7: D7, delay: DELAY) -> Self {
        Self {
            rs,
            en,
            d4,
            d5,
            d6,
            d7,
            delay,
        }
    }

    pub fn init(&mut self) {
        self.delay.delay_ms(9);
        self.rs.set_low().unwrap();
        // ...
    }
}
