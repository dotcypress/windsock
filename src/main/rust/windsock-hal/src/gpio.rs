//! General Purpose Input / Output

/// Default pin mode
pub type DefaultMode = Input;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
  /// The parts to split the GPIO into
  type Parts;

  /// Splits the GPIO block into independent pins and registers
  fn split(self) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input;

/// Output mode (type state)
pub struct Output;

macro_rules! gpio {
  ($GPIOX:ident, $gpiox:ident, $PXx:ident, [$($PXi:ident: ($pxi:ident, $idr:ident, $odr:ident, $moder:ident),)+]) => {
      pub mod $gpiox {
        use core::convert::Infallible;
        use core::marker::PhantomData;
        use hal::digital::v2::{toggleable, InputPin, OutputPin, StatefulOutputPin};
        use crate::$gpiox::Instance;
        use crate::{modify_reg, read_reg};
        use super::*;

        pub struct Parts {
          $(pub $pxi: $PXi<DefaultMode>,)+
        }

        impl GpioExt for Instance {
          type Parts = Parts;

          fn split(self) -> Parts {
            Parts {
              $(
                $pxi: $PXi { _mode: PhantomData },
              )+
            }
          }
        }

        pub struct $PXx<MODE> {
          _mode: PhantomData<MODE>,
        }

        $(
          pub struct $PXi<MODE> {
            _mode: PhantomData<MODE>,
          }

          impl Into<$PXi<Output>> for $PXi<DefaultMode> {
            fn into(self) -> $PXi<Output> {
              self.into_output()
            }
          }

          impl<MODE> $PXi<MODE> {
            /// Configures the pin to operate as input pin
            pub fn into_input(self) -> $PXi<Input> {
              unsafe { modify_reg!(crate::$gpiox, $GPIOX, MODER, $moder: 0x0) };
              $PXi { _mode: PhantomData }
            }

            /// Configures the pin to operate as an push pull output pin
            pub fn into_output(self) -> $PXi<Output> {
              unsafe { modify_reg!(crate::$gpiox, $GPIOX, MODER, $moder: 0x1) };
              $PXi { _mode: PhantomData }
            }
          }

          impl $PXi<Output> {
            pub fn downgrade(self) -> $PXx<Output> {
              $PXx { _mode: self._mode }
            }

            pub fn set_level(&mut self, level: bool) -> Result<(), Infallible> {
              unsafe { modify_reg!(crate::$gpiox, $GPIOX, ODR, $odr: level as u32) };
              Ok(())
            }
          }

          impl OutputPin for $PXi<Output> {
            type Error = Infallible;

            fn set_high(&mut self) -> Result<(), Self::Error> {
              unsafe { modify_reg!(crate::$gpiox, $GPIOX, ODR, $odr: 0x1) };
              Ok(())
            }

            fn set_low(&mut self) -> Result<(), Self::Error>{
              unsafe { modify_reg!(crate::$gpiox, $GPIOX, ODR, $odr: 0x0) };
              Ok(())
            }
          }

          impl StatefulOutputPin for $PXi<Output> {
            fn is_set_high(&self) -> Result<bool, Self::Error> {
              let is_set_high = !self.is_set_low()?;
              Ok(is_set_high)
            }

            fn is_set_low(&self) -> Result<bool, Self::Error> {
              let val = unsafe { read_reg!(crate::$gpiox, $GPIOX, ODR, $odr) };
              Ok(val == 0)
            }
          }

          impl $PXi<Input> {
            pub fn downgrade(self) -> $PXx<Input> {
              $PXx { _mode: self._mode }
            }
          }

          impl InputPin for $PXi<Input> {
            type Error = Infallible;

            fn is_high(&self) -> Result<bool, Self::Error> {
              let is_high = !self.is_low()?;
              Ok(is_high)
            }

            fn is_low(&self) -> Result<bool, Self::Error> {
              let val = unsafe { read_reg!(crate::$gpiox, $GPIOX, IDR, $idr) };
              Ok(val == 0)
            }
          }

          impl toggleable::Default for $PXi<Output> {
        }
      )+
    }
  }
}

gpio!(
  GPIOA,
  gpioa,
  PA,
  [
    PA0: (pa0, IDR0, ODR0, MODER0),
    PA1: (pa1, IDR1, ODR1, MODER1),
    PA2: (pa2, IDR2, ODR2, MODER2),
    PA3: (pa3, IDR3, ODR3, MODER3),
    PA4: (pa4, IDR4, ODR4, MODER4),
    PA5: (pa5, IDR5, ODR5, MODER5),
    PA6: (pa6, IDR6, ODR6, MODER6),
    PA7: (pa7, IDR7, ODR7, MODER7),
    PA8: (pa8, IDR8, ODR8, MODER8),
    PA9: (pa9, IDR9, ODR9, MODER9),
    PA10: (pa10, IDR10, ODR10, MODER10),
    PA11: (pa11, IDR11, ODR11, MODER11),
    PA12: (pa12, IDR12, ODR12, MODER12),
    PA13: (pa13, IDR13, ODR13, MODER13),
    PA14: (pa14, IDR14, ODR14, MODER14),
    PA15: (pa15, IDR15, ODR15, MODER15),
    PA16: (pa16, IDR16, ODR16, MODER16),
    PA17: (pa17, IDR17, ODR17, MODER17),
    PA18: (pa18, IDR18, ODR18, MODER18),
    PA19: (pa19, IDR19, ODR19, MODER19),
    PA20: (pa20, IDR20, ODR20, MODER20),
    PA21: (pa21, IDR21, ODR21, MODER21),
    PA22: (pa22, IDR22, ODR22, MODER22),
    PA23: (pa23, IDR23, ODR23, MODER23),
    PA24: (pa24, IDR24, ODR24, MODER24),
    PA25: (pa25, IDR25, ODR25, MODER25),
    PA26: (pa26, IDR26, ODR26, MODER26),
    PA27: (pa27, IDR27, ODR27, MODER27),
    PA28: (pa28, IDR28, ODR28, MODER28),
    PA29: (pa29, IDR29, ODR29, MODER29),
    PA30: (pa30, IDR30, ODR30, MODER30),
    PA31: (pa31, IDR31, ODR31, MODER31),
  ]
);
