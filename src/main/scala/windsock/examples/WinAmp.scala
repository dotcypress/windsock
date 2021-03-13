package windsock.examples

import java.io.FileInputStream
import spinal.core._
import spinal.lib._
import spinal.lib.misc._
import windsock.lib.pmod._
import windsock.bsp._

object WinAmp {
  def main(args: Array[String]) = ECPIX5.generate(new WinAmp)
}

case class WinAmp() extends Component {
  val io = new Bundle {
    val pmod4 = master(AudioAmp())
    val pmod7 = master(SnapOff())
  }

  val snapOff = new SnapOffCtrl
  snapOff.io.pins <> io.pmod7

  val amp = new AudioAmpCtrl()
  amp.io.pins <> io.pmod4
  amp.io.enable := snapOff.io.button1
  amp.io.gain := snapOff.io.button2

  val soundPath = "src/main/resources/winamp.32khz.pcm"
  val sampleRate = 32 kHz

  val ram = Mem(
    Bits(8 bits),
    wordCount = new FileInputStream(soundPath).available()
  )
  BinTools.initRam(ram, soundPath)

  new SlowArea(sampleRate) {
    val sample = ram.readSync(
      enable = True,
      address = CounterFreeRun(ram.wordCount)
    )

    amp.io.sample := U(sample)

    snapOff.io.led1 := sample(7) & sample(5)
    snapOff.io.led2 := sample(7) & sample(4)
    snapOff.io.led3 := sample(7) & sample(3)
    snapOff.io.led4 := sample(7) & sample(2)
    snapOff.io.led5 := sample(7) & sample(1)
  }
}
