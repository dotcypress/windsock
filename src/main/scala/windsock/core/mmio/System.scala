package windsock.core.mmio

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba3.apb._
import spinal.lib.bus.misc.BusSlaveFactory

import windsock.lib.{LedArray, LedArrayCtrl}
import org.w3c.dom.css.RGBColor
import spinal.lib.graphic.Rgb

case class Apb3SystemCtrl() extends Component {
  val io = new Bundle {
    val apb = slave(Apb3(addressWidth = 8, dataWidth = 32))
    val leds = out(LedArray())
    val panic = out(Bool)
  }

  val busCtrl = Apb3SlaveFactory(io.apb)
  io.panic := busCtrl.createReadAndWrite(Bool(), 0x00)

  val leds = new LedArrayCtrl()
  leds.io.leds <> io.leds
  val enable = busCtrl.createReadAndWrite(Bool, 0x00, 1)
  leds.io.enable := enable
  leds.io.colors.valid := enable

  val sysClockFreq = S(clockDomain.frequency.getValue.toInt)
  val sysClockPeriod = S(1000000000 / clockDomain.frequency.getValue.toInt)
  busCtrl.read(sysClockFreq, 0x04)
  busCtrl.read(sysClockPeriod, 0x08)

  val rgb = Rgb(4, 4, 4)
  val colors = leds.io.colors.payload
  colors(0) := busCtrl.createReadAndWrite(Bits(12 bits), 0x0c, 0).as(rgb)
  colors(1) := busCtrl.createReadAndWrite(Bits(12 bits), 0x0c, 16).as(rgb)
  colors(2) := busCtrl.createReadAndWrite(Bits(12 bits), 0x10, 0).as(rgb)
  colors(3) := busCtrl.createReadAndWrite(Bits(12 bits), 0x10, 16).as(rgb)
}
