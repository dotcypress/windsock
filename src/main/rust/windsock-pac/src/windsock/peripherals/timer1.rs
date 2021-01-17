#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Timer1

use crate::{RWRegister};
use core::marker::PhantomData;

/// Timer counter register
pub mod COUNTER {
}

/// Timer prescaler register
pub mod PSC {
}

/// Timer limit register
pub mod LIMIT {
}

/// Timer configuration register
pub mod CFG {
    /// Timer auto reload
    pub mod RELOAD {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (1 bit: 0x1 << 16)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Timer enable
    pub mod EN {
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
    
    }}

/// Status register
pub mod SR {
}

/// Timer interrupt status register
pub mod ISR {
}

/// Timer interrupt mask register
pub mod MASKR {
}

pub struct RegisterBlock {
    /// Timer counter register
    pub COUNTER: RWRegister<u32>,

    /// Timer prescaler register
    pub PSC: RWRegister<u32>,

    /// Timer limit register
    pub LIMIT: RWRegister<u32>,

    /// Timer configuration register
    pub CFG: RWRegister<u32>,

    /// Status register
    pub SR: RWRegister<u32>,

    /// Timer interrupt status register
    pub ISR: RWRegister<u32>,

    /// Timer interrupt mask register
    pub MASKR: RWRegister<u32>,
}

pub struct ResetValues {
    pub COUNTER: u32,
    pub PSC: u32,
    pub LIMIT: u32,
    pub CFG: u32,
    pub SR: u32,
    pub ISR: u32,
    pub MASKR: u32,
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
