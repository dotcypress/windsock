package windsock.lib

import spinal.core._
import spinal.lib._

case class RgbLed() extends Bundle {
  val r = Bool()
  val g = Bool()
  val b = Bool()
}
