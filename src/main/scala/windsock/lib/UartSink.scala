package windsock.lib

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._

case class UartSink() extends Component {
  val io = new Bundle {
    val uart = master(Uart())
    val sink = slave(Stream(Bits(8 bits)))
  }

  val uartCtrl = new UartCtrl()
  uartCtrl.io.config.setClockDivider(115.2 kHz)
  uartCtrl.io.config.frame.dataLength := 7
  uartCtrl.io.config.frame.parity := UartParityType.NONE
  uartCtrl.io.config.frame.stop := UartStopType.ONE
  uartCtrl.io.writeBreak := False

  uartCtrl.io.uart <> io.uart
  uartCtrl.io.write <> io.sink
}
