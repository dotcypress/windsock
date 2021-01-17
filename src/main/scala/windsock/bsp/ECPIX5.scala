package windsock.bsp

import java.nio.file._
import spinal.core._
import spinal.lib._

object ECPIX5 {
  def generate[T <: Component](
      gen: => T,
      defaultClockDomainFrequency: HertzNumber = 26 MHz
  ) {
    val targetDirectory = Paths.get("target/bitstream")
    if (!Files.exists(targetDirectory)) {
      Files.createDirectory(targetDirectory)
    }
    new SpinalConfig(
      defaultClockDomainFrequency = FixedFrequency(defaultClockDomainFrequency),
      defaultConfigForClockDomains = ClockDomainConfig(
        resetKind = ASYNC,
        resetActiveLevel = LOW
      ),
      targetDirectory = targetDirectory.toString()
    ).generateVerilog(gen)
  }
}
