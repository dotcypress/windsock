use crate::prelude::*;
use crate::time::NanoSecond;
use hal::blocking::delay::{DelayMs, DelayUs};
use hal::timer::{CountDown, Periodic};
use void::Void;

pub struct Timer<TIM> {
  tim: TIM,
}

pub trait TimerExt<TIM> {
  fn timer(self) -> Timer<TIM>;
}

macro_rules! timer {
  ($tim:ident) => {
    use crate::$tim::Instance;
    use crate::{read_reg, write_reg, modify_reg};

    impl Timer<Instance> {
      pub fn $tim<T>(tim: Instance) -> Self {
        Timer { tim }
      }

      pub fn pause(&mut self) {
        modify_reg!(crate::$tim, self.tim, CFG, EN: 0);
      }

      pub fn resume(&mut self) {
        modify_reg!(crate::$tim, self.tim, CFG, EN: 1);
      }

      pub fn reset(&mut self) {
        write_reg!(crate::$tim, self.tim, COUNTER, 0);
      }

      pub fn enable_autoreload(&mut self) {
        modify_reg!(crate::$tim, self.tim, CFG, RELOAD: 1);
      }

      pub fn disable_autoreload(&mut self) {
        modify_reg!(crate::$tim, self.tim, CFG, RELOAD: 0);
      }

      pub fn listen(&mut self) {
        write_reg!(crate::$tim, self.tim, MASKR, 1);
      }

      pub fn unlisten(&mut self) {
        write_reg!(crate::$tim, self.tim, MASKR, 0);
      }

      pub fn is_pending(&mut self) -> bool {
        read_reg!(crate::$tim, self.tim, ISR) != 0
      }

      pub fn unpend(&mut self) {
        write_reg!(crate::$tim, self.tim, ISR, 1);
      }

      pub fn get_current(&self) -> u32 {
        read_reg!(crate::$tim, self.tim, COUNTER) as u32
      }

      pub fn delay<T>(&mut self, delay: T)
      where
          T: Into<NanoSecond>,
      {
        self.pause();
        self.reload(delay);
        self.resume();
        while read_reg!(crate::$tim, self.tim, SR) == 0 {}
      }

      pub fn reload<T>(&mut self, period: T)
      where
        T: Into<NanoSecond>,
      {
        let cycles = period.into().0 / crate::System::clk_period().0;
        let psc = cycles / 0xffff;
        let limit = cycles / (psc + 1);
        write_reg!(crate::$tim, self.tim, PSC, psc);
        write_reg!(crate::$tim, self.tim, LIMIT, limit);
      }

      pub fn release(self) -> Instance {
        self.tim
      }
    }

    impl TimerExt<Instance> for Instance {
      fn timer(self) -> Timer<Instance> {
        Timer::$tim::<Instance>(self)
      }
    }

    impl CountDown for Timer<Instance> {
      type Time = NanoSecond;

      fn start<T>(&mut self, timeout: T)
      where
        T: Into<NanoSecond>,
      {
        self.pause();
        self.reload(timeout);
        self.resume();
      }

      fn wait(&mut self) -> nb::Result<(), Void> {
        if read_reg!(crate::$tim, self.tim, SR) == 0 {
          Err(nb::Error::WouldBlock)
        } else {
          self.pause();
          Ok(())
        }
      }
    }

    impl Periodic for Timer<Instance> {}

    impl DelayUs<u32> for Timer<Instance> {
      fn delay_us(&mut self, us: u32) {
          self.delay(us.us())
      }
    }

    impl DelayUs<u16> for Timer<Instance> {
        fn delay_us(&mut self, us: u16) {
            self.delay_us(us as u32)
        }
    }

    impl DelayUs<u8> for Timer<Instance> {
      fn delay_us(&mut self, us: u8) {
          self.delay_us(us as u32)
      }
    }

    impl DelayMs<u32> for Timer<Instance> {
      fn delay_ms(&mut self, ms: u32) {
          self.delay_us(ms.saturating_mul(1_000));
      }
    }

    impl DelayMs<u16> for Timer<Instance> {
      fn delay_ms(&mut self, ms: u16) {
          self.delay_ms(ms as u32);
      }
    }

    impl DelayMs<u8> for Timer<Instance> {
      fn delay_ms(&mut self, ms: u8) {
          self.delay_ms(ms as u32);
      }
    }
  };
}

timer!(timer1);
