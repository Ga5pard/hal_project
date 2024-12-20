/// Represents the USART interface.
pub struct USART;

impl USART {
    /// Initialize the USART with a given baud rate.
    /// double_speed = true will enable U2X mode on AVR.
    pub fn init(baud_rate: u16, double_speed: bool) {
        #[cfg(target_arch = "avr")]
        crate::atmega::usart_init(baud_rate, double_speed);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::usart_init(baud_rate, double_speed);
    }

    /// Write a byte via USART.
    pub fn write(data: u8) {
        #[cfg(target_arch = "avr")]
        crate::atmega::usart_write(data);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::usart_write(data);
    }

    /// Read a byte from USART.
    pub fn read() -> u8 {
        #[cfg(target_arch = "avr")]
        return crate::atmega::usart_read();

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::usart_read();
    }
}
