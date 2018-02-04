
debug:
	xargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/msp432-newio upload/msp432.debug.bin

release:
	xargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/msp432-newio upload/msp432.release.bin

clean:
	xargo clean
	rm upload/*
