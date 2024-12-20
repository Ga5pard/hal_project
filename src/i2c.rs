/// Represents the I2C (TWI) interface.
pub struct I2C;

impl I2C {
    /// Initialize the I2C bus to the given frequency (e.g., 100 kHz).
    pub fn init(frequency: u32) {
        #[cfg(target_arch = "avr")]
        crate::atmega::i2c_init(frequency);

        #[cfg(target_arch = "arm")]
        crate::cortex_m::i2c_init(frequency);
    }

    /// Send a START condition.
    pub fn start() -> bool {
        #[cfg(target_arch = "avr")]
        return crate::atmega::i2c_start();

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::i2c_start();
    }

    /// Send a STOP condition.
    pub fn stop() {
        #[cfg(target_arch = "avr")]
        crate::atmega::i2c_stop();

        #[cfg(target_arch = "arm")]
        crate::cortex_m::i2c_stop();
    }

    /// Write a byte to the I2C bus.
    pub fn write(byte: u8) -> bool {
        #[cfg(target_arch = "avr")]
        return crate::atmega::i2c_write(byte);

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::i2c_write(byte);
    }

    /// Read a byte from the I2C bus. If ack=true, send ACK after reading, otherwise NACK.
    pub fn read(ack: bool) -> u8 {
        #[cfg(target_arch = "avr")]
        return crate::atmega::i2c_read(ack);

        #[cfg(target_arch = "arm")]
        return crate::cortex_m::i2c_read(ack);
    }
}
