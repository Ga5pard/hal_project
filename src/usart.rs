/// Represents the USART.
pub struct USART;

impl USART {
    /// Initializes USART with a given baud rate.
    /// Calls target-specific register setup.
    pub unsafe fn init(baud_rate: u16) {
        #[cfg(target_arch = "avr")]
        crate::atmega::usart_init(baud_rate);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::usart_init(baud_rate);
    }

    /// Sends a byte via USART.
    pub unsafe fn write(data: u8) {
        #[cfg(target_arch = "avr")]
        crate::atmega::usart_write(data);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::usart_write(data);
    }

    /// Receives a byte via USART.
    pub unsafe fn read() -> u8 {
        #[cfg(target_arch = "avr")]
        return crate::atmega::usart_read();

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::usart_read();
    }
}