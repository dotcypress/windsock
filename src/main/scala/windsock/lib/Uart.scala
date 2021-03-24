package windsock.lib

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._

case class UartStream(
    baudrate: HertzNumber = 115.2 kHz,
    txEnabled: Boolean = true,
    rxEnabled: Boolean = true
) extends Component {
  val io = new Bundle {
    val tx = if (txEnabled) slave(Stream(Bits(8 bits))) else null
    val rx = if (rxEnabled) master(Stream(Bits(8 bits))) else null
    val uart = master(Uart())
  }

  val uartCtrl = new UartCtrl()
  uartCtrl.io.writeBreak := False
  uartCtrl.io.config.frame.dataLength := 7
  uartCtrl.io.config.frame.parity := UartParityType.NONE
  uartCtrl.io.config.frame.stop := UartStopType.ONE
  uartCtrl.io.config.setClockDivider(baudrate)
  uartCtrl.io.uart <> io.uart

  if (txEnabled) {
    uartCtrl.io.write <> io.tx
  } else {
    uartCtrl.io.write.payload := 0
    uartCtrl.io.write.valid := False
  }

  if (rxEnabled) {
    uartCtrl.io.read <> io.rx
  }
}

case class UartSink(
    baudrate: HertzNumber = 115.2 kHz,
    dataWidth: BitCount = 8 bits,
    fifoDepth: Int = 32,
    endianness: Endianness = LITTLE,
    padding: Boolean = false
) extends Component {
  val io = new Bundle {
    val data = slave(Stream(Bits(dataWidth)))
    val uart = master(Uart())
  }

  val fifo = io.data.queue(fifoDepth)

  val serial = new UartStream(baudrate, rxEnabled = false)
  serial.io.uart <> io.uart

  if (dataWidth.value == 8) {
    serial.io.tx <> fifo
  } else {
    StreamWidthAdapter(
      input = fifo,
      output = serial.io.tx,
      endianness = endianness,
      padding = padding
    )
  }
}

case class UartPump(
    baudrate: HertzNumber = 115.2 kHz,
    dataClock: ClockDomain = ClockDomain.current,
    fifoDepth: Int = 256,
    waterline: Int = 256 / 4
) extends Component {
  val io = new Bundle {
    val uart = master(Uart())
    val data = master(Stream(Bits(8 bits)))
  }

  val serial = new UartStream(baudrate)
  serial.io.uart <> io.uart

  val fifo = new StreamFifoCC(
    Bits(8 bits),
    fifoDepth,
    pushClock = clockDomain,
    popClock = dataClock
  )
  fifo.io.push << serial.io.rx
  fifo.io.pop >> io.data

  dataClock {
    val tx = serial.io.tx
    tx.payload := 0
    tx.valid := False

    when(fifo.io.popOccupancy > fifoDepth - waterline) {
      val xoff = B"8'x13"
      tx.payload := xoff
      tx.valid := True
    }

    when(fifo.io.popOccupancy < waterline) {
      val xon = B"8'x11"
      tx.payload := xon
      tx.valid := True
    }
  }
}
