// gpio.rs
/// Structure représentant une broche GPIO.
pub struct GPIO {
    pub pin: u8, // Numéro de la broche (entre 0 et 7)
}

impl GPIO {
    /// Initialise une nouvelle broche GPIO en tant qu'entrée ou sortie.
    /// Retourne None si le numéro de broche est invalide (hors 0-7).
    pub unsafe fn new(pin: u8, output: bool) -> Option<Self> {
        if pin > 7 {
            return None;
        }
        Self::config_pin(pin, output);
        Some(GPIO { pin })
    }

    /// Configure une broche comme entrée ou sortie.
    /// Appelle les fonctions spécifiques à la cible.
    pub unsafe fn config_pin(pin: u8, output: bool) {
        #[cfg(target_arch = "avr")]
        crate::atmega::config_pin(pin, output);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::config_pin(pin, output);
    }

    /// Met la broche à l'état `HIGH`.
    pub unsafe fn set_high(&self) {
        #[cfg(target_arch = "avr")]
        crate::atmega::write_pin(self.pin, true);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::write_pin(self.pin, true);
    }

    /// Met la broche à l'état `LOW`.
    pub unsafe fn set_low(&self) {
        #[cfg(target_arch = "avr")]
        crate::atmega::write_pin(self.pin, false);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::write_pin(self.pin, false);
    }

    /// Vérifie si la broche est à l'état `HIGH`.
    pub unsafe fn is_high(&self) -> bool {
        #[cfg(target_arch = "avr")]
        return crate::atmega::read_pin(self.pin);

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::read_pin(self.pin);
    }
}
