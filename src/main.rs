#![no_std]
#![no_main]

use cortex_m_rt::entry;

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *const u8 = 0x23 as *const u8;

pub unsafe fn config_pin(pin: u8, output: bool) {
    if output {
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | (1 << pin));
    } else {
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) & !(1 << pin));
    }
}

pub unsafe fn write_pin(pin: u8, high: bool) {
    if high {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) | (1 << pin));
    } else {
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) & !(1 << pin));
    }
}

pub unsafe fn read_pin(pin: u8) -> bool {
    (core::ptr::read_volatile(PINB) & (1 << pin)) != 0
}

#[entry]
fn main() -> ! {
    unsafe {
        config_pin(5, true);
        loop {
            write_pin(5, true);
            for _ in 0..1_000_000 {}
            write_pin(5, false); 
            for _ in 0..1_000_000 {}
        }
    }
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
