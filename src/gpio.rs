/// Represents a GPIO pin on the selected target. For AVR, we use Port B (0-7).
pub struct GPIO {
    pub pin: u8,
}

impl GPIO {
    /// Create a new GPIO pin object and configure it as input or output.
    /// Returns None if the pin number is invalid.
    pub fn new(pin: u8, output: bool) -> Option<Self> {
        if pin > 7 {
            return None;
        }
        Self::config_pin(pin, output);
        Some(GPIO { pin })
    }

    fn config_pin(pin: u8, output: bool) {
        #[cfg(target_arch = "avr")]
        crate::atmega::config_pin(pin, output);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::config_pin(pin, output);
    }

    /// Set the pin to HIGH.
    pub fn set_high(&self) {
        #[cfg(target_arch = "avr")]
        crate::atmega::write_pin(self.pin, true);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::write_pin(self.pin, true);
    }

    /// Set the pin to LOW.
    pub fn set_low(&self) {
        #[cfg(target_arch = "avr")]
        crate::atmega::write_pin(self.pin, false);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::write_pin(self.pin, false);
    }

    /// Returns true if the pin is HIGH.
    pub fn is_high(&self) -> bool {
        #[cfg(target_arch = "avr")]
        return crate::atmega::read_pin(self.pin);

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::read_pin(self.pin);
    }
}
