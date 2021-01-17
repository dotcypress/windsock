package windsock.core

import scala.collection.mutable.ArrayBuffer
import spinal.core._
import spinal.lib._
import spinal.lib.com.uart._
import vexriscv._
import vexriscv.plugin._
import vexriscv.ip.InstructionCacheConfig
import vexriscv.ip.DataCacheConfig

case class CoreConfig(
    onChipRamHexFile: String,
    onChipRamSize: BigInt,
    gpioWidth: Int,
    enableDebug: Boolean,
    plugins: ArrayBuffer[Plugin[VexRiscv]],
    uartConfig: UartCtrlMemoryMappedConfig
)

object CoreConfig {
  def withRamFile(onChipRamHexFile: String) =
    CoreConfig
      .default()
      .copy(
        onChipRamHexFile = onChipRamHexFile
      )

  def default() =
    CoreConfig(
      enableDebug = false,
      onChipRamHexFile = null,
      onChipRamSize = 32 kB,
      gpioWidth = 32,
      plugins = ArrayBuffer(
        new PcManagerSimplePlugin(0x00000000L, false),
        new IBusCachedPlugin(
          resetVector = 0x80000000L,
          prediction = NONE,
          compressedGen = true,
          injectorStage = true,
          config = InstructionCacheConfig(
            cacheSize = 4096,
            bytePerLine = 32,
            wayCount = 1,
            addressWidth = 32,
            cpuDataWidth = 32,
            memDataWidth = 32,
            catchIllegalAccess = true,
            catchAccessFault = true,
            asyncTagMemory = false,
            twoCycleRam = false,
            twoCycleCache = false
          )
        ),
        new DBusCachedPlugin(
          config = new DataCacheConfig(
            cacheSize = 4096,
            bytePerLine = 32,
            wayCount = 1,
            addressWidth = 32,
            cpuDataWidth = 32,
            memDataWidth = 32,
            catchAccessError = true,
            catchIllegal = true,
            catchUnaligned = true
          )
        ),
        new StaticMemoryTranslatorPlugin(ioRange = _(31 downto 28) === 0xf),
        new CsrPlugin(
          config = CsrPluginConfig(
            mvendorid = null,
            marchid = null,
            mimpid = null,
            mhartid = 0,
            mtvecInit = 0x80000020L,
            misaExtensionsInit = 66,
            ecallGen = true,
            wfiGenAsWait = true,
            mscratchGen = false,
            catchIllegalAccess = true,
            misaAccess = CsrAccess.READ_WRITE,
            mtvecAccess = CsrAccess.READ_WRITE,
            mepcAccess = CsrAccess.READ_WRITE,
            minstretAccess = CsrAccess.READ_WRITE,
            mcycleAccess = CsrAccess.READ_ONLY,
            mcauseAccess = CsrAccess.READ_ONLY,
            mbadaddrAccess = CsrAccess.READ_ONLY,
            ucycleAccess = CsrAccess.READ_ONLY,
            uinstretAccess = CsrAccess.READ_ONLY
          )
        ),
        new DecoderSimplePlugin(),
        new RegFilePlugin(regFileReadyKind = plugin.SYNC),
        new IntAluPlugin,
        new SrcPlugin(
          separatedAddSub = false,
          executeInsertion = true
        ),
        new LightShifterPlugin,
        new MulPlugin,
        new DivPlugin,
        new HazardSimplePlugin(
          bypassExecute = true,
          bypassMemory = true,
          bypassWriteBack = true,
          bypassWriteBackBuffer = true
        ),
        new BranchPlugin(earlyBranch = true),
        new YamlPlugin("cpu0.yaml")
      ),
      uartConfig = UartCtrlMemoryMappedConfig(
        uartCtrlConfig = UartCtrlGenerics(
          dataWidthMax = 8,
          clockDividerWidth = 20,
          preSamplingSize = 1,
          samplingSize = 3,
          postSamplingSize = 1
        ),
        initConfig = UartCtrlInitConfig(
          baudrate = 115200,
          dataLength = 7,
          parity = UartParityType.NONE,
          stop = UartStopType.ONE
        ),
        txFifoDepth = 16,
        rxFifoDepth = 16
      )
    )
}
