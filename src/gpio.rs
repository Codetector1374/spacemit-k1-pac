#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]
#![allow(non_camel_case_types)]
#[doc = "General Purpose Input/Output"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO {
    ptr: *mut u8,
}
unsafe impl Send for GPIO {}
unsafe impl Sync for GPIO {}
impl GPIO {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port Level Register"]
    #[inline(always)]
    pub const fn plr(self) -> crate::common::Reg<regs::PLR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Port Direction Register"]
    #[inline(always)]
    pub const fn pdr(self) -> crate::common::Reg<regs::PDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Port Set Register"]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::PSR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Port Clear Register"]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::PCR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Rising-Edge Detect Enable Register"]
    #[inline(always)]
    pub const fn rer(self) -> crate::common::Reg<regs::RER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Falling-Edge Detect Enable Register"]
    #[inline(always)]
    pub const fn fer(self) -> crate::common::Reg<regs::FER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Edge Detect Status Register"]
    #[inline(always)]
    pub const fn edr(self) -> crate::common::Reg<regs::EDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Set Direction Register"]
    #[inline(always)]
    pub const fn sdr(self) -> crate::common::Reg<regs::SDR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Clear Direction Register"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::CDR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Set Rising Edge Detect Enable Register"]
    #[inline(always)]
    pub const fn srer(self) -> crate::common::Reg<regs::SRER, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Clear Rising Edge Detect Enable Register"]
    #[inline(always)]
    pub const fn crer(self) -> crate::common::Reg<regs::CRER, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Set Falling Edge Detect Enable Register"]
    #[inline(always)]
    pub const fn sfer(self) -> crate::common::Reg<regs::SFER, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Clear Falling Edge Detect Enable Register"]
    #[inline(always)]
    pub const fn cfer(self) -> crate::common::Reg<regs::CFER, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CDR(pub u32);
    impl CDR {
        #[doc = "Clear GPIO port direction (0=unaffected, 1=set input)"]
        #[inline(always)]
        pub const fn cd(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear GPIO port direction (0=unaffected, 1=set input)"]
        #[inline(always)]
        pub fn set_cd(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CDR {
        #[inline(always)]
        fn default() -> CDR {
            CDR(0)
        }
    }
    impl core::fmt::Debug for CDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CDR")
                .field(
                    "cd",
                    &[
                        self.cd(0usize),
                        self.cd(1usize),
                        self.cd(2usize),
                        self.cd(3usize),
                        self.cd(4usize),
                        self.cd(5usize),
                        self.cd(6usize),
                        self.cd(7usize),
                        self.cd(8usize),
                        self.cd(9usize),
                        self.cd(10usize),
                        self.cd(11usize),
                        self.cd(12usize),
                        self.cd(13usize),
                        self.cd(14usize),
                        self.cd(15usize),
                        self.cd(16usize),
                        self.cd(17usize),
                        self.cd(18usize),
                        self.cd(19usize),
                        self.cd(20usize),
                        self.cd(21usize),
                        self.cd(22usize),
                        self.cd(23usize),
                        self.cd(24usize),
                        self.cd(25usize),
                        self.cd(26usize),
                        self.cd(27usize),
                        self.cd(28usize),
                        self.cd(29usize),
                        self.cd(30usize),
                        self.cd(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CDR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CDR {
                cd: [bool; 32usize],
            }
            let proxy = CDR {
                cd: [
                    self.cd(0usize),
                    self.cd(1usize),
                    self.cd(2usize),
                    self.cd(3usize),
                    self.cd(4usize),
                    self.cd(5usize),
                    self.cd(6usize),
                    self.cd(7usize),
                    self.cd(8usize),
                    self.cd(9usize),
                    self.cd(10usize),
                    self.cd(11usize),
                    self.cd(12usize),
                    self.cd(13usize),
                    self.cd(14usize),
                    self.cd(15usize),
                    self.cd(16usize),
                    self.cd(17usize),
                    self.cd(18usize),
                    self.cd(19usize),
                    self.cd(20usize),
                    self.cd(21usize),
                    self.cd(22usize),
                    self.cd(23usize),
                    self.cd(24usize),
                    self.cd(25usize),
                    self.cd(26usize),
                    self.cd(27usize),
                    self.cd(28usize),
                    self.cd(29usize),
                    self.cd(30usize),
                    self.cd(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CFER(pub u32);
    impl CFER {
        #[doc = "Clear GPIO Falling Edge detect enable"]
        #[inline(always)]
        pub const fn cfer(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear GPIO Falling Edge detect enable"]
        #[inline(always)]
        pub fn set_cfer(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CFER {
        #[inline(always)]
        fn default() -> CFER {
            CFER(0)
        }
    }
    impl core::fmt::Debug for CFER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CFER")
                .field(
                    "cfer",
                    &[
                        self.cfer(0usize),
                        self.cfer(1usize),
                        self.cfer(2usize),
                        self.cfer(3usize),
                        self.cfer(4usize),
                        self.cfer(5usize),
                        self.cfer(6usize),
                        self.cfer(7usize),
                        self.cfer(8usize),
                        self.cfer(9usize),
                        self.cfer(10usize),
                        self.cfer(11usize),
                        self.cfer(12usize),
                        self.cfer(13usize),
                        self.cfer(14usize),
                        self.cfer(15usize),
                        self.cfer(16usize),
                        self.cfer(17usize),
                        self.cfer(18usize),
                        self.cfer(19usize),
                        self.cfer(20usize),
                        self.cfer(21usize),
                        self.cfer(22usize),
                        self.cfer(23usize),
                        self.cfer(24usize),
                        self.cfer(25usize),
                        self.cfer(26usize),
                        self.cfer(27usize),
                        self.cfer(28usize),
                        self.cfer(29usize),
                        self.cfer(30usize),
                        self.cfer(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CFER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CFER {
                cfer: [bool; 32usize],
            }
            let proxy = CFER {
                cfer: [
                    self.cfer(0usize),
                    self.cfer(1usize),
                    self.cfer(2usize),
                    self.cfer(3usize),
                    self.cfer(4usize),
                    self.cfer(5usize),
                    self.cfer(6usize),
                    self.cfer(7usize),
                    self.cfer(8usize),
                    self.cfer(9usize),
                    self.cfer(10usize),
                    self.cfer(11usize),
                    self.cfer(12usize),
                    self.cfer(13usize),
                    self.cfer(14usize),
                    self.cfer(15usize),
                    self.cfer(16usize),
                    self.cfer(17usize),
                    self.cfer(18usize),
                    self.cfer(19usize),
                    self.cfer(20usize),
                    self.cfer(21usize),
                    self.cfer(22usize),
                    self.cfer(23usize),
                    self.cfer(24usize),
                    self.cfer(25usize),
                    self.cfer(26usize),
                    self.cfer(27usize),
                    self.cfer(28usize),
                    self.cfer(29usize),
                    self.cfer(30usize),
                    self.cfer(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CRER(pub u32);
    impl CRER {
        #[doc = "Clear GPIO Rising Edge detect enable"]
        #[inline(always)]
        pub const fn crer(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear GPIO Rising Edge detect enable"]
        #[inline(always)]
        pub fn set_crer(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CRER {
        #[inline(always)]
        fn default() -> CRER {
            CRER(0)
        }
    }
    impl core::fmt::Debug for CRER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CRER")
                .field(
                    "crer",
                    &[
                        self.crer(0usize),
                        self.crer(1usize),
                        self.crer(2usize),
                        self.crer(3usize),
                        self.crer(4usize),
                        self.crer(5usize),
                        self.crer(6usize),
                        self.crer(7usize),
                        self.crer(8usize),
                        self.crer(9usize),
                        self.crer(10usize),
                        self.crer(11usize),
                        self.crer(12usize),
                        self.crer(13usize),
                        self.crer(14usize),
                        self.crer(15usize),
                        self.crer(16usize),
                        self.crer(17usize),
                        self.crer(18usize),
                        self.crer(19usize),
                        self.crer(20usize),
                        self.crer(21usize),
                        self.crer(22usize),
                        self.crer(23usize),
                        self.crer(24usize),
                        self.crer(25usize),
                        self.crer(26usize),
                        self.crer(27usize),
                        self.crer(28usize),
                        self.crer(29usize),
                        self.crer(30usize),
                        self.crer(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CRER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CRER {
                crer: [bool; 32usize],
            }
            let proxy = CRER {
                crer: [
                    self.crer(0usize),
                    self.crer(1usize),
                    self.crer(2usize),
                    self.crer(3usize),
                    self.crer(4usize),
                    self.crer(5usize),
                    self.crer(6usize),
                    self.crer(7usize),
                    self.crer(8usize),
                    self.crer(9usize),
                    self.crer(10usize),
                    self.crer(11usize),
                    self.crer(12usize),
                    self.crer(13usize),
                    self.crer(14usize),
                    self.crer(15usize),
                    self.crer(16usize),
                    self.crer(17usize),
                    self.crer(18usize),
                    self.crer(19usize),
                    self.crer(20usize),
                    self.crer(21usize),
                    self.crer(22usize),
                    self.crer(23usize),
                    self.crer(24usize),
                    self.crer(25usize),
                    self.crer(26usize),
                    self.crer(27usize),
                    self.crer(28usize),
                    self.crer(29usize),
                    self.crer(30usize),
                    self.crer(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EDR(pub u32);
    impl EDR {
        #[doc = "GPIO edge detect status"]
        #[inline(always)]
        pub const fn ed(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO edge detect status"]
        #[inline(always)]
        pub fn set_ed(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for EDR {
        #[inline(always)]
        fn default() -> EDR {
            EDR(0)
        }
    }
    impl core::fmt::Debug for EDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EDR")
                .field(
                    "ed",
                    &[
                        self.ed(0usize),
                        self.ed(1usize),
                        self.ed(2usize),
                        self.ed(3usize),
                        self.ed(4usize),
                        self.ed(5usize),
                        self.ed(6usize),
                        self.ed(7usize),
                        self.ed(8usize),
                        self.ed(9usize),
                        self.ed(10usize),
                        self.ed(11usize),
                        self.ed(12usize),
                        self.ed(13usize),
                        self.ed(14usize),
                        self.ed(15usize),
                        self.ed(16usize),
                        self.ed(17usize),
                        self.ed(18usize),
                        self.ed(19usize),
                        self.ed(20usize),
                        self.ed(21usize),
                        self.ed(22usize),
                        self.ed(23usize),
                        self.ed(24usize),
                        self.ed(25usize),
                        self.ed(26usize),
                        self.ed(27usize),
                        self.ed(28usize),
                        self.ed(29usize),
                        self.ed(30usize),
                        self.ed(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EDR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EDR {
                ed: [bool; 32usize],
            }
            let proxy = EDR {
                ed: [
                    self.ed(0usize),
                    self.ed(1usize),
                    self.ed(2usize),
                    self.ed(3usize),
                    self.ed(4usize),
                    self.ed(5usize),
                    self.ed(6usize),
                    self.ed(7usize),
                    self.ed(8usize),
                    self.ed(9usize),
                    self.ed(10usize),
                    self.ed(11usize),
                    self.ed(12usize),
                    self.ed(13usize),
                    self.ed(14usize),
                    self.ed(15usize),
                    self.ed(16usize),
                    self.ed(17usize),
                    self.ed(18usize),
                    self.ed(19usize),
                    self.ed(20usize),
                    self.ed(21usize),
                    self.ed(22usize),
                    self.ed(23usize),
                    self.ed(24usize),
                    self.ed(25usize),
                    self.ed(26usize),
                    self.ed(27usize),
                    self.ed(28usize),
                    self.ed(29usize),
                    self.ed(30usize),
                    self.ed(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FER(pub u32);
    impl FER {
        #[doc = "GPIO port falling-edge detect enable"]
        #[inline(always)]
        pub const fn fe(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO port falling-edge detect enable"]
        #[inline(always)]
        pub fn set_fe(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for FER {
        #[inline(always)]
        fn default() -> FER {
            FER(0)
        }
    }
    impl core::fmt::Debug for FER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FER")
                .field(
                    "fe",
                    &[
                        self.fe(0usize),
                        self.fe(1usize),
                        self.fe(2usize),
                        self.fe(3usize),
                        self.fe(4usize),
                        self.fe(5usize),
                        self.fe(6usize),
                        self.fe(7usize),
                        self.fe(8usize),
                        self.fe(9usize),
                        self.fe(10usize),
                        self.fe(11usize),
                        self.fe(12usize),
                        self.fe(13usize),
                        self.fe(14usize),
                        self.fe(15usize),
                        self.fe(16usize),
                        self.fe(17usize),
                        self.fe(18usize),
                        self.fe(19usize),
                        self.fe(20usize),
                        self.fe(21usize),
                        self.fe(22usize),
                        self.fe(23usize),
                        self.fe(24usize),
                        self.fe(25usize),
                        self.fe(26usize),
                        self.fe(27usize),
                        self.fe(28usize),
                        self.fe(29usize),
                        self.fe(30usize),
                        self.fe(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FER {
                fe: [bool; 32usize],
            }
            let proxy = FER {
                fe: [
                    self.fe(0usize),
                    self.fe(1usize),
                    self.fe(2usize),
                    self.fe(3usize),
                    self.fe(4usize),
                    self.fe(5usize),
                    self.fe(6usize),
                    self.fe(7usize),
                    self.fe(8usize),
                    self.fe(9usize),
                    self.fe(10usize),
                    self.fe(11usize),
                    self.fe(12usize),
                    self.fe(13usize),
                    self.fe(14usize),
                    self.fe(15usize),
                    self.fe(16usize),
                    self.fe(17usize),
                    self.fe(18usize),
                    self.fe(19usize),
                    self.fe(20usize),
                    self.fe(21usize),
                    self.fe(22usize),
                    self.fe(23usize),
                    self.fe(24usize),
                    self.fe(25usize),
                    self.fe(26usize),
                    self.fe(27usize),
                    self.fe(28usize),
                    self.fe(29usize),
                    self.fe(30usize),
                    self.fe(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PCR(pub u32);
    impl PCR {
        #[doc = "GPIO output port clear (0=unaffected, 1=clear low if output)"]
        #[inline(always)]
        pub const fn pc(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO output port clear (0=unaffected, 1=clear low if output)"]
        #[inline(always)]
        pub fn set_pc(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PCR {
        #[inline(always)]
        fn default() -> PCR {
            PCR(0)
        }
    }
    impl core::fmt::Debug for PCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PCR")
                .field(
                    "pc",
                    &[
                        self.pc(0usize),
                        self.pc(1usize),
                        self.pc(2usize),
                        self.pc(3usize),
                        self.pc(4usize),
                        self.pc(5usize),
                        self.pc(6usize),
                        self.pc(7usize),
                        self.pc(8usize),
                        self.pc(9usize),
                        self.pc(10usize),
                        self.pc(11usize),
                        self.pc(12usize),
                        self.pc(13usize),
                        self.pc(14usize),
                        self.pc(15usize),
                        self.pc(16usize),
                        self.pc(17usize),
                        self.pc(18usize),
                        self.pc(19usize),
                        self.pc(20usize),
                        self.pc(21usize),
                        self.pc(22usize),
                        self.pc(23usize),
                        self.pc(24usize),
                        self.pc(25usize),
                        self.pc(26usize),
                        self.pc(27usize),
                        self.pc(28usize),
                        self.pc(29usize),
                        self.pc(30usize),
                        self.pc(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PCR {
                pc: [bool; 32usize],
            }
            let proxy = PCR {
                pc: [
                    self.pc(0usize),
                    self.pc(1usize),
                    self.pc(2usize),
                    self.pc(3usize),
                    self.pc(4usize),
                    self.pc(5usize),
                    self.pc(6usize),
                    self.pc(7usize),
                    self.pc(8usize),
                    self.pc(9usize),
                    self.pc(10usize),
                    self.pc(11usize),
                    self.pc(12usize),
                    self.pc(13usize),
                    self.pc(14usize),
                    self.pc(15usize),
                    self.pc(16usize),
                    self.pc(17usize),
                    self.pc(18usize),
                    self.pc(19usize),
                    self.pc(20usize),
                    self.pc(21usize),
                    self.pc(22usize),
                    self.pc(23usize),
                    self.pc(24usize),
                    self.pc(25usize),
                    self.pc(26usize),
                    self.pc(27usize),
                    self.pc(28usize),
                    self.pc(29usize),
                    self.pc(30usize),
                    self.pc(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PDR(pub u32);
    impl PDR {
        #[doc = "GPIO port direction (0=input, 1=output)"]
        #[inline(always)]
        pub const fn pd(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO port direction (0=input, 1=output)"]
        #[inline(always)]
        pub fn set_pd(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PDR {
        #[inline(always)]
        fn default() -> PDR {
            PDR(0)
        }
    }
    impl core::fmt::Debug for PDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PDR")
                .field(
                    "pd",
                    &[
                        self.pd(0usize),
                        self.pd(1usize),
                        self.pd(2usize),
                        self.pd(3usize),
                        self.pd(4usize),
                        self.pd(5usize),
                        self.pd(6usize),
                        self.pd(7usize),
                        self.pd(8usize),
                        self.pd(9usize),
                        self.pd(10usize),
                        self.pd(11usize),
                        self.pd(12usize),
                        self.pd(13usize),
                        self.pd(14usize),
                        self.pd(15usize),
                        self.pd(16usize),
                        self.pd(17usize),
                        self.pd(18usize),
                        self.pd(19usize),
                        self.pd(20usize),
                        self.pd(21usize),
                        self.pd(22usize),
                        self.pd(23usize),
                        self.pd(24usize),
                        self.pd(25usize),
                        self.pd(26usize),
                        self.pd(27usize),
                        self.pd(28usize),
                        self.pd(29usize),
                        self.pd(30usize),
                        self.pd(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PDR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PDR {
                pd: [bool; 32usize],
            }
            let proxy = PDR {
                pd: [
                    self.pd(0usize),
                    self.pd(1usize),
                    self.pd(2usize),
                    self.pd(3usize),
                    self.pd(4usize),
                    self.pd(5usize),
                    self.pd(6usize),
                    self.pd(7usize),
                    self.pd(8usize),
                    self.pd(9usize),
                    self.pd(10usize),
                    self.pd(11usize),
                    self.pd(12usize),
                    self.pd(13usize),
                    self.pd(14usize),
                    self.pd(15usize),
                    self.pd(16usize),
                    self.pd(17usize),
                    self.pd(18usize),
                    self.pd(19usize),
                    self.pd(20usize),
                    self.pd(21usize),
                    self.pd(22usize),
                    self.pd(23usize),
                    self.pd(24usize),
                    self.pd(25usize),
                    self.pd(26usize),
                    self.pd(27usize),
                    self.pd(28usize),
                    self.pd(29usize),
                    self.pd(30usize),
                    self.pd(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLR(pub u32);
    impl PLR {
        #[inline(always)]
        pub const fn pl(&self, n: usize) -> super::vals::PLR_PL {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::PLR_PL::from_bits(val as u8)
        }
        #[inline(always)]
        pub fn set_pl(&mut self, n: usize, val: super::vals::PLR_PL) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for PLR {
        #[inline(always)]
        fn default() -> PLR {
            PLR(0)
        }
    }
    impl core::fmt::Debug for PLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PLR")
                .field(
                    "pl",
                    &[
                        self.pl(0usize),
                        self.pl(1usize),
                        self.pl(2usize),
                        self.pl(3usize),
                        self.pl(4usize),
                        self.pl(5usize),
                        self.pl(6usize),
                        self.pl(7usize),
                        self.pl(8usize),
                        self.pl(9usize),
                        self.pl(10usize),
                        self.pl(11usize),
                        self.pl(12usize),
                        self.pl(13usize),
                        self.pl(14usize),
                        self.pl(15usize),
                        self.pl(16usize),
                        self.pl(17usize),
                        self.pl(18usize),
                        self.pl(19usize),
                        self.pl(20usize),
                        self.pl(21usize),
                        self.pl(22usize),
                        self.pl(23usize),
                        self.pl(24usize),
                        self.pl(25usize),
                        self.pl(26usize),
                        self.pl(27usize),
                        self.pl(28usize),
                        self.pl(29usize),
                        self.pl(30usize),
                        self.pl(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PLR {
                pl: [super::vals::PLR_PL; 32usize],
            }
            let proxy = PLR {
                pl: [
                    self.pl(0usize),
                    self.pl(1usize),
                    self.pl(2usize),
                    self.pl(3usize),
                    self.pl(4usize),
                    self.pl(5usize),
                    self.pl(6usize),
                    self.pl(7usize),
                    self.pl(8usize),
                    self.pl(9usize),
                    self.pl(10usize),
                    self.pl(11usize),
                    self.pl(12usize),
                    self.pl(13usize),
                    self.pl(14usize),
                    self.pl(15usize),
                    self.pl(16usize),
                    self.pl(17usize),
                    self.pl(18usize),
                    self.pl(19usize),
                    self.pl(20usize),
                    self.pl(21usize),
                    self.pl(22usize),
                    self.pl(23usize),
                    self.pl(24usize),
                    self.pl(25usize),
                    self.pl(26usize),
                    self.pl(27usize),
                    self.pl(28usize),
                    self.pl(29usize),
                    self.pl(30usize),
                    self.pl(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PSR(pub u32);
    impl PSR {
        #[doc = "GPIO output port set (0=unaffected, 1=set high if output)"]
        #[inline(always)]
        pub const fn ps(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO output port set (0=unaffected, 1=set high if output)"]
        #[inline(always)]
        pub fn set_ps(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PSR {
        #[inline(always)]
        fn default() -> PSR {
            PSR(0)
        }
    }
    impl core::fmt::Debug for PSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PSR")
                .field(
                    "ps",
                    &[
                        self.ps(0usize),
                        self.ps(1usize),
                        self.ps(2usize),
                        self.ps(3usize),
                        self.ps(4usize),
                        self.ps(5usize),
                        self.ps(6usize),
                        self.ps(7usize),
                        self.ps(8usize),
                        self.ps(9usize),
                        self.ps(10usize),
                        self.ps(11usize),
                        self.ps(12usize),
                        self.ps(13usize),
                        self.ps(14usize),
                        self.ps(15usize),
                        self.ps(16usize),
                        self.ps(17usize),
                        self.ps(18usize),
                        self.ps(19usize),
                        self.ps(20usize),
                        self.ps(21usize),
                        self.ps(22usize),
                        self.ps(23usize),
                        self.ps(24usize),
                        self.ps(25usize),
                        self.ps(26usize),
                        self.ps(27usize),
                        self.ps(28usize),
                        self.ps(29usize),
                        self.ps(30usize),
                        self.ps(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PSR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PSR {
                ps: [bool; 32usize],
            }
            let proxy = PSR {
                ps: [
                    self.ps(0usize),
                    self.ps(1usize),
                    self.ps(2usize),
                    self.ps(3usize),
                    self.ps(4usize),
                    self.ps(5usize),
                    self.ps(6usize),
                    self.ps(7usize),
                    self.ps(8usize),
                    self.ps(9usize),
                    self.ps(10usize),
                    self.ps(11usize),
                    self.ps(12usize),
                    self.ps(13usize),
                    self.ps(14usize),
                    self.ps(15usize),
                    self.ps(16usize),
                    self.ps(17usize),
                    self.ps(18usize),
                    self.ps(19usize),
                    self.ps(20usize),
                    self.ps(21usize),
                    self.ps(22usize),
                    self.ps(23usize),
                    self.ps(24usize),
                    self.ps(25usize),
                    self.ps(26usize),
                    self.ps(27usize),
                    self.ps(28usize),
                    self.ps(29usize),
                    self.ps(30usize),
                    self.ps(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RER(pub u32);
    impl RER {
        #[doc = "GPIO port rising-edge detect enable"]
        #[inline(always)]
        pub const fn pe(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO port rising-edge detect enable"]
        #[inline(always)]
        pub fn set_pe(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for RER {
        #[inline(always)]
        fn default() -> RER {
            RER(0)
        }
    }
    impl core::fmt::Debug for RER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RER")
                .field(
                    "pe",
                    &[
                        self.pe(0usize),
                        self.pe(1usize),
                        self.pe(2usize),
                        self.pe(3usize),
                        self.pe(4usize),
                        self.pe(5usize),
                        self.pe(6usize),
                        self.pe(7usize),
                        self.pe(8usize),
                        self.pe(9usize),
                        self.pe(10usize),
                        self.pe(11usize),
                        self.pe(12usize),
                        self.pe(13usize),
                        self.pe(14usize),
                        self.pe(15usize),
                        self.pe(16usize),
                        self.pe(17usize),
                        self.pe(18usize),
                        self.pe(19usize),
                        self.pe(20usize),
                        self.pe(21usize),
                        self.pe(22usize),
                        self.pe(23usize),
                        self.pe(24usize),
                        self.pe(25usize),
                        self.pe(26usize),
                        self.pe(27usize),
                        self.pe(28usize),
                        self.pe(29usize),
                        self.pe(30usize),
                        self.pe(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RER {
                pe: [bool; 32usize],
            }
            let proxy = RER {
                pe: [
                    self.pe(0usize),
                    self.pe(1usize),
                    self.pe(2usize),
                    self.pe(3usize),
                    self.pe(4usize),
                    self.pe(5usize),
                    self.pe(6usize),
                    self.pe(7usize),
                    self.pe(8usize),
                    self.pe(9usize),
                    self.pe(10usize),
                    self.pe(11usize),
                    self.pe(12usize),
                    self.pe(13usize),
                    self.pe(14usize),
                    self.pe(15usize),
                    self.pe(16usize),
                    self.pe(17usize),
                    self.pe(18usize),
                    self.pe(19usize),
                    self.pe(20usize),
                    self.pe(21usize),
                    self.pe(22usize),
                    self.pe(23usize),
                    self.pe(24usize),
                    self.pe(25usize),
                    self.pe(26usize),
                    self.pe(27usize),
                    self.pe(28usize),
                    self.pe(29usize),
                    self.pe(30usize),
                    self.pe(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SDR(pub u32);
    impl SDR {
        #[doc = "Set GPIO port direction (0=unaffected, 1=set output)"]
        #[inline(always)]
        pub const fn sd(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Set GPIO port direction (0=unaffected, 1=set output)"]
        #[inline(always)]
        pub fn set_sd(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for SDR {
        #[inline(always)]
        fn default() -> SDR {
            SDR(0)
        }
    }
    impl core::fmt::Debug for SDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SDR")
                .field(
                    "sd",
                    &[
                        self.sd(0usize),
                        self.sd(1usize),
                        self.sd(2usize),
                        self.sd(3usize),
                        self.sd(4usize),
                        self.sd(5usize),
                        self.sd(6usize),
                        self.sd(7usize),
                        self.sd(8usize),
                        self.sd(9usize),
                        self.sd(10usize),
                        self.sd(11usize),
                        self.sd(12usize),
                        self.sd(13usize),
                        self.sd(14usize),
                        self.sd(15usize),
                        self.sd(16usize),
                        self.sd(17usize),
                        self.sd(18usize),
                        self.sd(19usize),
                        self.sd(20usize),
                        self.sd(21usize),
                        self.sd(22usize),
                        self.sd(23usize),
                        self.sd(24usize),
                        self.sd(25usize),
                        self.sd(26usize),
                        self.sd(27usize),
                        self.sd(28usize),
                        self.sd(29usize),
                        self.sd(30usize),
                        self.sd(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SDR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SDR {
                sd: [bool; 32usize],
            }
            let proxy = SDR {
                sd: [
                    self.sd(0usize),
                    self.sd(1usize),
                    self.sd(2usize),
                    self.sd(3usize),
                    self.sd(4usize),
                    self.sd(5usize),
                    self.sd(6usize),
                    self.sd(7usize),
                    self.sd(8usize),
                    self.sd(9usize),
                    self.sd(10usize),
                    self.sd(11usize),
                    self.sd(12usize),
                    self.sd(13usize),
                    self.sd(14usize),
                    self.sd(15usize),
                    self.sd(16usize),
                    self.sd(17usize),
                    self.sd(18usize),
                    self.sd(19usize),
                    self.sd(20usize),
                    self.sd(21usize),
                    self.sd(22usize),
                    self.sd(23usize),
                    self.sd(24usize),
                    self.sd(25usize),
                    self.sd(26usize),
                    self.sd(27usize),
                    self.sd(28usize),
                    self.sd(29usize),
                    self.sd(30usize),
                    self.sd(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SFER(pub u32);
    impl SFER {
        #[doc = "Set GPIO Falling Edge detect enable"]
        #[inline(always)]
        pub const fn sfer(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Set GPIO Falling Edge detect enable"]
        #[inline(always)]
        pub fn set_sfer(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for SFER {
        #[inline(always)]
        fn default() -> SFER {
            SFER(0)
        }
    }
    impl core::fmt::Debug for SFER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SFER")
                .field(
                    "sfer",
                    &[
                        self.sfer(0usize),
                        self.sfer(1usize),
                        self.sfer(2usize),
                        self.sfer(3usize),
                        self.sfer(4usize),
                        self.sfer(5usize),
                        self.sfer(6usize),
                        self.sfer(7usize),
                        self.sfer(8usize),
                        self.sfer(9usize),
                        self.sfer(10usize),
                        self.sfer(11usize),
                        self.sfer(12usize),
                        self.sfer(13usize),
                        self.sfer(14usize),
                        self.sfer(15usize),
                        self.sfer(16usize),
                        self.sfer(17usize),
                        self.sfer(18usize),
                        self.sfer(19usize),
                        self.sfer(20usize),
                        self.sfer(21usize),
                        self.sfer(22usize),
                        self.sfer(23usize),
                        self.sfer(24usize),
                        self.sfer(25usize),
                        self.sfer(26usize),
                        self.sfer(27usize),
                        self.sfer(28usize),
                        self.sfer(29usize),
                        self.sfer(30usize),
                        self.sfer(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SFER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SFER {
                sfer: [bool; 32usize],
            }
            let proxy = SFER {
                sfer: [
                    self.sfer(0usize),
                    self.sfer(1usize),
                    self.sfer(2usize),
                    self.sfer(3usize),
                    self.sfer(4usize),
                    self.sfer(5usize),
                    self.sfer(6usize),
                    self.sfer(7usize),
                    self.sfer(8usize),
                    self.sfer(9usize),
                    self.sfer(10usize),
                    self.sfer(11usize),
                    self.sfer(12usize),
                    self.sfer(13usize),
                    self.sfer(14usize),
                    self.sfer(15usize),
                    self.sfer(16usize),
                    self.sfer(17usize),
                    self.sfer(18usize),
                    self.sfer(19usize),
                    self.sfer(20usize),
                    self.sfer(21usize),
                    self.sfer(22usize),
                    self.sfer(23usize),
                    self.sfer(24usize),
                    self.sfer(25usize),
                    self.sfer(26usize),
                    self.sfer(27usize),
                    self.sfer(28usize),
                    self.sfer(29usize),
                    self.sfer(30usize),
                    self.sfer(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRER(pub u32);
    impl SRER {
        #[doc = "Set GPIO Rising Edge detect enable"]
        #[inline(always)]
        pub const fn srer(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Set GPIO Rising Edge detect enable"]
        #[inline(always)]
        pub fn set_srer(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for SRER {
        #[inline(always)]
        fn default() -> SRER {
            SRER(0)
        }
    }
    impl core::fmt::Debug for SRER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRER")
                .field(
                    "srer",
                    &[
                        self.srer(0usize),
                        self.srer(1usize),
                        self.srer(2usize),
                        self.srer(3usize),
                        self.srer(4usize),
                        self.srer(5usize),
                        self.srer(6usize),
                        self.srer(7usize),
                        self.srer(8usize),
                        self.srer(9usize),
                        self.srer(10usize),
                        self.srer(11usize),
                        self.srer(12usize),
                        self.srer(13usize),
                        self.srer(14usize),
                        self.srer(15usize),
                        self.srer(16usize),
                        self.srer(17usize),
                        self.srer(18usize),
                        self.srer(19usize),
                        self.srer(20usize),
                        self.srer(21usize),
                        self.srer(22usize),
                        self.srer(23usize),
                        self.srer(24usize),
                        self.srer(25usize),
                        self.srer(26usize),
                        self.srer(27usize),
                        self.srer(28usize),
                        self.srer(29usize),
                        self.srer(30usize),
                        self.srer(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRER {
                srer: [bool; 32usize],
            }
            let proxy = SRER {
                srer: [
                    self.srer(0usize),
                    self.srer(1usize),
                    self.srer(2usize),
                    self.srer(3usize),
                    self.srer(4usize),
                    self.srer(5usize),
                    self.srer(6usize),
                    self.srer(7usize),
                    self.srer(8usize),
                    self.srer(9usize),
                    self.srer(10usize),
                    self.srer(11usize),
                    self.srer(12usize),
                    self.srer(13usize),
                    self.srer(14usize),
                    self.srer(15usize),
                    self.srer(16usize),
                    self.srer(17usize),
                    self.srer(18usize),
                    self.srer(19usize),
                    self.srer(20usize),
                    self.srer(21usize),
                    self.srer(22usize),
                    self.srer(23usize),
                    self.srer(24usize),
                    self.srer(25usize),
                    self.srer(26usize),
                    self.srer(27usize),
                    self.srer(28usize),
                    self.srer(29usize),
                    self.srer(30usize),
                    self.srer(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[doc = "Port Level"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PLR_PL {
        #[doc = "Port is low"]
        LOW = 0x0,
        #[doc = "Port state is high"]
        HIGH = 0x01,
    }
    impl PLR_PL {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PLR_PL {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PLR_PL {
        #[inline(always)]
        fn from(val: u8) -> PLR_PL {
            PLR_PL::from_bits(val)
        }
    }
    impl From<PLR_PL> for u8 {
        #[inline(always)]
        fn from(val: PLR_PL) -> u8 {
            PLR_PL::to_bits(val)
        }
    }
}
