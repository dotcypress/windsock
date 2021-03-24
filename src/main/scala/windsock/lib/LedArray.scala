package windsock.lib

import spinal.core._
import spinal.lib._
import spinal.lib.graphic._

case class LedArray(size: Int = 4) extends Bundle {
  val leds = Vec.fill(size)(Rgb(RgbConfig(1, 1, 1)))

  def powerOff() = {
    this.allowOverride
    leds.foreach(led => {
      led.r := U(1)
      led.g := U(1)
      led.b := U(1)
    })
  }
}

case class LedArrayCtrl(size: Int = 4) extends Component {
  val io = new Bundle {
    val colors = slave(Flow(Vec.fill(size)(Rgb(4, 4, 4))))
    val enable = in(Bool)

    val leds = out(LedArray(size))
  }

  val counter = Counter(8 bit, io.enable)
  val colors = io.colors.toReg()

  val disable = ~io.enable.asUInt
  val leds = io.leds.leds

  val gamma = Vec[UInt](
    0, 3, 7, 10, 18, 28, 41, 56, 73, 92, 113, 137, 163, 192, 222, 255
  )

  Seq.tabulate(colors.length)(idx => {
    val color = colors(idx)
    leds(idx).r := disable | RegNext(counter >= gamma(color.r)).asUInt
    leds(idx).g := disable | RegNext(counter >= gamma(color.g)).asUInt
    leds(idx).b := disable | RegNext(counter >= gamma(color.b)).asUInt
  })
}
