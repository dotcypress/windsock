package windsock.examples

import spinal.core._
import spinal.lib._
import blackbox._

import windsock.lib.LedArray
import windsock.lib.pmod._
import windsock.bsp._
import spinal.lib.graphic.RgbConfig
import spinal.lib.graphic.vga.VgaCtrl
import spinal.lib.graphic.vga.Vga
object VGA {
  def main(args: Array[String]) = ECPIX5.generate(new VGA)
}

case class VGA() extends Component {
  val rgbConfig = RgbConfig(4, 4, 4)
  val io = new Bundle {
    val hdmiReset = out(Bool())
    val pixelClock = out(Bool())
    val vga = master(Vga(rgbConfig))
  }

  io.hdmiReset := True

  val pll = PLL(
    PLLConfig(
      // 25.175 MHz
      refClockDivisor = B(13),
      feedbackDivisor = B(3),
      primaryClockConfig = PllClockConfig(B(24))
    )
  )
  pll.io.clockIn := ClockDomain.current.readClockWire
  io.pixelClock := pll.io.primaryClockOut

  val animation = new SlowArea(10 Hz) {
    val offset = CounterFreeRun(16)
  }

  ClockDomain(
    pll.io.primaryClockOut,
    ClockDomain.current.readResetWire
  ) {

    val ctrl = new VgaCtrl(rgbConfig)
    ctrl.io.softReset := False
    ctrl.io.timings.setAs_h640_v480_r60
    ctrl.io.vga <> io.vga

    val x = Counter(640)
    val y = Counter(480)

    when(ctrl.io.vga.colorEn) {
      x.increment()
    }

    when(ctrl.io.vga.hSync.fall()) {
      x.clear()
      y.increment()
    }

    when(ctrl.io.vga.vSync.fall()) {
      x.clear()
      y.clear()
    }

    val counter = Reg(UInt(4 bits))

    val off = U(0)
    val on = U(15)

    ctrl.io.pixels.valid := True
    ctrl.io.pixels.r := (x > 40 & x < 50) ? on | off
    ctrl.io.pixels.g := (x > 120 & x < 150) ? on | off
    ctrl.io.pixels.b := (y > 80 & y < 100) ? on | off

    counter := counter + 1
    when(ctrl.io.frameStart) {
      counter := 0
    }
  }
}
