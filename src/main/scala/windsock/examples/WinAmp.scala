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

  val soundPath = "src/main/resources/winamp.raw"
  val sampleRate = 20800 Hz

  val ram = Mem(
    Bits(8 bits),
    wordCount = new FileInputStream(soundPath).available()
  )
  BinTools.initRam(ram, soundPath)

  val track = new SlowArea(sampleRate) {
    val frame = ram.readSync(
      enable = True,
      address = Counter(ram.wordCount, snapOff.io.button1)
    )

    snapOff.io.led1 := frame(7) & frame(5)
    snapOff.io.led2 := frame(7) & frame(4)
    snapOff.io.led3 := frame(7) & frame(3)
    snapOff.io.led4 := frame(7) & frame(2)
    snapOff.io.led5 := frame(7) & frame(1)

    amp.io.sample := U(frame)
  }
}
