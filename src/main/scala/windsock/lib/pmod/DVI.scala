package windsock.lib.pmod

import spinal.core._
import spinal.lib._
import spinal.lib.fsm._
import spinal.lib.graphic._
import spinal.lib.graphic.vga._

case class Coordinate(maxX: Int, maxY: Int) extends Bundle {
  val x = UInt(log2Up(maxX) bits)
  val y = UInt(log2Up(maxY) bits)
}

case class DVI() extends PMODBundle {
  override def asMaster() = this.asOutput()
}

case class DVICtrl() extends Component {
  val rgbConfig = RgbConfig(4, 4, 4)
  val io = new Bundle {
    val pinsA = master(DVI())
    val pinsB = master(DVI())

    val position = master(Flow(Coordinate(640, 480)))
    val frameStart = out(Bool)

    val pixels = slave(Stream(Rgb(rgbConfig)))
  }

  val ctrl = new VgaCtrl(rgbConfig)
  ctrl.io.softReset := False
  ctrl.io.timings.setAs_h640_v480_r60
  ctrl.io.pixels <> io.pixels
  ctrl.io.frameStart <> io.frameStart
  ctrl.io.vga.colorEn <> io.position.valid

  io.pinsB.pin2 := ClockDomain.current.readClockWire.asBits
  io.pinsB.pin9 := ctrl.io.vga.colorEn.asBits
  io.pinsB.pin4 := ctrl.io.vga.hSync.asBits
  io.pinsB.pin10 := ctrl.io.vga.vSync.asBits

  io.pinsA.pin8 := ctrl.io.vga.color.r(0).asBits
  io.pinsA.pin2 := ctrl.io.vga.color.r(1).asBits
  io.pinsA.pin7 := ctrl.io.vga.color.r(2).asBits
  io.pinsA.pin1 := ctrl.io.vga.color.r(3).asBits

  io.pinsA.pin10 := ctrl.io.vga.color.g(0).asBits
  io.pinsA.pin4 := ctrl.io.vga.color.g(1).asBits
  io.pinsA.pin9 := ctrl.io.vga.color.g(2).asBits
  io.pinsA.pin3 := ctrl.io.vga.color.g(3).asBits

  io.pinsB.pin3 := ctrl.io.vga.color.b(0).asBits
  io.pinsB.pin8 := ctrl.io.vga.color.b(1).asBits
  io.pinsB.pin7 := ctrl.io.vga.color.b(2).asBits
  io.pinsB.pin1 := ctrl.io.vga.color.b(3).asBits

  val x = Counter(640)
  val y = Counter(480)

  io.position.payload.x := x
  io.position.payload.y := y

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
}
