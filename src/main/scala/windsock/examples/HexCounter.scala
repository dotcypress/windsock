package windsock.examples

import spinal.core._
import spinal.lib._
import windsock.lib.ui._
import windsock.lib.pmods._
import windsock.bsp._

object HexCounter {
  def main(args: Array[String]) = ECPIX5.generate(new HexCounter)
}

case class HexCounter() extends Component {
  val io = new Bundle {
    val leds = out(LedArray())
    val pmod0 = master(SevenSegmentDisplay())
  }

  io.leds.switchOff()

  val core = new SlowArea(400 Hz) {
    val display = new SevenSegmentDisplayCtrl
    display.io.pins <> io.pmod0
    display.io.enable := True
  }

  new SlowArea(1 Hz) {
    core.display.io.value := CounterFreeRun(256)
  }
}
