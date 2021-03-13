package windsock.examples

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart.Uart
import windsock.bsp.ECPIX5
import windsock.lib._

object Serial {
  def main(args: Array[String]) = ECPIX5.generate(new Serial)
}

case class Serial() extends Component {
  val io = new Bundle {
    val leds = out(LedArray())
    val uart = master(Uart())
  }

  io.leds.powerOff()

  val counter = Counter(5 bit)
  var charStream = Stream(Bits())
  charStream.payload := B(U('a', 8 bits) + counter.resized)
  charStream.valid := False

  val log = UartSink()
  log.io.uart <> io.uart
  log.io.data <> charStream

  val timeout = Timeout(16 Hz)
  when(timeout) {
    charStream.valid := True
    counter.increment()
    timeout.clear()
  }
}
