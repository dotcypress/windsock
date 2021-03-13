MODULE = Serial
BUILD_DIR = target/bitstream
CONSTRAINTS = src/main/scala/windsock/bsp/ecpix5.lpf

all: elaborate bitstream

elaborate: 
	sbt --supershell=never "runMain windsock.examples.$(MODULE)"

bitstream: 
	cd $(BUILD_DIR) && \
	yosys -q -p 'synth_ecp5 -top $(MODULE) -json $(MODULE).json' $(MODULE).v && \
	nextpnr-ecp5 --um5g-85k --speed 8 --package CABGA554 --json $(MODULE).json --textcfg $(MODULE).config --lpf ../../$(CONSTRAINTS) && \
	ecppack --compress --input $(MODULE).config --bit $(MODULE).bit

app:
	cd src/main/rust/windsock-app && \
	cargo build --release && \
	riscv32-unknown-elf-objcopy -O ihex -S target/riscv32imc-unknown-none-elf/release/windsock-app ../../resources/ram.hex

prog:
	ecpprog -S $(BUILD_DIR)/$(MODULE).bit

flash:
	ecpprog $(BUILD_DIR)/$(MODULE).bit

clean:
	sbt clean --supershell=never
	cd src/main/rust/windsock-app && cargo clean
	cd src/main/rust/windsock-hal && cargo clean
	cd src/main/rust/windsock-pac && cargo clean
	rm -rf $(BUILD_DIR)

.SECONDARY:
.PHONY: all bitstream build clean elaborate flash prog app
