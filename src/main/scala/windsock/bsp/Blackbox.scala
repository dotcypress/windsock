package windsock.bsp

import spinal.core._
import spinal.lib._

case class InternalOscillator(divisor: Bits) extends BlackBox {
  setDefinitionName("OSCG")
  addGeneric("DIV", divisor)

  val clockOut = out(Bool).setName("OSC")
}

case class PllClockConfig(
    val divisor: Bits,
    val cPhase: Bits = B(8, 10 bit),
    val fPhase: Bits = B(0, 10 bit)
) {}

case class PLLConfig(
    val refClockDivisor: Bits,
    val feedbackDivisor: Bits,
    val primaryClockConfig: PllClockConfig,
    val secondaryClockConfig: PllClockConfig = null,
    val feedbackPath: String = "CLKOP"
) {
  def applyTo(bb: BlackBox): Unit = {
    bb.addGeneric("PLLRST_ENA", "DISABLED")
    bb.addGeneric("INTFB_WAKE", "DISABLED")
    bb.addGeneric("STDBY_ENABLE", "DISABLED")
    bb.addGeneric("DPHASE_SOURCE", "DISABLED")
    bb.addGeneric("OUTDIVIDER_MUXA", "DIVA")
    bb.addGeneric("OUTDIVIDER_MUXB", "DIVB")
    bb.addGeneric("OUTDIVIDER_MUXC", "DIVC")
    bb.addGeneric("OUTDIVIDER_MUXD", "DIVD")
    bb.addGeneric("CLKI_DIV", refClockDivisor)
    bb.addGeneric("CLKFB_DIV", feedbackDivisor)
    bb.addGeneric("FEEDBK_PATH", feedbackPath)

    bb.addGeneric("CLKOP_ENABLE", "ENABLED")
    bb.addGeneric("CLKOP_DIV", primaryClockConfig.divisor)
    bb.addGeneric("CLKOP_CPHASE", primaryClockConfig.cPhase)
    bb.addGeneric("CLKOP_FPHASE", primaryClockConfig.fPhase)

    if (secondaryClockConfig != null) {
      bb.addGeneric("CLKOS_ENABLE", "ENABLED")
      bb.addGeneric("CLKOS_DIV", secondaryClockConfig.divisor)
      bb.addGeneric("CLKOS_CPHASE", secondaryClockConfig.cPhase)
      bb.addGeneric("CLKOS_FPHASE", secondaryClockConfig.fPhase)
    }
  }
}

case class PLL(cfg: PLLConfig) extends Component {
  val io = new Bundle {
    val clockIn = in(Bool)
    val feedbackClockIn = in(Bool)
    val locked = out(Bool)
    val primaryClockOut = out(Bool)
    val secondaryClockOut = if (cfg.secondaryClockConfig != null) {
      out(Bool)
    } else {
      null
    }
  }

  val pll = EHXPLLL(cfg)
  pll.feedbackClockIn <> pll.primaryClockOut
  pll.clockIn <> io.clockIn
  pll.locked <> io.locked
  pll.primaryClockOut <> io.primaryClockOut
  if (cfg.secondaryClockConfig != null) {
    pll.secondaryClockOut <> io.secondaryClockOut
  }
}

case class EHXPLLL(cfg: PLLConfig) extends BlackBox {
  setDefinitionName("EHXPLLL")
  cfg.applyTo(this)

  val clockIn = in(Bool).setName("CLKI")
  val feedbackClockIn = in(Bool).setName("CLKFB")
  val locked = out(Bool).setName("LOCK")
  val primaryClockOut = out(Bool).setName("CLKOP")
  val secondaryClockOut = out(Bool).setName("CLKOS")
}
