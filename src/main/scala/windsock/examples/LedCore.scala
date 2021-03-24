package windsock.examples

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._
import spinal.lib.graphic.Rgb

import windsock.lib.{LedArray, LedArrayCtrl, UartStream}
import windsock.lib.pmod._
import windsock.bsp._

object LedCore {
  def main(args: Array[String]) = ECPIX5.generate(new LedCore)
}

case class LedCore() extends Component {
  val io = new Bundle {
    val uart = master(Uart())
    val leds = out(LedArray())
  }

  val serial = UartStream(txEnabled = false)
  serial.io.uart <> io.uart

  val leds = new LedArrayCtrl()
  leds.io.leds <> io.leds
  leds.io.enable := True

  val luma = CounterUpDown(
    16,
    serial.io.rx.valid && serial.io.rx.payload === B"8'x77",
    serial.io.rx.valid && serial.io.rx.payload === B"8'x73"
  )

  val colors = leds.io.colors
  colors.valid := True

  val animation = new SlowArea(2 Hz) {
    val blink = CounterFreeRun(2).willOverflow
  }

  Seq.tabulate(colors.payload.length)(idx => {
    val color = colors.payload(idx)
    color.r := 0
    color.g := 0
    color.b := Mux(animation.blink, luma.value, U(0))
  })
}
