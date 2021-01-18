#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! RGB leds and Color sensor

use crate::{RORegister};
use core::marker::PhantomData;

/// Color sensor input data register
pub mod SENSOR {
    /// Blue component
    pub mod BLUE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
    
        /// Mask (8 bit: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Green component
    pub mod GREEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (8 bit: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Red component
    pub mod RED {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
    
        /// Mask (8 bit: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Luminusity
    pub mod LUMA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (8 bit: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

pub struct RegisterBlock {
    /// Color sensor input data register
    pub SENSOR: RORegister<u32>,
}

pub struct ResetValues {
    pub SENSOR: u32,
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
