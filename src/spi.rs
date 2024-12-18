/// Represents the SPI.
pub struct SPI;

impl SPI {
    /// Initializes SPI in master mode with a given clock divider.
    pub unsafe fn init_master(clock_div: u8) {
        #[cfg(target_arch = "avr")]
        crate::atmega::spi_init_master(clock_div);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::spi_init_master(clock_div);
    }

    /// Sends and receives a byte via SPI.
    pub unsafe fn transfer(data: u8) -> u8 {
        #[cfg(target_arch = "avr")]
        return crate::atmega::spi_transfer(data);

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::spi_transfer(data);
    }
}