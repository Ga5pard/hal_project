/// Represents a GPIO pin.
pub struct GPIO {
    pub pin: u8, // Pin number (0 to 7)
}

impl GPIO {
    /// Initializes a new GPIO pin as input or output.
    /// Returns None if the pin number is invalid (outside 0-7).
    pub unsafe fn new(pin: u8, output: bool) -> Option<Self> {
        if pin > 7 {
            return None;
        }
        Self::config_pin(pin, output);
        Some(GPIO { pin })
    }

    /// Configures a pin as input or output.
    /// Calls target-specific functions.
    pub unsafe fn config_pin(pin: u8, output: bool) {
        #[cfg(target_arch = "avr")]
        crate::atmega::config_pin(pin, output);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::config_pin(pin, output);
    }

    /// Sets the pin to `HIGH` state.
    pub unsafe fn set_high(&self) {
        #[cfg(target_arch = "avr")]
        crate::atmega::write_pin(self.pin, true);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::write_pin(self.pin, true);
    }

    /// Sets the pin to `LOW` state.
    pub unsafe fn set_low(&self) {
        #[cfg(target_arch = "avr")]
        crate::atmega::write_pin(self.pin, false);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::write_pin(self.pin, false);
    }

    /// Checks if the pin is in `HIGH` state.
    pub unsafe fn is_high(&self) -> bool {
        #[cfg(target_arch = "avr")]
        return crate::atmega::read_pin(self.pin);

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::read_pin(self.pin);
    }
}