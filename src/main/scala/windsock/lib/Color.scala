package windsock.lib

import spinal.core._

case class Color(width: BitCount) extends Bundle {
  val luma, red, green, blue = UInt(width)
}