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

  val log = UartSink()
  log.io.uart <> io.uart

  val counter = Counter(4 bit)
  val timeout = Timeout(200 ms)

  when(log.io.sink.ready) {
    counter.increment()
    timeout.clear()
  }

  val letter = U('a', 8 bits) + counter.resized
  log.io.sink.valid := timeout
  log.io.sink.payload := letter.asBits
}
