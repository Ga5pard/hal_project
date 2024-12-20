/// Represents the SPI interface.
pub struct SPI;

impl SPI {
    /// Initialize the SPI in master mode.
    pub fn init_master(clock_div: u8) {
        #[cfg(target_arch = "avr")]
        crate::atmega::spi_init_master(clock_div);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::spi_init_master(clock_div);
    }

    /// Initialize the SPI in slave mode.
    pub fn init_slave() {
        #[cfg(target_arch = "avr")]
        crate::atmega::spi_init_slave();

        #[cfg(target_arch = "arm")]
        crate::cortex_m::spi_init_slave();
    }

    /// Transfer a byte via SPI (full-duplex) and return the received byte.
    pub fn transfer(data: u8) -> u8 {
        #[cfg(target_arch = "avr")]
        return crate::atmega::spi_transfer(data);

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::spi_transfer(data);
    }

    /// Receive a byte via SPI by performing a dummy write.
    pub fn receive() -> u8 {
        #[cfg(target_arch = "avr")]
        return crate::atmega::spi_receive();

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::spi_receive();
    }
}
