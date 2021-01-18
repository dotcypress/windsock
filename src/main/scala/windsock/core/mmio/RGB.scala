package windsock.core.mmio

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba3.apb._
import spinal.lib.bus.misc.BusSlaveFactory
import windsock.lib.Color

case class Apb3RGBCtrl() extends Component {
  val io = new Bundle {
    val apb = slave(Apb3(addressWidth = 8, dataWidth = 32))
    val colors = slave(Flow(Color(8 bit)))
  }

  val busCtrl = Apb3SlaveFactory(io.apb)

  val lastColor = io.colors.toReg
  busCtrl.read(lastColor.asBits, 0x00)
}
