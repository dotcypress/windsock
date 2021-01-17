#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General Purpose I/O

use crate::{RORegister, RWRegister};
use core::marker::PhantomData;

/// GPIO port input data register
pub mod IDR {
    /// Pin 31 input data
    pub mod IDR31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
    
        /// Mask (1 bit: 0x1 << 31)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 30 input data
    pub mod IDR30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
    
        /// Mask (1 bit: 0x1 << 30)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 29 input data
    pub mod IDR29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
    
        /// Mask (1 bit: 0x1 << 29)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 28 input data
    pub mod IDR28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
    
        /// Mask (1 bit: 0x1 << 28)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 27 input data
    pub mod IDR27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
    
        /// Mask (1 bit: 0x1 << 27)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 26 input data
    pub mod IDR26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
    
        /// Mask (1 bit: 0x1 << 26)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 25 input data
    pub mod IDR25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
    
        /// Mask (1 bit: 0x1 << 25)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 24 input data
    pub mod IDR24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
    
        /// Mask (1 bit: 0x1 << 24)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 23 input data
    pub mod IDR23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
    
        /// Mask (1 bit: 0x1 << 23)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 22 input data
    pub mod IDR22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
    
        /// Mask (1 bit: 0x1 << 22)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 21 input data
    pub mod IDR21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
    
        /// Mask (1 bit: 0x1 << 21)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 20 input data
    pub mod IDR20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
    
        /// Mask (1 bit: 0x1 << 20)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 19 input data
    pub mod IDR19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
    
        /// Mask (1 bit: 0x1 << 19)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 18 input data
    pub mod IDR18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
    
        /// Mask (1 bit: 0x1 << 18)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 17 input data
    pub mod IDR17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
    
        /// Mask (1 bit: 0x1 << 17)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 16 input data
    pub mod IDR16 {
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
    /// Pin 15 input data
    pub mod IDR15 {
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
    /// Pin 14 input data
    pub mod IDR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
    
        /// Mask (1 bit: 0x1 << 14)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 13 input data
    pub mod IDR13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
    
        /// Mask (1 bit: 0x1 << 13)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 12 input data
    pub mod IDR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
    
        /// Mask (1 bit: 0x1 << 12)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 11 input data
    pub mod IDR11 {
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
    /// Pin 10 input data
    pub mod IDR10 {
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
    /// Pin 9 input data
    pub mod IDR9 {
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
    /// Pin 8 input data
    pub mod IDR8 {
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
    /// Pin 7 input data
    pub mod IDR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
    
        /// Mask (1 bit: 0x1 << 7)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 6 input data
    pub mod IDR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
    
        /// Mask (1 bit: 0x1 << 6)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 5 input data
    pub mod IDR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
    
        /// Mask (1 bit: 0x1 << 5)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 4 input data
    pub mod IDR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
    
        /// Mask (1 bit: 0x1 << 4)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 3 input data
    pub mod IDR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
    
        /// Mask (1 bit: 0x1 << 3)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 2 input data
    pub mod IDR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
    
        /// Mask (1 bit: 0x1 << 2)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 1 input data
    pub mod IDR1 {
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
    /// Pin 0 input data
    pub mod IDR0 {
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

/// GPIO port output data register
pub mod ODR {
    /// Pin 31 output data
    pub mod ODR31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
    
        /// Mask (1 bit: 0x1 << 31)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 30 output data
    pub mod ODR30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
    
        /// Mask (1 bit: 0x1 << 30)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 29 output data
    pub mod ODR29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
    
        /// Mask (1 bit: 0x1 << 29)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 28 output data
    pub mod ODR28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
    
        /// Mask (1 bit: 0x1 << 28)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 27 output data
    pub mod ODR27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
    
        /// Mask (1 bit: 0x1 << 27)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 26 output data
    pub mod ODR26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
    
        /// Mask (1 bit: 0x1 << 26)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 25 output data
    pub mod ODR25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
    
        /// Mask (1 bit: 0x1 << 25)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 24 output data
    pub mod ODR24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
    
        /// Mask (1 bit: 0x1 << 24)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 23 output data
    pub mod ODR23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
    
        /// Mask (1 bit: 0x1 << 23)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 22 output data
    pub mod ODR22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
    
        /// Mask (1 bit: 0x1 << 22)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 21 output data
    pub mod ODR21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
    
        /// Mask (1 bit: 0x1 << 21)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 20 output data
    pub mod ODR20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
    
        /// Mask (1 bit: 0x1 << 20)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 19 output data
    pub mod ODR19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
    
        /// Mask (1 bit: 0x1 << 19)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 18 output data
    pub mod ODR18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
    
        /// Mask (1 bit: 0x1 << 18)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 17 output data
    pub mod ODR17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
    
        /// Mask (1 bit: 0x1 << 17)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 16 output data
    pub mod ODR16 {
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
    /// Pin 15 output data
    pub mod ODR15 {
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
    /// Pin 14 output data
    pub mod ODR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
    
        /// Mask (1 bit: 0x1 << 14)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 13 output data
    pub mod ODR13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
    
        /// Mask (1 bit: 0x1 << 13)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 12 output data
    pub mod ODR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
    
        /// Mask (1 bit: 0x1 << 12)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 11 output data
    pub mod ODR11 {
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
    /// Pin 10 output data
    pub mod ODR10 {
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
    /// Pin 9 output data
    pub mod ODR9 {
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
    /// Pin 8 output data
    pub mod ODR8 {
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
    /// Pin 7 output data
    pub mod ODR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
    
        /// Mask (1 bit: 0x1 << 7)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 6 output data
    pub mod ODR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
    
        /// Mask (1 bit: 0x1 << 6)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 5 output data
    pub mod ODR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
    
        /// Mask (1 bit: 0x1 << 5)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 4 output data
    pub mod ODR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
    
        /// Mask (1 bit: 0x1 << 4)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 3 output data
    pub mod ODR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
    
        /// Mask (1 bit: 0x1 << 3)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 2 output data
    pub mod ODR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
    
        /// Mask (1 bit: 0x1 << 2)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 1 output data
    pub mod ODR1 {
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
    /// Pin 0 output data
    pub mod ODR0 {
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

/// GPIO port mode register
pub mod MODER {
    /// Pin 31 mode
    pub mod MODER31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
    
        /// Mask (1 bit: 0x1 << 31)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 30 mode
    pub mod MODER30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
    
        /// Mask (1 bit: 0x1 << 30)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 29 mode
    pub mod MODER29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
    
        /// Mask (1 bit: 0x1 << 29)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 28 mode
    pub mod MODER28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
    
        /// Mask (1 bit: 0x1 << 28)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 27 mode
    pub mod MODER27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
    
        /// Mask (1 bit: 0x1 << 27)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 26 mode
    pub mod MODER26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
    
        /// Mask (1 bit: 0x1 << 26)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 25 mode
    pub mod MODER25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
    
        /// Mask (1 bit: 0x1 << 25)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 24 mode
    pub mod MODER24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
    
        /// Mask (1 bit: 0x1 << 24)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 23 mode
    pub mod MODER23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
    
        /// Mask (1 bit: 0x1 << 23)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 22 mode
    pub mod MODER22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
    
        /// Mask (1 bit: 0x1 << 22)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 21 mode
    pub mod MODER21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
    
        /// Mask (1 bit: 0x1 << 21)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 20 mode
    pub mod MODER20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
    
        /// Mask (1 bit: 0x1 << 20)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 19 mode
    pub mod MODER19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
    
        /// Mask (1 bit: 0x1 << 19)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 18 mode
    pub mod MODER18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
    
        /// Mask (1 bit: 0x1 << 18)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 17 mode
    pub mod MODER17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
    
        /// Mask (1 bit: 0x1 << 17)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 16 mode
    pub mod MODER16 {
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
    /// Pin 15 mode
    pub mod MODER15 {
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
    /// Pin 14 mode
    pub mod MODER14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
    
        /// Mask (1 bit: 0x1 << 14)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 13 mode
    pub mod MODER13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
    
        /// Mask (1 bit: 0x1 << 13)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 12 mode
    pub mod MODER12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
    
        /// Mask (1 bit: 0x1 << 12)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 11 mode
    pub mod MODER11 {
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
    /// Pin 10 mode
    pub mod MODER10 {
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
    /// Pin 9 mode
    pub mod MODER9 {
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
    /// Pin 8 mode
    pub mod MODER8 {
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
    /// Pin 7 mode
    pub mod MODER7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
    
        /// Mask (1 bit: 0x1 << 7)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 6 mode
    pub mod MODER6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
    
        /// Mask (1 bit: 0x1 << 6)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 5 mode
    pub mod MODER5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
    
        /// Mask (1 bit: 0x1 << 5)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 4 mode
    pub mod MODER4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
    
        /// Mask (1 bit: 0x1 << 4)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 3 mode
    pub mod MODER3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
    
        /// Mask (1 bit: 0x1 << 3)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 2 mode
    pub mod MODER2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
    
        /// Mask (1 bit: 0x1 << 2)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Pin 1 mode
    pub mod MODER1 {
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
    /// Pin 0 mode
    pub mod MODER0 {
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
    /// GPIO port input data register
    pub IDR: RORegister<u32>,

    /// GPIO port output data register
    pub ODR: RWRegister<u32>,

    /// GPIO port mode register
    pub MODER: RWRegister<u32>,
}

pub struct ResetValues {
    pub IDR: u32,
    pub ODR: u32,
    pub MODER: u32,
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
