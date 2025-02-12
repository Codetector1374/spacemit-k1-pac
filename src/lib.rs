#![no_std]
pub mod common;
pub mod gpio;
pub mod uart;

pub const GPIO0: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0xD4019000usize as _) };
pub const GPIO1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0xD4019004usize as _) };
pub const GPIO2: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0xD4019008usize as _) };
pub const GPIO3: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0xD4019100usize as _) };
pub const UART0: uart::UART = unsafe { uart::UART::from_ptr(0xD4017000usize as _) };
pub const UART1: uart::UART = unsafe { uart::UART::from_ptr(0xD4017100usize as _) };
pub const UART2: uart::UART = unsafe { uart::UART::from_ptr(0xD4017200usize as _) };
pub const UART3: uart::UART = unsafe { uart::UART::from_ptr(0xD4017300usize as _) };
pub const UART4: uart::UART = unsafe { uart::UART::from_ptr(0xD4017400usize as _) };
pub const UART5: uart::UART = unsafe { uart::UART::from_ptr(0xD4017500usize as _) };
pub const UART6: uart::UART = unsafe { uart::UART::from_ptr(0xD4017600usize as _) };
pub const UART7: uart::UART = unsafe { uart::UART::from_ptr(0xD4017700usize as _) };
pub const UART8: uart::UART = unsafe { uart::UART::from_ptr(0xD4017800usize as _) };
pub const UART9: uart::UART = unsafe { uart::UART::from_ptr(0xD4017900usize as _) };
pub const UART10: uart::UART = unsafe { uart::UART::from_ptr(0xD4017A00usize as _) };
