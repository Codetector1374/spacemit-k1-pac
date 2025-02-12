#![no_std]
pub mod common;
pub mod gpio;

pub const GPIO0: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0xD4019000usize as _) };
pub const GPIO1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0xD4019004usize as _) };
pub const GPIO2: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0xD4019008usize as _) };
pub const GPIO3: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0xD4019100usize as _) };
