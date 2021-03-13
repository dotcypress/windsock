package windsock.lib.pmod

import spinal.core._
import spinal.lib._

case class AudioAmp(lowConnector: Boolean = true) extends PMODBundle {
  override def asMaster() = {
    val value = if (lowConnector) {
      out(pin7, pin8, pin10)
      in(pin1, pin2, pin3, pin4, pin9)
    } else {
      out(pin1, pin2, pin4)
      in(pin3, pin7, pin8, pin9, pin10)
    }
  }
}

case class AudioAmpCtrl(wordWidth: BitCount = 8 bit, lowConnector: Boolean = true) extends Component {
  val io = new Bundle {
    val pins = master(AudioAmp(lowConnector))
    val sample = in(UInt(wordWidth))
    val enable = in(Bool())
    val gain = in(Bool())
  }

  val pdm = PDM(wordWidth)
  pdm.io.input <> io.sample

  if (lowConnector) {
    io.pins.pin7 := B(pdm.io.output)
    io.pins.pin8 := ~B(io.gain)
    io.pins.pin10 := B(io.enable)
  } else {
    io.pins.pin1 := B(pdm.io.output)
    io.pins.pin2 := ~B(io.gain)
    io.pins.pin4 := B(io.enable)
  }
}

case class PDM(width: BitCount) extends Component {
  val io = new Bundle {
    val input = in(UInt(width))
    val output = out(Bool())
  }

  val max = UInt(width).setAll()
  val acc = Reg(UInt(width.value + 1 bit))
  acc := acc + io.input

  io.output := False
  when(acc >= max) {
    acc := acc - max
    io.output := True
  }
}
