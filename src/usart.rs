// usart.rs
/// Structure représentant l'USART.
pub struct USART;

impl USART {
    /// Initialise l'USART avec une vitesse en bauds donnée.
    /// Appelle la fonction spécifique à la cible pour configurer les registres.
    pub unsafe fn init(baud_rate: u16) {
        #[cfg(target_arch = "avr")]
        crate::atmega::usart_init(baud_rate);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::usart_init(baud_rate);
    }

    /// Envoie un octet via l'USART.
    pub unsafe fn write(data: u8) {
        #[cfg(target_arch = "avr")]
        crate::atmega::usart_write(data);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::usart_write(data);
    }

    /// Reçoit un octet via l'USART.
    pub unsafe fn read() -> u8 {
        #[cfg(target_arch = "avr")]
        return crate::atmega::usart_read();

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::usart_read();
    }
}
