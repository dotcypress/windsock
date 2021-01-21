package windsock.core

import scala.collection.mutable.ArrayBuffer

import vexriscv.plugin._
import vexriscv._
import vexriscv.ip.{DataCacheConfig, InstructionCacheConfig}
import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba3.apb._
import spinal.lib.bus.amba4.axi._
import spinal.lib.com.jtag.Jtag
import spinal.lib.com.uart._
import spinal.lib.graphic.vga.{Axi4VgaCtrl, Axi4VgaCtrlGenerics, Vga}
import spinal.lib.graphic.Rgb
import spinal.lib.io.TriStateArray
import spinal.lib.misc.HexTools
import spinal.lib.system.debugger._

import windsock.core.mmio.{Apb3SystemCtrl, Apb3TimerCtrl, Apb3RGBCtrl}
import windsock.lib._

class Core(config: CoreConfig) extends Component {
  val io = new Bundle {
    val asyncReset = in(Bool())
    val gpio = master(TriStateArray(config.gpioWidth bits))
    val uart = master(Uart())
    val colors = slave(Flow(Rgb(8,8,8)))
    val jtag = if (config.enableDebug) slave(Jtag()) else null
    val panic = out(Bool())
  }

  val resetCtrlClockDomain = ClockDomain(
    clock = clockDomain.readClockWire,
    config = ClockDomainConfig(
      resetKind = BOOT
    )
  )

  val resetCtrl = new ClockingArea(resetCtrlClockDomain) {
    val systemResetUnbuffered = False
    val systemResetCounter = Reg(UInt(6 bits)) init (0)
    when(systemResetCounter =/= U(systemResetCounter.range -> true)) {
      systemResetCounter := systemResetCounter + 1
      systemResetUnbuffered := True
    }
    when(BufferCC(io.asyncReset)) {
      systemResetCounter := 0
    }

    val systemReset = RegNext(systemResetUnbuffered)
    val axiReset = RegNext(systemResetUnbuffered)
  }

  val axiClockDomain = ClockDomain(
    clock = clockDomain.readClockWire,
    reset = resetCtrl.axiReset,
    frequency = clockDomain.frequency
  )

  axiClockDomain.setSyncWith(clockDomain)

  val axi = new ClockingArea(axiClockDomain) {
    val externalInterrupt = False

    val ram = Axi4SharedOnChipRam(
      dataWidth = 32,
      byteCount = config.onChipRamSize,
      idWidth = 4
    )

    if (config.onChipRamHexFile != null) {
      HexTools.initRam(ram.ram, config.onChipRamHexFile, 0x80000000L)
    }

    val apbBridge = Axi4SharedToApb3Bridge(
      addressWidth = 20,
      dataWidth = 32,
      idWidth = 4
    )

    if (config.enableDebug) {
      val debugClockDomain = ClockDomain(
        clock = clockDomain.readClockWire,
        reset = resetCtrl.systemReset,
        frequency = clockDomain.frequency
      )

      val debugPlugin = new DebugPlugin(debugClockDomain)
      debugClockDomain {
        resetCtrl.axiReset setWhen (RegNext(debugPlugin.io.resetOut))
        io.jtag <> debugPlugin.io.bus.fromJtag()
      }
      config.plugins += debugPlugin
    }

    val core = new Area {
      val cpu = new VexRiscv(
        VexRiscvConfig(
          plugins = config.plugins
        )
      )
      var iBus: Axi4ReadOnly = null
      var dBus: Axi4Shared = null
      for (plugin <- config.plugins) plugin match {
        case plugin: IBusSimplePlugin => iBus = plugin.iBus.toAxi4ReadOnly()
        case plugin: IBusCachedPlugin => iBus = plugin.iBus.toAxi4ReadOnly()
        case plugin: DBusSimplePlugin => dBus = plugin.dBus.toAxi4Shared()
        case plugin: DBusCachedPlugin => dBus = plugin.dBus.toAxi4Shared(true)
        case plugin: CsrPlugin => {
          plugin.timerInterrupt := False
          plugin.externalInterrupt := BufferCC(externalInterrupt)
        }
        case _ =>
      }
    }

    val axiCrossbar = Axi4CrossbarFactory()

    axiCrossbar.addSlaves(
      ram.io.axi -> (0x80000000L, config.onChipRamSize),
      apbBridge.io.axi -> (0xf0000000L, 1 MB)
    )

    axiCrossbar.addConnections(
      core.iBus -> List(ram.io.axi),
      core.dBus -> List(ram.io.axi, apbBridge.io.axi)
    )

    axiCrossbar.addPipelining(apbBridge.io.axi)((crossbar, bridge) => {
      crossbar.sharedCmd.halfPipe() >> bridge.sharedCmd
      crossbar.writeData.halfPipe() >> bridge.writeData
      crossbar.writeRsp << bridge.writeRsp
      crossbar.readRsp << bridge.readRsp
    })

    axiCrossbar.addPipelining(ram.io.axi)((crossbar, ctrl) => {
      crossbar.sharedCmd.halfPipe() >> ctrl.sharedCmd
      crossbar.writeData >/-> ctrl.writeData
      crossbar.writeRsp << ctrl.writeRsp
      crossbar.readRsp << ctrl.readRsp
    })

    axiCrossbar.addPipelining(core.dBus)((cpu, crossbar) => {
      cpu.sharedCmd >> crossbar.sharedCmd
      cpu.writeData >> crossbar.writeData
      cpu.writeRsp << crossbar.writeRsp
      cpu.readRsp <-< crossbar.readRsp
    })

    axiCrossbar.build()

    val sysCtrl = Apb3SystemCtrl()
    sysCtrl.io.panic <> io.panic

    val gpioCtrl = Apb3Gpio(
      gpioWidth = config.gpioWidth,
      withReadSync = true
    )

    val uartCtrl = Apb3UartCtrl(config.uartConfig)
    externalInterrupt setWhen (uartCtrl.io.interrupt)

    val timerCtrl = Apb3TimerCtrl()
    externalInterrupt setWhen (timerCtrl.io.interrupt)

    val rgbCtrl = Apb3RGBCtrl()

    val apbDecoder = Apb3Decoder(
      master = apbBridge.io.apb,
      slaves = List(
        sysCtrl.io.apb -> (0x00000, 4 kB),
        gpioCtrl.io.apb -> (0x10000, 4 kB),
        uartCtrl.io.apb -> (0x20000, 4 kB),
        timerCtrl.io.apb -> (0x30000, 4 kB),
        rgbCtrl.io.apb -> (0x40000, 4 kB)
      )
    )
  }

  io.gpio <> axi.gpioCtrl.io.gpio
  io.uart <> axi.uartCtrl.io.uart
  io.colors <> axi.rgbCtrl.io.colors
}
