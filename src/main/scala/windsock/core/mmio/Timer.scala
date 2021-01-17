package windsock.core.mmio

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba3.apb._
import spinal.lib.bus.misc.BusSlaveFactory
import spinal.lib.misc.Prescaler
import spinal.lib.misc.InterruptCtrl

case class Apb3TimerCtrl(width: Int = 16) extends Component {
  val io = new Bundle {
    val apb = slave(Apb3(addressWidth = 8, dataWidth = 32))
    val interrupt = out(Bool())
  }

  val prescaler = Prescaler(16)
  val counter = Reg(UInt(width bits))
  val limit = Reg(UInt(width bits))
  val limitHit = counter === limit

  val tick = False
  val clear = False
  val busClearing = False

  when(tick) {
    counter := counter + (!limitHit).asUInt
  }

  when(clear) {
    counter := 0
  }

  val busCtrl = Apb3SlaveFactory(io.apb)

  busCtrl.read(counter, 0x00)
  busClearing.setWhen(busCtrl.isWriting(0x00))

  prescaler.driveFrom(busCtrl, 0x04)

  busCtrl.driveAndRead(limit, 0x08)
  busClearing.setWhen(busCtrl.isWriting(0x08))

  val timerEnable = busCtrl.createReadAndWrite(
    Bits(1 bits),
    0x0c,
    0
  ) init (0)

  val autoReload = busCtrl.createReadAndWrite(
    Bits(1 bits),
    0x0c,
    16
  ) init (0)

  busCtrl.read(limitHit, 0x10)

  val interruptCtrl = InterruptCtrl(1)
  interruptCtrl.driveFrom(busCtrl, 0x14)
  io.interrupt := interruptCtrl.io.pendings.orR
  interruptCtrl.io.inputs(0) := limitHit

  clear.setWhen(busClearing | (autoReload.orR & limitHit))
  tick.setWhen(prescaler.io.overflow & timerEnable.orR)
}
