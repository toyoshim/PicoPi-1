setup: rp2040.svd openocd/src/openocd

rp2040.svd:
	curl -o $@ https://raw.githubusercontent.com/raspberrypi/pico-sdk/master/src/rp2040/hardware_regs/rp2040.svd

openocd/src/openocd:
	cd openocd && \
		./bootstrap && \
		./configure --enable-picoprobe --disable-werror && \
		make -j4