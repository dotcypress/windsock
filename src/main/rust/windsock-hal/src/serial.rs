use crate::prelude::*;
use crate::time::Bps;
use core::fmt;
use core::marker::PhantomData;
use nb::block;

#[derive(Debug)]
pub enum Error {
  Framing,
  Overrun,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum WordLength {
  DataBits7 = 0x6,
  DataBits8 = 0x7,
  DataBits9 = 0x8,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Parity {
  ParityNone = 0x0,
  ParityEven = 0x1,
  ParityOdd = 0x2,
}

pub enum StopBits {
  One = 0x0,
  Two = 0x1,
}

pub struct Config {
  baudrate: Bps,
  word_length: WordLength,
  parity: Parity,
  stop_bits: StopBits,
}

impl Config {
  pub fn baudrate(mut self, baudrate: Bps) -> Self {
    self.baudrate = baudrate;
    self
  }

  pub fn parity_none(mut self) -> Self {
    self.parity = Parity::ParityNone;
    self
  }

  pub fn parity_even(mut self) -> Self {
    self.parity = Parity::ParityEven;
    self
  }

  pub fn parity_odd(mut self) -> Self {
    self.parity = Parity::ParityOdd;
    self
  }

  pub fn word_length_8(mut self) -> Self {
    self.word_length = WordLength::DataBits8;
    self
  }

  pub fn word_length_9(mut self) -> Self {
    self.word_length = WordLength::DataBits9;
    self
  }

  pub fn stop_bits(mut self, stop_bits: StopBits) -> Self {
    self.stop_bits = stop_bits;
    self
  }
}

impl Default for Config {
  fn default() -> Config {
    let baudrate = 115_200.bps();
    Config {
      baudrate,
      word_length: WordLength::DataBits8,
      parity: Parity::ParityNone,
      stop_bits: StopBits::One,
    }
  }
}

/// Serial receiver
pub struct Rx<UART> {
  _uart: PhantomData<UART>,
}

/// Serial transmitter
pub struct Tx<UART> {
  _uart: PhantomData<UART>,
}

/// Serial abstraction
pub struct Serial<UART> {
  tx: Tx<UART>,
  rx: Rx<UART>,
}

pub trait SerialExt<UART> {
  fn serial(self, config: Config) -> Serial<UART>;
}

impl<UART> fmt::Write for Serial<UART>
where
  Serial<UART>: hal::serial::Write<u8>,
{
  fn write_str(&mut self, s: &str) -> fmt::Result {
    let _ = s.as_bytes().iter().map(|c| block!(self.write(*c))).last();
    Ok(())
  }
}

impl<UART> fmt::Write for Tx<UART>
where
  Tx<UART>: hal::serial::Write<u8>,
{
  fn write_str(&mut self, s: &str) -> fmt::Result {
    let _ = s.as_bytes().iter().map(|c| block!(self.write(*c))).last();
    Ok(())
  }
}

macro_rules! uart {
  ($UARTX: ident, $uartX:ident) => {
    use super::*;
    use crate::$uartX::Instance;
    use crate::{read_reg, write_reg, modify_reg};

    impl SerialExt<Instance> for Instance {
      fn serial(self, config: Config) -> Serial<Instance> {
        Serial::$uartX(self, config)
      }
    }

    impl Serial<Instance> {
      pub fn $uartX(uart: Instance, config: Config) -> Self {
        let clk = crate::Device::clk_frequency().0 as u64;
        let div = (clk / config.baudrate.0 as u64 / 5) - 1;
        write_reg!(crate::$uartX, uart, DIV, div as u32);
        modify_reg!(crate::$uartX, uart, CFG,
          STOP: config.stop_bits as u32,
          PARITY: config.parity as u32,
          LENGTH: config.word_length as u32
        );
        Serial {
          tx: Tx { _uart: PhantomData },
          rx: Rx { _uart: PhantomData },
        }
      }

      pub fn split(self) -> (Tx<Instance>, Rx<Instance>) {
        (self.tx, self.rx)
      }

      pub fn tx(&mut self) -> &mut Tx<Instance> {
        &mut self.tx
      }

      pub fn rx(&mut self) -> &mut Rx<Instance> {
        &mut self.rx
      }
    }

    impl Rx<Instance> {
      pub fn fifo_occupancy(&self) -> u8 {
        let val = unsafe {
          read_reg!(crate::$uartX, $UARTX, ISR, RX_FIFO)
        };
        val as u8
      }

      pub fn fifo_full(&self) -> bool {
        self.fifo_occupancy() == 0x0f
      }

      pub fn fifo_empty(&self) -> bool {
        self.fifo_occupancy() == 0
      }

      pub fn listen(&mut self) {
        unsafe {
          modify_reg!(crate::$uartX, $UARTX, ISR, RX_INT_EN: 1);
        }
      }

      pub fn unlisten(&mut self) {
        unsafe {
          modify_reg!(crate::$uartX, $UARTX, ISR, RX_INT_EN: 0);
        }
      }

      pub fn is_pending(&mut self) -> bool {
        unsafe {
          read_reg!(crate::$uartX, $UARTX, ISR, RX_INT_PEND) != 0
        }
      }
    }

    impl hal::serial::Read<u8> for Rx<Instance> {
      type Error = Error;

      fn read(&mut self) -> nb::Result<u8, Error> {
        let byte = unsafe {
          let (err, overflow) = read_reg!(crate::$uartX, $UARTX, RXSR, ERROR, OVERFLOW);
          if overflow != 0{
            return Err(nb::Error::Other(Error::Overrun));
          } else if err != 0{
            return Err(nb::Error::Other(Error::Framing));
          } else if self.fifo_empty() {
            return Err(nb::Error::WouldBlock);
          }
          read_reg!(crate::$uartX, $UARTX, DATA, RX) as u8
        };
        Ok(byte)
      }
    }

    impl Tx<Instance> {
      pub fn fifo_availability(&self) -> u8 {
        let val = unsafe {
          read_reg!(crate::$uartX, $UARTX, ISR, TX_FIFO)
        };
        val as u8
      }

      pub fn fifo_full(&self) -> bool {
        self.fifo_availability() == 0
      }

      pub fn fifo_empty(&self) -> bool {
        self.fifo_availability() == 0x0f
      }

      pub fn listen(&mut self) {
        unsafe {
          modify_reg!(crate::$uartX, $UARTX, ISR, TX_INT_EN: 1);
        }
      }

      pub fn unlisten(&mut self) {
        unsafe {
          modify_reg!(crate::$uartX, $UARTX, ISR, TX_INT_EN: 0);
        }
      }

      pub fn is_pending(&mut self) -> bool {
        unsafe {
          read_reg!(crate::$uartX, $UARTX, ISR, TX_INT_PEND) != 0
        }
      }
    }

    impl hal::serial::Write<u8> for Tx<Instance> {
      type Error = Error;

      fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.fifo_empty() {
          Ok(())
        } else {
          Err(nb::Error::WouldBlock)
        }
      }

      fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        if self.fifo_full() {
          Err(nb::Error::WouldBlock)
        } else {
          unsafe {
            write_reg!(crate::$uartX, $UARTX, DATA, TX: byte as u32)
          }
          Ok(())
        }
      }
    }

    impl hal::serial::Read<u8> for Serial<Instance> {
      type Error = Error;

      fn read(&mut self) -> nb::Result<u8, Error> {
        self.rx.read()
      }
    }

    impl hal::serial::Write<u8> for Serial<Instance> {
      type Error = Error;

      fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush()
      }

      fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.tx.write(byte)
      }
    }
  };
}

uart!(UART1, uart1);
