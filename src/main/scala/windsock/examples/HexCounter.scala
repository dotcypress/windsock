package windsock.examples

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._
import windsock.lib.comps._
import windsock.lib.pmods._
import windsock.bsp._

object HexCounter {
  def main(args: Array[String]) = ECPIX5.generate(new HexCounter)
}

case class HexCounter() extends Component {
  val io = new Bundle {
    val pmod0 = pmod(SevenSegmentDisplay())
    val pmod7 = pmod(SnapOff())
    val led0 = out(RgbLed())
    val led1 = out(RgbLed())
    val led2 = out(RgbLed())
    val led3 = out(RgbLed())
  }

  val core = new SlowArea(400 Hz) {
    val display = new SevenSegmentDisplayCtrl
    display.io.pins <> io.pmod0
    display.io.enable := True

    val snapOff = new SnapOffCtrl
    snapOff.io.pins <> io.pmod7

    snapOff.io.led1 := snapOff.io.button1
    snapOff.io.led2 := snapOff.io.button2
    snapOff.io.led3 := snapOff.io.button3
    snapOff.io.led4 := ~snapOff.io.button2
    snapOff.io.led5 := ~snapOff.io.button1
  }

  new SlowArea(1 Hz) {
    core.display.io.value := CounterFreeRun(256)

    io.led0.r := ~CounterFreeRun(2).willOverflow
    io.led0.g := ~CounterFreeRun(4).willOverflow
    io.led0.b := ~CounterFreeRun(8).willOverflow

    io.led1.r := ~CounterFreeRun(2).willOverflow
    io.led1.g := ~CounterFreeRun(4).willOverflow
    io.led1.b := ~CounterFreeRun(8).willOverflow

    io.led2.r := ~CounterFreeRun(2).willOverflow
    io.led2.g := ~CounterFreeRun(4).willOverflow
    io.led2.b := ~CounterFreeRun(8).willOverflow

    io.led3.r := ~CounterFreeRun(2).willOverflow
    io.led3.g := ~CounterFreeRun(4).willOverflow
    io.led3.b := ~CounterFreeRun(8).willOverflow
  }
}
