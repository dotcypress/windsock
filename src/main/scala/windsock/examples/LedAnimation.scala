package windsock.examples

import spinal.core._
import spinal.lib._
import spinal.lib.graphic.Rgb
import windsock.bsp._
import windsock.lib.ui._
import windsock.lib.pmods._

object LedAnimation {
  def main(args: Array[String]) = ECPIX5.generate(new LedAnimation)
}

case class LedAnimation() extends Component {
  val io = new Bundle {
    val pmod7 = master(SnapOff())
    val pmod0 = master(SevenSegmentDisplay())
    val leds = out(LedArray())
  }

  val speed = Counter(4)

  val leds = new LedArrayCtrl(4)
  leds.io.leds <> io.leds

  val snapOff = new SnapOffCtrl
  snapOff.io.pins <> io.pmod7

  when(snapOff.io.button1.fall()) {
    speed.increment()
  }

  snapOff.io.led1 := snapOff.io.button1
  snapOff.io.led2 := speed === U(0)
  snapOff.io.led3 := speed === U(2)
  snapOff.io.led4 := speed === U(1)
  snapOff.io.led5 := speed === U(3)

  new SlowArea(500 Hz) {
    val display = new SevenSegmentDisplayCtrl
    display.io.pins <> io.pmod0
    display.io.enable := True
    display.io.value := speed.resized

    var counter = CounterFreeRun(256)

    leds.io.colors.valid := True
    Seq.tabulate(leds.io.colors.payload.length)(idx => {
      val color = leds.io.colors.payload(idx)
      val duty = counter.value + (U(idx) * 8 * speed)
      color.r := 0
      color.g := 0
      color.b := 255 - duty.resized
    })
  }
}
