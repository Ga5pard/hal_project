# HAL Project
### 4th-Year Engineering Project
### Course: Hardware Virtualization and Trust

# Project Overview
This project presents a Hardware Abstraction Layer (HAL) written in Rust for bare-metal microcontroller development. It was created during my 4th year in engineering school for the “Hardware Virtualization and Trust” course. The primary focus is the Atmega328p (as used in an Arduino Uno), but the structure is designed to be adaptable to other architectures such as ARM Cortex-M.
By separating unsafe register operations into a dedicated module and providing higher-level abstractions, the project shows how to control multiple peripherals without relying on an operating system. This approach highlights Rust’s capabilities for safety and clarity in embedded programming.

# Key Features
### Bare-Metal Rust (no_std):
No standard library usage, with a custom panic handler for minimal overhead.
### Multi-Architecture Setup:
Real implementations exist for the Atmega328p, while placeholders (stubs) illustrate how to port the code to ARM Cortex-M or other MCUs.
### Peripheral Abstractions:
GPIO for digital input and output
USART for serial communication
SPI for synchronous serial data exchange
I2C (also known as TWI) for two-wire data transfers
### Safe/Unsafe Separation:
Unsafe operations involving direct register writes are encapsulated, while user-facing APIs remain safe.
### Scalability:
Easily extended to support more MCUs, additional features (e.g., advanced SPI modes), or refined error handling in I2C and USART.

# Repository Structure
### Core Modules:
Each peripheral (GPIO, USART, SPI, I2C) has its own module for clarity, while the Atmega328p-specific code resides in a dedicated file.
### Main Entry Point:
A small main file illustrates how to configure the peripherals in a loop that toggles an LED, sends serial data, and performs SPI and I2C transactions.
### Stubs for ARM:
Minimal placeholder functions demonstrate how the abstraction can be extended to a second architecture. These can be replaced with real implementations for actual ARM hardware.

# Context and Usage
### Educational Purpose:
Created as part of a 4th-year engineering course on Hardware Virtualization and Trust, showcasing Rust’s advantages in embedded systems.
### Bare-Metal Compilation:
Configured to compile without the standard library. Suitable tools include a Rust nightly toolchain, along with AVR or ARM cross-compilers.
### Potential Extensions:
Implement advanced modes for SPI (CPOL/CPHA)
Add full USART configuration options (parity, stop bits)
Refine I2C error handling and detect device acknowledgments

Developed for academic purposes as part of the “Hardware Virtualization and Trust” course during the 4th year of my engineering studies.
