package windsock.lib

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._

case class UartPump(
    baudrate: HertzNumber,
    dataClock: ClockDomain = ClockDomain.current,
    fifoDepth: Int = 256,
    waterline: Int = 256 / 4
) extends Component {
  val io = new Bundle {
    val uart = master(Uart())
    val data = master(Stream(Bits(8 bits)))
  }

  val uartCtrl = new UartCtrl()
  uartCtrl.io.uart <> io.uart

  uartCtrl.io.config.setClockDivider(baudrate)
  uartCtrl.io.config.frame.dataLength := 7
  uartCtrl.io.config.frame.parity := UartParityType.NONE
  uartCtrl.io.config.frame.stop := UartStopType.ONE
  uartCtrl.io.writeBreak := False

  val fifo = new StreamFifoCC(
    Bits(8 bits),
    fifoDepth,
    pushClock = clockDomain,
    popClock = dataClock
  )
  fifo.io.push << uartCtrl.io.read
  fifo.io.pop >> io.data

  dataClock {
    uartCtrl.io.write.payload := B(0)
    uartCtrl.io.write.valid := False

    when(fifo.io.popOccupancy > fifoDepth - waterline) {
      val xoff = B"8'x13"
      uartCtrl.io.write.payload := xoff
      uartCtrl.io.write.valid := True
    }

    when(fifo.io.popOccupancy < waterline) {
      val xon = B"8'x11"
      uartCtrl.io.write.payload := xon
      uartCtrl.io.write.valid := True
    }
  }
}
