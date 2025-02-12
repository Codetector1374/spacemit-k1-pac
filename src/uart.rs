#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]
#![allow(non_camel_case_types)]
#[doc = "Universal Asynchronous Receiver/Transmitter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UART {
    ptr: *mut u8,
}
unsafe impl Send for UART {}
unsafe impl Sync for UART {}
impl UART {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Divisor Latch Low Byte Register"]
    #[inline(always)]
    pub const fn dll(self) -> crate::common::Reg<regs::DLL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Receive Buffer Register"]
    #[inline(always)]
    pub const fn rbr(self) -> crate::common::Reg<regs::RBR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(self) -> crate::common::Reg<regs::THR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Divisor Latch High Byte Register"]
    #[inline(always)]
    pub const fn dlh(self) -> crate::common::Reg<regs::DLH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::FCR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Interrupt Identification Register"]
    #[inline(always)]
    pub const fn iir(self) -> crate::common::Reg<regs::IIR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Line Control Register"]
    #[inline(always)]
    pub const fn lcr(self) -> crate::common::Reg<regs::LCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Modem Control Register"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Line Status Register"]
    #[inline(always)]
    pub const fn lsr(self) -> crate::common::Reg<regs::LSR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Modem Status Register"]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::MSR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Scratchpad Register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::SCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Infrared Selection Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::ISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Receive FIFO Occupancy Register"]
    #[inline(always)]
    pub const fn fifo_occupancy(
        self,
    ) -> crate::common::Reg<regs::FIFO_OCCUPANCY, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Auto-Baud Control Register"]
    #[inline(always)]
    pub const fn abr(self) -> crate::common::Reg<regs::ABR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Auto-Baud Count Register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::ACR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Full Baud Divisor Register"]
    #[inline(always)]
    pub const fn fbd(self) -> crate::common::Reg<regs::FBD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "FIFO Control Register (Write/Read)"]
    #[inline(always)]
    pub const fn fcrw(self) -> crate::common::Reg<regs::FCRW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Baud Newreg Enable Register"]
    #[inline(always)]
    pub const fn bne(self) -> crate::common::Reg<regs::BNE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ABR(pub u32);
    impl ABR {
        #[doc = "Auto-baud Enable"]
        #[inline(always)]
        pub const fn abe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-baud Enable"]
        #[inline(always)]
        pub fn set_abe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Auto-baud Lock Interrupt Enable"]
        #[inline(always)]
        pub const fn ablie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-baud Lock Interrupt Enable"]
        #[inline(always)]
        pub fn set_ablie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Auto-baud UART Programs"]
        #[inline(always)]
        pub const fn abup(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-baud UART Programs"]
        #[inline(always)]
        pub fn set_abup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Auto-baud Table/Formula Select"]
        #[inline(always)]
        pub const fn abt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-baud Table/Formula Select"]
        #[inline(always)]
        pub fn set_abt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for ABR {
        #[inline(always)]
        fn default() -> ABR {
            ABR(0)
        }
    }
    impl core::fmt::Debug for ABR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ABR")
                .field("abe", &self.abe())
                .field("ablie", &self.ablie())
                .field("abup", &self.abup())
                .field("abt", &self.abt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ABR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ABR {
                abe: bool,
                ablie: bool,
                abup: bool,
                abt: bool,
            }
            let proxy = ABR {
                abe: self.abe(),
                ablie: self.ablie(),
                abup: self.abup(),
                abt: self.abt(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACR(pub u32);
    impl ACR {
        #[doc = "Clock cycles in start-bit pulse"]
        #[inline(always)]
        pub const fn count_value(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Clock cycles in start-bit pulse"]
        #[inline(always)]
        pub fn set_count_value(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ACR {
        #[inline(always)]
        fn default() -> ACR {
            ACR(0)
        }
    }
    impl core::fmt::Debug for ACR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ACR")
                .field("count_value", &self.count_value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ACR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ACR {
                count_value: u16,
            }
            let proxy = ACR {
                count_value: self.count_value(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BNE(pub u32);
    impl BNE {
        #[doc = "Enable New Register Access"]
        #[inline(always)]
        pub const fn baud_newreg_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable New Register Access"]
        #[inline(always)]
        pub fn set_baud_newreg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Baud Rate Sync Done"]
        #[inline(always)]
        pub const fn baud_sync_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Baud Rate Sync Done"]
        #[inline(always)]
        pub fn set_baud_sync_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for BNE {
        #[inline(always)]
        fn default() -> BNE {
            BNE(0)
        }
    }
    impl core::fmt::Debug for BNE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BNE")
                .field("baud_newreg_en", &self.baud_newreg_en())
                .field("baud_sync_done", &self.baud_sync_done())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BNE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct BNE {
                baud_newreg_en: bool,
                baud_sync_done: bool,
            }
            let proxy = BNE {
                baud_newreg_en: self.baud_newreg_en(),
                baud_sync_done: self.baud_sync_done(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DLH(pub u32);
    impl DLH {
        #[doc = "Divisor Latch High"]
        #[inline(always)]
        pub const fn dlh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Divisor Latch High"]
        #[inline(always)]
        pub fn set_dlh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for DLH {
        #[inline(always)]
        fn default() -> DLH {
            DLH(0)
        }
    }
    impl core::fmt::Debug for DLH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DLH").field("dlh", &self.dlh()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DLH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DLH {
                dlh: u8,
            }
            let proxy = DLH { dlh: self.dlh() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DLL(pub u32);
    impl DLL {
        #[doc = "Divisor Latch Low"]
        #[inline(always)]
        pub const fn dll(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Divisor Latch Low"]
        #[inline(always)]
        pub fn set_dll(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for DLL {
        #[inline(always)]
        fn default() -> DLL {
            DLL(0)
        }
    }
    impl core::fmt::Debug for DLL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DLL").field("dll", &self.dll()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DLL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DLL {
                dll: u8,
            }
            let proxy = DLL { dll: self.dll() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FBD(pub u32);
    impl FBD {
        #[doc = "Divisor Latch Low"]
        #[inline(always)]
        pub const fn dll(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Divisor Latch Low"]
        #[inline(always)]
        pub fn set_dll(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Divisor Latch High"]
        #[inline(always)]
        pub const fn dlh(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Divisor Latch High"]
        #[inline(always)]
        pub fn set_dlh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for FBD {
        #[inline(always)]
        fn default() -> FBD {
            FBD(0)
        }
    }
    impl core::fmt::Debug for FBD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FBD")
                .field("dll", &self.dll())
                .field("dlh", &self.dlh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FBD {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FBD {
                dll: u8,
                dlh: u8,
            }
            let proxy = FBD {
                dll: self.dll(),
                dlh: self.dlh(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCR(pub u32);
    impl FCR {
        #[doc = "Transmit and Receive FIFO Enable"]
        #[inline(always)]
        pub const fn trfifoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit and Receive FIFO Enable"]
        #[inline(always)]
        pub fn set_trfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Reset Receive FIFO"]
        #[inline(always)]
        pub const fn resetrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Receive FIFO"]
        #[inline(always)]
        pub fn set_resetrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reset Transmit FIFO"]
        #[inline(always)]
        pub const fn resettf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Transmit FIFO"]
        #[inline(always)]
        pub fn set_resettf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter Interrupt Level"]
        #[inline(always)]
        pub const fn til(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Interrupt Level"]
        #[inline(always)]
        pub fn set_til(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Trailing Bytes"]
        #[inline(always)]
        pub const fn trail(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Trailing Bytes"]
        #[inline(always)]
        pub fn set_trail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "32-Bit Peripheral Bus"]
        #[inline(always)]
        pub const fn bus(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "32-Bit Peripheral Bus"]
        #[inline(always)]
        pub fn set_bus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Interrupt Trigger Level"]
        #[inline(always)]
        pub const fn itl(&self) -> super::vals::ITL {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::ITL::from_bits(val as u8)
        }
        #[doc = "Interrupt Trigger Level"]
        #[inline(always)]
        pub fn set_itl(&mut self, val: super::vals::ITL) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
    }
    impl Default for FCR {
        #[inline(always)]
        fn default() -> FCR {
            FCR(0)
        }
    }
    impl core::fmt::Debug for FCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCR")
                .field("trfifoe", &self.trfifoe())
                .field("resetrf", &self.resetrf())
                .field("resettf", &self.resettf())
                .field("til", &self.til())
                .field("trail", &self.trail())
                .field("bus", &self.bus())
                .field("itl", &self.itl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FCR {
                trfifoe: bool,
                resetrf: bool,
                resettf: bool,
                til: bool,
                trail: bool,
                bus: bool,
                itl: super::vals::ITL,
            }
            let proxy = FCR {
                trfifoe: self.trfifoe(),
                resetrf: self.resetrf(),
                resettf: self.resettf(),
                til: self.til(),
                trail: self.trail(),
                bus: self.bus(),
                itl: self.itl(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCRW(pub u32);
    impl FCRW {
        #[doc = "Transmit and Receive FIFO Enable"]
        #[inline(always)]
        pub const fn trfifoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit and Receive FIFO Enable"]
        #[inline(always)]
        pub fn set_trfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Reset Receive FIFO"]
        #[inline(always)]
        pub const fn resetrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Receive FIFO"]
        #[inline(always)]
        pub fn set_resetrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reset Transmit FIFO"]
        #[inline(always)]
        pub const fn resettf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Transmit FIFO"]
        #[inline(always)]
        pub fn set_resettf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter Interrupt Level"]
        #[inline(always)]
        pub const fn til(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Interrupt Level"]
        #[inline(always)]
        pub fn set_til(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Trailing Bytes"]
        #[inline(always)]
        pub const fn trail(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Trailing Bytes"]
        #[inline(always)]
        pub fn set_trail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "32-Bit Peripheral Bus"]
        #[inline(always)]
        pub const fn bus(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "32-Bit Peripheral Bus"]
        #[inline(always)]
        pub fn set_bus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Interrupt Trigger Level"]
        #[inline(always)]
        pub const fn itl(&self) -> super::vals::ITL {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::ITL::from_bits(val as u8)
        }
        #[doc = "Interrupt Trigger Level"]
        #[inline(always)]
        pub fn set_itl(&mut self, val: super::vals::ITL) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
    }
    impl Default for FCRW {
        #[inline(always)]
        fn default() -> FCRW {
            FCRW(0)
        }
    }
    impl core::fmt::Debug for FCRW {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCRW")
                .field("trfifoe", &self.trfifoe())
                .field("resetrf", &self.resetrf())
                .field("resettf", &self.resettf())
                .field("til", &self.til())
                .field("trail", &self.trail())
                .field("bus", &self.bus())
                .field("itl", &self.itl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCRW {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FCRW {
                trfifoe: bool,
                resetrf: bool,
                resettf: bool,
                til: bool,
                trail: bool,
                bus: bool,
                itl: super::vals::ITL,
            }
            let proxy = FCRW {
                trfifoe: self.trfifoe(),
                resetrf: self.resetrf(),
                resettf: self.resettf(),
                til: self.til(),
                trail: self.trail(),
                bus: self.bus(),
                itl: self.itl(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIFO_OCCUPANCY(pub u32);
    impl FIFO_OCCUPANCY {
        #[doc = "Number of bytes in receive FIFO"]
        #[inline(always)]
        pub const fn byte_count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Number of bytes in receive FIFO"]
        #[inline(always)]
        pub fn set_byte_count(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FIFO_OCCUPANCY {
        #[inline(always)]
        fn default() -> FIFO_OCCUPANCY {
            FIFO_OCCUPANCY(0)
        }
    }
    impl core::fmt::Debug for FIFO_OCCUPANCY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIFO_OCCUPANCY")
                .field("byte_count", &self.byte_count())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIFO_OCCUPANCY {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FIFO_OCCUPANCY {
                byte_count: u8,
            }
            let proxy = FIFO_OCCUPANCY {
                byte_count: self.byte_count(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IER(pub u32);
    impl IER {
        #[doc = "Receiver Data Available Interrupt Enable"]
        #[inline(always)]
        pub const fn ravie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Data Available Interrupt Enable"]
        #[inline(always)]
        pub fn set_ravie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Data Request Interrupt Enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Data Request Interrupt Enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receiver Line Status Interrupt Enable"]
        #[inline(always)]
        pub const fn rlse(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Line Status Interrupt Enable"]
        #[inline(always)]
        pub fn set_rlse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Modem Interrupt Enable"]
        #[inline(always)]
        pub const fn mie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Modem Interrupt Enable"]
        #[inline(always)]
        pub fn set_mie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receiver Time-out Interrupt Enable"]
        #[inline(always)]
        pub const fn rtoie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Time-out Interrupt Enable"]
        #[inline(always)]
        pub fn set_rtoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "NRZ Coding Enable"]
        #[inline(always)]
        pub const fn nrze(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "NRZ Coding Enable"]
        #[inline(always)]
        pub fn set_nrze(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "UART Unit Enable"]
        #[inline(always)]
        pub const fn uue(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART Unit Enable"]
        #[inline(always)]
        pub fn set_uue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA Requests Enable"]
        #[inline(always)]
        pub const fn dmae(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Requests Enable"]
        #[inline(always)]
        pub fn set_dmae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for IER {
        #[inline(always)]
        fn default() -> IER {
            IER(0)
        }
    }
    impl core::fmt::Debug for IER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IER")
                .field("ravie", &self.ravie())
                .field("tie", &self.tie())
                .field("rlse", &self.rlse())
                .field("mie", &self.mie())
                .field("rtoie", &self.rtoie())
                .field("nrze", &self.nrze())
                .field("uue", &self.uue())
                .field("dmae", &self.dmae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IER {
                ravie: bool,
                tie: bool,
                rlse: bool,
                mie: bool,
                rtoie: bool,
                nrze: bool,
                uue: bool,
                dmae: bool,
            }
            let proxy = IER {
                ravie: self.ravie(),
                tie: self.tie(),
                rlse: self.rlse(),
                mie: self.mie(),
                rtoie: self.rtoie(),
                nrze: self.nrze(),
                uue: self.uue(),
                dmae: self.dmae(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IIR(pub u32);
    impl IIR {
        #[doc = "Interrupt Pending"]
        #[inline(always)]
        pub const fn nip(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Pending"]
        #[inline(always)]
        pub fn set_nip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt Source Encoded"]
        #[inline(always)]
        pub const fn iid10(&self) -> super::vals::IID {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::IID::from_bits(val as u8)
        }
        #[doc = "Interrupt Source Encoded"]
        #[inline(always)]
        pub fn set_iid10(&mut self, val: super::vals::IID) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Time Out Detected"]
        #[inline(always)]
        pub const fn tod(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Time Out Detected"]
        #[inline(always)]
        pub fn set_tod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Auto-baud Lock"]
        #[inline(always)]
        pub const fn abl(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-baud Lock"]
        #[inline(always)]
        pub fn set_abl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DMA End of Descriptor Chain"]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA End of Descriptor Chain"]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FIFO Mode Enable Status"]
        #[inline(always)]
        pub const fn fifoes10(&self) -> super::vals::FIFOES {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::FIFOES::from_bits(val as u8)
        }
        #[doc = "FIFO Mode Enable Status"]
        #[inline(always)]
        pub fn set_fifoes10(&mut self, val: super::vals::FIFOES) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
    }
    impl Default for IIR {
        #[inline(always)]
        fn default() -> IIR {
            IIR(0)
        }
    }
    impl core::fmt::Debug for IIR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IIR")
                .field("nip", &self.nip())
                .field("iid10", &self.iid10())
                .field("tod", &self.tod())
                .field("abl", &self.abl())
                .field("eoc", &self.eoc())
                .field("fifoes10", &self.fifoes10())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IIR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IIR {
                nip: bool,
                iid10: super::vals::IID,
                tod: bool,
                abl: bool,
                eoc: bool,
                fifoes10: super::vals::FIFOES,
            }
            let proxy = IIR {
                nip: self.nip(),
                iid10: self.iid10(),
                tod: self.tod(),
                abl: self.abl(),
                eoc: self.eoc(),
                fifoes10: self.fifoes10(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ISR(pub u32);
    impl ISR {
        #[doc = "Transmitter SIR Enable"]
        #[inline(always)]
        pub const fn xmitir(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter SIR Enable"]
        #[inline(always)]
        pub fn set_xmitir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receiver SIR Enable"]
        #[inline(always)]
        pub const fn rcveir(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver SIR Enable"]
        #[inline(always)]
        pub fn set_rcveir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit Pulse Width Select"]
        #[inline(always)]
        pub const fn xmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Pulse Width Select"]
        #[inline(always)]
        pub fn set_xmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit Data Polarity"]
        #[inline(always)]
        pub const fn txpl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Data Polarity"]
        #[inline(always)]
        pub fn set_txpl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receive Data Polarity"]
        #[inline(always)]
        pub const fn rxpl(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Data Polarity"]
        #[inline(always)]
        pub fn set_rxpl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for ISR {
        #[inline(always)]
        fn default() -> ISR {
            ISR(0)
        }
    }
    impl core::fmt::Debug for ISR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ISR")
                .field("xmitir", &self.xmitir())
                .field("rcveir", &self.rcveir())
                .field("xmode", &self.xmode())
                .field("txpl", &self.txpl())
                .field("rxpl", &self.rxpl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ISR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ISR {
                xmitir: bool,
                rcveir: bool,
                xmode: bool,
                txpl: bool,
                rxpl: bool,
            }
            let proxy = ISR {
                xmitir: self.xmitir(),
                rcveir: self.rcveir(),
                xmode: self.xmode(),
                txpl: self.txpl(),
                rxpl: self.rxpl(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LCR(pub u32);
    impl LCR {
        #[doc = "Word Length Select"]
        #[inline(always)]
        pub const fn wls10(&self) -> super::vals::WLS {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::WLS::from_bits(val as u8)
        }
        #[doc = "Word Length Select"]
        #[inline(always)]
        pub fn set_wls10(&mut self, val: super::vals::WLS) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Stop Bits"]
        #[inline(always)]
        pub const fn stb(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Bits"]
        #[inline(always)]
        pub fn set_stb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Parity Enable"]
        #[inline(always)]
        pub const fn pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Enable"]
        #[inline(always)]
        pub fn set_pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Even Parity Select"]
        #[inline(always)]
        pub const fn eps(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Even Parity Select"]
        #[inline(always)]
        pub fn set_eps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Sticky Parity"]
        #[inline(always)]
        pub const fn stkyp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Sticky Parity"]
        #[inline(always)]
        pub fn set_stkyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Set Break"]
        #[inline(always)]
        pub const fn sb(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Set Break"]
        #[inline(always)]
        pub fn set_sb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Divisor Latch Access Bit"]
        #[inline(always)]
        pub const fn dlab(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Divisor Latch Access Bit"]
        #[inline(always)]
        pub fn set_dlab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for LCR {
        #[inline(always)]
        fn default() -> LCR {
            LCR(0)
        }
    }
    impl core::fmt::Debug for LCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LCR")
                .field("wls10", &self.wls10())
                .field("stb", &self.stb())
                .field("pen", &self.pen())
                .field("eps", &self.eps())
                .field("stkyp", &self.stkyp())
                .field("sb", &self.sb())
                .field("dlab", &self.dlab())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LCR {
                wls10: super::vals::WLS,
                stb: bool,
                pen: bool,
                eps: bool,
                stkyp: bool,
                sb: bool,
                dlab: bool,
            }
            let proxy = LCR {
                wls10: self.wls10(),
                stb: self.stb(),
                pen: self.pen(),
                eps: self.eps(),
                stkyp: self.stkyp(),
                sb: self.sb(),
                dlab: self.dlab(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LSR(pub u32);
    impl LSR {
        #[doc = "Data Ready"]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data Ready"]
        #[inline(always)]
        pub fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Overrun Error"]
        #[inline(always)]
        pub const fn oe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun Error"]
        #[inline(always)]
        pub fn set_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Parity Error"]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Error"]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Framing Error"]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Framing Error"]
        #[inline(always)]
        pub fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Break Interrupt"]
        #[inline(always)]
        pub const fn bi(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Break Interrupt"]
        #[inline(always)]
        pub fn set_bi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmit Data Request"]
        #[inline(always)]
        pub const fn tdrq(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Data Request"]
        #[inline(always)]
        pub fn set_tdrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmitter Empty"]
        #[inline(always)]
        pub const fn temt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Empty"]
        #[inline(always)]
        pub fn set_temt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FIFO Error Status"]
        #[inline(always)]
        pub const fn fifoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Error Status"]
        #[inline(always)]
        pub fn set_fifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for LSR {
        #[inline(always)]
        fn default() -> LSR {
            LSR(0)
        }
    }
    impl core::fmt::Debug for LSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LSR")
                .field("dr", &self.dr())
                .field("oe", &self.oe())
                .field("pe", &self.pe())
                .field("fe", &self.fe())
                .field("bi", &self.bi())
                .field("tdrq", &self.tdrq())
                .field("temt", &self.temt())
                .field("fifoe", &self.fifoe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LSR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LSR {
                dr: bool,
                oe: bool,
                pe: bool,
                fe: bool,
                bi: bool,
                tdrq: bool,
                temt: bool,
                fifoe: bool,
            }
            let proxy = LSR {
                dr: self.dr(),
                oe: self.oe(),
                pe: self.pe(),
                fe: self.fe(),
                bi: self.bi(),
                tdrq: self.tdrq(),
                temt: self.temt(),
                fifoe: self.fifoe(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCR(pub u32);
    impl MCR {
        #[doc = "Data Terminal Ready"]
        #[inline(always)]
        pub const fn dtr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data Terminal Ready"]
        #[inline(always)]
        pub fn set_dtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Request to Send"]
        #[inline(always)]
        pub const fn rts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Request to Send"]
        #[inline(always)]
        pub fn set_rts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "OUT2 Signal Control"]
        #[inline(always)]
        pub const fn out2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "OUT2 Signal Control"]
        #[inline(always)]
        pub fn set_out2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loopback Mode"]
        #[inline(always)]
        pub const fn loopback(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loopback Mode"]
        #[inline(always)]
        pub fn set_loopback(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Auto-flow Control Enable"]
        #[inline(always)]
        pub const fn afe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-flow Control Enable"]
        #[inline(always)]
        pub fn set_afe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Mask Bit for EOR Interrupt"]
        #[inline(always)]
        pub const fn eor_int_mask(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Mask Bit for EOR Interrupt"]
        #[inline(always)]
        pub fn set_eor_int_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable UART DMA RX Request on Timeout"]
        #[inline(always)]
        pub const fn ept_rxreq_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable UART DMA RX Request on Timeout"]
        #[inline(always)]
        pub fn set_ept_rxreq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for MCR {
        #[inline(always)]
        fn default() -> MCR {
            MCR(0)
        }
    }
    impl core::fmt::Debug for MCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCR")
                .field("dtr", &self.dtr())
                .field("rts", &self.rts())
                .field("out2", &self.out2())
                .field("loopback", &self.loopback())
                .field("afe", &self.afe())
                .field("eor_int_mask", &self.eor_int_mask())
                .field("ept_rxreq_en", &self.ept_rxreq_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCR {
                dtr: bool,
                rts: bool,
                out2: bool,
                loopback: bool,
                afe: bool,
                eor_int_mask: bool,
                ept_rxreq_en: bool,
            }
            let proxy = MCR {
                dtr: self.dtr(),
                rts: self.rts(),
                out2: self.out2(),
                loopback: self.loopback(),
                afe: self.afe(),
                eor_int_mask: self.eor_int_mask(),
                ept_rxreq_en: self.ept_rxreq_en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MSR(pub u32);
    impl MSR {
        #[doc = "Delta Clear to Send"]
        #[inline(always)]
        pub const fn dcts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Delta Clear to Send"]
        #[inline(always)]
        pub fn set_dcts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Delta Data Set Ready"]
        #[inline(always)]
        pub const fn ddsr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Delta Data Set Ready"]
        #[inline(always)]
        pub fn set_ddsr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Trailing Edge Ring Indicator"]
        #[inline(always)]
        pub const fn teri(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Trailing Edge Ring Indicator"]
        #[inline(always)]
        pub fn set_teri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Delta Data Carrier Detect"]
        #[inline(always)]
        pub const fn ddcd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Delta Data Carrier Detect"]
        #[inline(always)]
        pub fn set_ddcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear to Send"]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear to Send"]
        #[inline(always)]
        pub fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data Set Ready"]
        #[inline(always)]
        pub const fn dsr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Data Set Ready"]
        #[inline(always)]
        pub fn set_dsr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Ring Indicator"]
        #[inline(always)]
        pub const fn ri(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Ring Indicator"]
        #[inline(always)]
        pub fn set_ri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data Carrier Detect"]
        #[inline(always)]
        pub const fn dcd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data Carrier Detect"]
        #[inline(always)]
        pub fn set_dcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for MSR {
        #[inline(always)]
        fn default() -> MSR {
            MSR(0)
        }
    }
    impl core::fmt::Debug for MSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MSR")
                .field("dcts", &self.dcts())
                .field("ddsr", &self.ddsr())
                .field("teri", &self.teri())
                .field("ddcd", &self.ddcd())
                .field("cts", &self.cts())
                .field("dsr", &self.dsr())
                .field("ri", &self.ri())
                .field("dcd", &self.dcd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MSR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MSR {
                dcts: bool,
                ddsr: bool,
                teri: bool,
                ddcd: bool,
                cts: bool,
                dsr: bool,
                ri: bool,
                dcd: bool,
            }
            let proxy = MSR {
                dcts: self.dcts(),
                ddsr: self.ddsr(),
                teri: self.teri(),
                ddcd: self.ddcd(),
                cts: self.cts(),
                dsr: self.dsr(),
                ri: self.ri(),
                dcd: self.dcd(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RBR(pub u32);
    impl RBR {
        #[doc = "Byte 0"]
        #[inline(always)]
        pub const fn byte_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Byte 0"]
        #[inline(always)]
        pub fn set_byte_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Byte 1 (32-bit mode only)"]
        #[inline(always)]
        pub const fn byte_1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Byte 1 (32-bit mode only)"]
        #[inline(always)]
        pub fn set_byte_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Byte 2 (32-bit mode only)"]
        #[inline(always)]
        pub const fn byte_2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Byte 2 (32-bit mode only)"]
        #[inline(always)]
        pub fn set_byte_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Byte 3 (32-bit mode only)"]
        #[inline(always)]
        pub const fn byte_3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Byte 3 (32-bit mode only)"]
        #[inline(always)]
        pub fn set_byte_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for RBR {
        #[inline(always)]
        fn default() -> RBR {
            RBR(0)
        }
    }
    impl core::fmt::Debug for RBR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RBR")
                .field("byte_0", &self.byte_0())
                .field("byte_1", &self.byte_1())
                .field("byte_2", &self.byte_2())
                .field("byte_3", &self.byte_3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RBR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RBR {
                byte_0: u8,
                byte_1: u8,
                byte_2: u8,
                byte_3: u8,
            }
            let proxy = RBR {
                byte_0: self.byte_0(),
                byte_1: self.byte_1(),
                byte_2: self.byte_2(),
                byte_3: self.byte_3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR(pub u32);
    impl SCR {
        #[doc = "Scratchpad Register"]
        #[inline(always)]
        pub const fn scratchpad(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Scratchpad Register"]
        #[inline(always)]
        pub fn set_scratchpad(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SCR {
        #[inline(always)]
        fn default() -> SCR {
            SCR(0)
        }
    }
    impl core::fmt::Debug for SCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR")
                .field("scratchpad", &self.scratchpad())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR {
                scratchpad: u8,
            }
            let proxy = SCR {
                scratchpad: self.scratchpad(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct THR(pub u32);
    impl THR {
        #[doc = "Byte 0"]
        #[inline(always)]
        pub const fn byte_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Byte 0"]
        #[inline(always)]
        pub fn set_byte_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Byte 1 (32-bit mode only)"]
        #[inline(always)]
        pub const fn byte_1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Byte 1 (32-bit mode only)"]
        #[inline(always)]
        pub fn set_byte_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Byte 2 (32-bit mode only)"]
        #[inline(always)]
        pub const fn byte_2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Byte 2 (32-bit mode only)"]
        #[inline(always)]
        pub fn set_byte_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Byte 3 (32-bit mode only)"]
        #[inline(always)]
        pub const fn byte_3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Byte 3 (32-bit mode only)"]
        #[inline(always)]
        pub fn set_byte_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for THR {
        #[inline(always)]
        fn default() -> THR {
            THR(0)
        }
    }
    impl core::fmt::Debug for THR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("THR")
                .field("byte_0", &self.byte_0())
                .field("byte_1", &self.byte_1())
                .field("byte_2", &self.byte_2())
                .field("byte_3", &self.byte_3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for THR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct THR {
                byte_0: u8,
                byte_1: u8,
                byte_2: u8,
                byte_3: u8,
            }
            let proxy = THR {
                byte_0: self.byte_0(),
                byte_1: self.byte_1(),
                byte_2: self.byte_2(),
                byte_3: self.byte_3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FIFOES {
        #[doc = "Non-FIFO mode selected"]
        NON_FIFO = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "FIFO mode selected"]
        FIFO = 0x03,
    }
    impl FIFOES {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FIFOES {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FIFOES {
        #[inline(always)]
        fn from(val: u8) -> FIFOES {
            FIFOES::from_bits(val)
        }
    }
    impl From<FIFOES> for u8 {
        #[inline(always)]
        fn from(val: FIFOES) -> u8 {
            FIFOES::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum IID {
        #[doc = "Modem Status Changed"]
        MODEM_STATUS = 0x0,
        #[doc = "Transmit FIFO Requests Data"]
        TX_REQUEST = 0x01,
        #[doc = "Received Data Available"]
        RX_AVAILABLE = 0x02,
        #[doc = "Receive Error"]
        RX_ERROR = 0x03,
    }
    impl IID {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IID {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IID {
        #[inline(always)]
        fn from(val: u8) -> IID {
            IID::from_bits(val)
        }
    }
    impl From<IID> for u8 {
        #[inline(always)]
        fn from(val: IID) -> u8 {
            IID::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ITL {
        #[doc = "1 byte causes interrupt"]
        ONE_BYTE = 0x0,
        #[doc = "8 bytes cause interrupt"]
        EIGHT_BYTES = 0x01,
        #[doc = "16 bytes cause interrupt"]
        SIXTEEN_BYTES = 0x02,
        #[doc = "32 bytes cause interrupt"]
        THIRTY_TWO_BYTES = 0x03,
    }
    impl ITL {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ITL {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ITL {
        #[inline(always)]
        fn from(val: u8) -> ITL {
            ITL::from_bits(val)
        }
    }
    impl From<ITL> for u8 {
        #[inline(always)]
        fn from(val: ITL) -> u8 {
            ITL::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WLS {
        #[doc = "7-bit character"]
        SEVEN_BITS = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "8-bit character"]
        EIGHT_BITS = 0x03,
    }
    impl WLS {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WLS {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WLS {
        #[inline(always)]
        fn from(val: u8) -> WLS {
            WLS::from_bits(val)
        }
    }
    impl From<WLS> for u8 {
        #[inline(always)]
        fn from(val: WLS) -> u8 {
            WLS::to_bits(val)
        }
    }
}
