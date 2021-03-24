#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System information and status register

use crate::{RORegister, RWRegister};
use core::marker::PhantomData;

/// System reset and control register
pub mod CONTROL {
    /// Application panic flag
    pub mod PANIC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (1 bit: 0x1 << 0)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Onboard leds controler enable
    pub mod ENABLE_LEDS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
    
        /// Mask (1 bit: 0x1 << 1)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

/// System clock frequency in Hertz
pub mod CLOCK {
}

/// System clock period in nanoseconds
pub mod PERIOD {
}

/// Leds colors register
pub mod LEDA {
    /// Led1 color
    pub mod COLOR0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (12 bit: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Led2 color
    pub mod COLOR1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (12 bit: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

/// Leds colors register
pub mod LEDB {
    /// Led2 color
    pub mod COLOR2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (12 bit: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Led3 color
    pub mod COLOR3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (12 bit: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

pub struct RegisterBlock {
    /// System reset and control register
    pub CONTROL: RWRegister<u32>,

    /// System clock frequency in Hertz
    pub CLOCK: RORegister<u32>,

    /// System clock period in nanoseconds
    pub PERIOD: RORegister<u32>,

    /// Leds colors register
    pub LEDA: RWRegister<u32>,

    /// Leds colors register
    pub LEDB: RWRegister<u32>,
}

pub struct ResetValues {
    pub CONTROL: u32,
    pub CLOCK: u32,
    pub PERIOD: u32,
    pub LEDA: u32,
    pub LEDB: u32,
}

pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}

impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
