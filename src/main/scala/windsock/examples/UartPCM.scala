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

  val amp = new AudioAmpCtrl(8 bit)
  amp.io.pins <> io.pmod4
  amp.io.enable := True
  amp.io.gain := False

  val pcm = Stream(Bits(8 bits))
  amp.io.sample := U(pcm.toFlow.toReg())

  val pump = new UartPump(
    2 MHz,
    clockDomain.newSlowedClockDomain(8 kHz)
  )
  pump.io.uart <> io.uart
  pump.io.data <> pcm
}
