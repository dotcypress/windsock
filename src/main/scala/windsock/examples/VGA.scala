package windsock.examples

import spinal.core._
import spinal.lib._
import blackbox._

import windsock.lib.LedArray
import windsock.lib.pmod._
import windsock.bsp._
object VGA {
  def main(args: Array[String]) = ECPIX5.generate(new VGA)
}

case class VGA() extends Component {
  val io = new Bundle {
    val pmod5 = master(DVI())
    val pmod4 = master(DVI())
  }

  val pll = PLL(
    PLLConfig(
      // 25.175 MHz
      refClockDivisor = B(13),
      feedbackDivisor = B(3),
      primaryClockConfig = PllClockConfig(B(24))
    )
  )
  pll.io.clockIn := ClockDomain.current.readClockWire

  val animation = new SlowArea(10 Hz) {
    val offset = CounterFreeRun(16)
  }

  ClockDomain(
    pll.io.primaryClockOut,
    ClockDomain.current.readResetWire
  ) {
    val counter = Reg(UInt(4 bits))

    val dvi = new DVICtrl
    dvi.io.pinsA <> io.pmod5
    dvi.io.pinsB <> io.pmod4
    dvi.io.pixels.valid := True

    val point = dvi.io.position.payload
    val off = U(0)
    val on = U(15)

    dvi.io.pixels.r := (point.x > 40 & point.x < 50) ? on | off
    dvi.io.pixels.g := (point.x > 120 & point.x < 150) ? on | off
    dvi.io.pixels.b := (point.y > 80 & point.y < 100) ? on | off

    counter := counter + 1
    when(dvi.io.frameStart) {
      counter := 0
    }
  }
}
