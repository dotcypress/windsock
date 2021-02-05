package windsock.examples

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._
import windsock.lib.UartPump
import windsock.lib.pmod._
import windsock.bsp._

object UartPCM {
  def main(args: Array[String]) = ECPIX5.generate(new UartPCM)
}

case class UartPCM() extends Component {
  val io = new Bundle {
    val uart = master(Uart())
    val pmod4 = master(AudioAmp())
  }

  val amp = new AudioAmpCtrl()
  amp.io.pins <> io.pmod4
  amp.io.enable := True
  amp.io.gain := False

  val playback = new SlowArea(32 kHz)
  val pump = new UartPump(playback.clockDomain)
  pump.io.uart <> io.uart

  val sample = pump.io.data.toFlow.toReg()
  amp.io.sample := U(sample)
}
