package windsock.examples

import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._
import windsock.lib.UartPump
import windsock.lib.pmod._
import windsock.lib.UartSink
import windsock.bsp._

object Mic {
  def main(args: Array[String]) = ECPIX5.generate(new Mic)
}

case class Mic() extends Component {
  val io = new Bundle {
    val uart = master(Uart())
    val pmod2 = master(PDMMic())
    val pmod4 = master(AudioAmp())
  }

  val amp = new AudioAmpCtrl(8 bit)
  amp.io.pins <> io.pmod4
  amp.io.enable := True
  amp.io.gain := False

  val audio = new SlowArea(5 MHz) {
    val clock = CounterFreeRun(2).willOverflow
  }

  val wordWidth = 16 bit

  val mic = new PDMMicCtrl(wordWidth)
  mic.io.pins <> io.pmod2
  mic.io.audioClock := audio.clock

  val sink = UartSink(2 MHz, wordWidth)
  sink.io.uart <> io.uart
  sink.io.data <> mic.io.pcm.toStream.transmuteWith(Bits(wordWidth))
}
