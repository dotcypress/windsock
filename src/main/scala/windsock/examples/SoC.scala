package windsock.examples

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._

import windsock.core._
import windsock.lib.pmod._
import windsock.lib.LedArray
import windsock.bsp._

object SoC {
  def main(args: Array[String]) = ECPIX5.generate(new SoC)
}

case class SoC() extends Component {
  val coreConfig = CoreConfig.withRamFile("src/main/resources/ram.hex")
  val io = new Bundle {
    val pmod7 = master(SnapOff())
    val uart = master(Uart())
    val leds = out(LedArray())
  }

  val core = new Core(coreConfig)
  core.io.uart <> io.uart
  core.io.leds <> io.leds

  val snapOff = new SnapOffCtrl
  snapOff.io.pins <> io.pmod7
  core.io.asyncReset <> ~snapOff.io.button3

  new SlowArea(8 Hz) {
    snapOff.io.led1 := core.io.panic && CounterFreeRun(2).willOverflow
  }

  val gpio = core.io.gpio

  gpio.write(0) <> snapOff.io.led2
  gpio.write(1) <> snapOff.io.led3
  gpio.write(2) <> snapOff.io.led4
  gpio.write(3) <> snapOff.io.led5

  gpio.read := 0
  snapOff.io.button1 <> gpio.read(31)
  snapOff.io.button2 <> gpio.read(30)
}
