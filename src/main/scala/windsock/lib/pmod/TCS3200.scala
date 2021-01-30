package windsock.lib.pmod

import spinal.core._
import spinal.lib._
import spinal.lib.fsm._
import spinal.lib.graphic.Rgb
import windsock.lib._

case class TCS3200() extends PMODBundle {
  override def asMaster() = {
    in(pin2, pin3, pin4)
    out(pin1, pin7, pin8, pin9, pin10)
  }
}

object Channel extends SpinalEnum {
  val luma, red, green, blue = newElement()
}

case class TCS3200Ctrl(
    width: BitCount = 8 bits,
    measurePeriod: TimeNumber = 100 ms
) extends Component {
  val io = new Bundle {
    val pins = master(TCS3200())
    val colors = master(Flow(Rgb(width.value, width.value, width.value)))
    val luma = master(Flow(UInt(width)))
  }

  val fsm = new StateMachine {
    val measure = new State() with EntryPoint
    val store = new State()

    val channel = Reg(Channel) init (Channel.luma)
    val luma, red, green, blue = Reg(UInt(width)) init (0)
    val pulseCounter = Counter(16 bit)
    val measureTimer = Timeout(measurePeriod)

    val s0, s1 = True
    val s2 = channel === Channel.luma || channel === Channel.green
    val s3 = channel === Channel.blue || channel === Channel.green

    io.luma.payload := luma
    io.colors.payload.r := red
    io.colors.payload.g := green
    io.colors.payload.b := blue
    io.colors.valid := False

    val vco = io.pins.pin4.as(Bool)
    io.pins.pin7 := B(0)
    io.pins.pin8 := B(s0)
    io.pins.pin1 := B(s1)
    io.pins.pin10 := B(s2)
    io.pins.pin9 := B(s3)

    measure.whenIsActive {
      when(vco.rise()) {
        pulseCounter.increment()
      }

      when(measureTimer) {
        measureTimer.clear()
        pulseCounter.clear()

        switch(channel) {
          is(Channel.luma) {
            luma := (pulseCounter / 8).resized
            channel := Channel.red
            goto(measure)
          }
          is(Channel.red) {
            red := (pulseCounter / 8).resized
            channel := Channel.green
            goto(measure)
          }
          is(Channel.green) {
            green := (pulseCounter / 8).resized
            channel := Channel.blue
            goto(measure)
          }
          is(Channel.blue) {
            blue := (pulseCounter / 8).resized
            channel := Channel.luma
            goto(store)
          }
        }
      }
    }

    store.whenIsActive {
      io.colors.valid := True
      goto(measure)
    }
  }
}
