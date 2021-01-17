#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System information and status register

use crate::{RORegister, WORegister};
use core::marker::PhantomData;

/// Application panic flag
pub mod PANIC {
}

/// System clock frequency
pub mod CLOCK {
}

/// System clock period in nanoseconds
pub mod PERIOD {
}

pub struct RegisterBlock {
    /// Application panic flag
    pub PANIC: WORegister<u32>,

    /// System clock frequency
    pub CLOCK: RORegister<u32>,

    /// System clock period in nanoseconds
    pub PERIOD: RORegister<u32>,
}

pub struct ResetValues {
    pub PANIC: u32,
    pub CLOCK: u32,
    pub PERIOD: u32,
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
