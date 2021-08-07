package windsock.examples

import spinal.core._
import spinal.lib._
import spinal.lib.graphic.Rgb

import windsock.lib.{LedArray, LedArrayCtrl}
import windsock.lib.pmod._
import windsock.bsp._

object LedAnimation {
  def main(args: Array[String]) = ECPIX5.generate(new LedAnimation)
}

case class LedAnimation() extends Component {
  val io = new Bundle {
    val leds = out(LedArray())
  }

  val leds = new LedArrayCtrl()
  leds.io.leds <> io.leds
  leds.io.enable := True

  new SlowArea(32 Hz) {
    var counter = CounterFreeRun(16)

    leds.io.colors.valid := True
    Seq.tabulate(leds.io.colors.payload.length)(idx => {
      val color = leds.io.colors.payload(idx)
      color.r := (U(idx) - counter.value).resized
      color.g := 0
      color.b := (255 - U(idx) - counter.value).resized
    })
  }
}
