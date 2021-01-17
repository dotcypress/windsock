#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal synchronous asynchronous receiver/transmitter

use crate::{RWRegister};
use core::marker::PhantomData;

/// UART data register
pub mod DATA {
    /// TX data
    pub mod TX {
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
    
    }
    /// RX data
    pub mod RX {
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

/// UART interrupt status register
pub mod ISR {
    /// RX fifo occupancy
    pub mod RX_FIFO {
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
    /// TX FIFO availability
    pub mod TX_FIFO {
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
    /// TX active flag
    pub mod TX_ACTIVE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
    
        /// Mask (1 bit: 0x1 << 15)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// RX interrupt pending flag
    pub mod RX_INT_PEND {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
    
        /// Mask (1 bit: 0x1 << 9)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// TX interrupt pending flag
    pub mod TX_INT_PEND {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
    
        /// Mask (1 bit: 0x1 << 8)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// RX interrupt enable
    pub mod RX_INT_EN {
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
    
    }
    /// TX interrupt enable
    pub mod TX_INT_EN {
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

/// UART clock divider register
pub mod DIV {
}

/// UART frame configuration register
pub mod CFG {
    /// Stop bits
    pub mod STOP {
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
    /// Parity
    pub mod PARITY {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
    
        /// Mask (2 bit: 0x3 << 8)
        pub const mask: u32 = 0x3 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Word length
    pub mod LENGTH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (4 bit: 0xf << 0)
        pub const mask: u32 = 0xf << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

/// UART RX status register
pub mod RXSR {
    /// Stop bits
    pub mod BREAK_CLEAR {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
    
        /// Mask (1 bit: 0x1 << 11)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Stop bits
    pub mod BREAK_REQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
    
        /// Mask (1 bit: 0x1 << 10)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Stop bits
    pub mod BREAK_DET {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
    
        /// Mask (1 bit: 0x1 << 9)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Stop bits
    pub mod BREAK {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
    
        /// Mask (1 bit: 0x1 << 8)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// RX overflow flag
    pub mod OVERFLOW {
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
    
    }
    /// RX error flag
    pub mod ERROR {
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

pub struct RegisterBlock {
    /// UART data register
    pub DATA: RWRegister<u32>,

    /// UART interrupt status register
    pub ISR: RWRegister<u32>,

    /// UART clock divider register
    pub DIV: RWRegister<u32>,

    /// UART frame configuration register
    pub CFG: RWRegister<u32>,

    /// UART RX status register
    pub RXSR: RWRegister<u32>,
}

pub struct ResetValues {
    pub DATA: u32,
    pub ISR: u32,
    pub DIV: u32,
    pub CFG: u32,
    pub RXSR: u32,
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
