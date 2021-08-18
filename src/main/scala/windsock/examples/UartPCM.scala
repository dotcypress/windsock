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

  val amp = new AudioAmpCtrl(13 bit)
  amp.io.pins <> io.pmod4
  amp.io.enable := True
  amp.io.gain := False

  val g711 = new G711Expander()
  amp.io.sample := U(g711.io.expanded + 4096).resized

  val pcm = Stream(Bits(8 bits))
  g711.io.compressed := S(pcm.toFlow.toReg())

  val pump = new UartPump(
    2 MHz,
    clockDomain.newSlowedClockDomain(8 kHz)
  )
  pump.io.uart <> io.uart
  pump.io.data <> pcm
}

case class G711Expander() extends Component {
  val io = new Bundle {
    val compressed = in(SInt(8 bit))
    val expanded = out(SInt(13 bit))
  }

  val sign = io.compressed.sign
  val high = io.compressed(4, 3 bits).asBits
  val low = io.compressed(3 downto 0).asBits

  io.expanded := high.mux(
    B"000" -> S(sign ## B"0000000" ## low ## B"1"),
    B"001" -> S(sign ## B"0000001" ## low ## B"1"),
    B"010" -> S(sign ## B"000001" ## low ## B"10"),
    B"011" -> S(sign ## B"00001" ## low ## B"100"),
    B"100" -> S(sign ## B"0001" ## low ## B"1000"),
    B"101" -> S(sign ## B"001" ## low ## B"10000"),
    B"110" -> S(sign ## B"01" ## low ## B"100000"),
    B"111" -> S(sign ## B"1" ## low ## B"1000000")
  )
}
