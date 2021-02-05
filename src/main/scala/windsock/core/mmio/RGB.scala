package windsock.core.mmio

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba3.apb._
import spinal.lib.bus.misc.BusSlaveFactory
import spinal.lib.graphic._
import windsock.lib._

case class Apb3RGBCtrl() extends Component {
  val io = new Bundle {
    val apb = slave(Apb3(addressWidth = 8, dataWidth = 32))
    val colors = slave(Flow(Rgb(8, 8, 8)))
  }

  val busCtrl = Apb3SlaveFactory(io.apb)

  val color = io.colors.toReg
  busCtrl.read(color.asBits, 0x0, 0x8)
}
