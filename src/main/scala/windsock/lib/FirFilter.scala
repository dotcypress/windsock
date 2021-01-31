package windsock.lib

import spinal.core._
import spinal.lib._
import spinal.lib.experimental.math._

class FirFilter(peak: ExpNumber, width: BitCount, taps: Seq[Double])
    extends Component {
  val io = new Bundle {
    val source = slave(Flow(SFix(peak, width)))
    val result = out(SFix(peak, width))
  }

  val history = History(
    io.source.payload,
    taps.length,
    io.source.valid
  )

  val mac = SFix(peak, width)
  mac := Vec
    .tabulate(taps.length)(i => {
      val tap = SFix(peak, width)
      tap := taps(i)
      history(i) * tap
    })
    .reduce(_ + _)
    .truncated

  io.result := RegNext(mac)
}
