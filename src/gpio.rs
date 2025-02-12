#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]
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
    #[inline(always)]
    pub const fn plr(self) -> crate::common::Reg<regs::PLR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLR(pub u32);
    impl PLR {
        #[inline(always)]
        pub const fn pl(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_pl(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                pl: [bool; 32usize],
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
}
