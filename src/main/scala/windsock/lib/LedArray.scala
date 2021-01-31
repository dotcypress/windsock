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

case class LedArrayCtrl(size: Int) extends Component {
  val io = new Bundle {
    val colors = slave(
      Flow(Vec.fill(size)(Rgb(8, 8, 8)))
    )
    val leds = out(LedArray(size))
  }

  val colors = io.colors.toReg()
  val counter = CounterFreeRun(256)

  Seq.tabulate(io.colors.payload.length)(idx => {
    io.leds.leds(idx).r := RegNext(counter > colors(idx).r).asUInt
    io.leds.leds(idx).g := RegNext(counter > colors(idx).g).asUInt
    io.leds.leds(idx).b := RegNext(counter > colors(idx).b).asUInt
  })
}
