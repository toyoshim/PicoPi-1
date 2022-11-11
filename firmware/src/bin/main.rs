// Copyright 2022 Takashi Toyoshima <toyoshim@gmail.com>.
// Use of this source code is governed by a BSD-style license that can be found
// in the LICENSE file.
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;
use rp_pico::entry;

use pico_pi_1::CoreMemory;
use pico_pi_1::Display;
use pico_pi_1::Pdp1;
use pico_pi_1::Rim;
use pico_pi_1::RimSpacewar;

use rp_pico::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::pin::FunctionPio0,
    pac,
    pio::PIOExt,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut _delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();
    led_pin.set_high().unwrap();

    // TODO: move to display.rs.
    pins.gpio6.into_mode::<FunctionPio0>();
    pins.gpio7.into_mode::<FunctionPio0>();
    pins.gpio8.into_mode::<FunctionPio0>();
    pins.gpio9.into_mode::<FunctionPio0>();
    pins.gpio10.into_mode::<FunctionPio0>();
    pins.gpio11.into_mode::<FunctionPio0>();
    pins.gpio12.into_mode::<FunctionPio0>();
    pins.gpio13.into_mode::<FunctionPio0>();
    pins.gpio14.into_mode::<FunctionPio0>();
    pins.gpio15.into_mode::<FunctionPio0>();
    pins.gpio16.into_mode::<FunctionPio0>();
    pins.gpio17.into_mode::<FunctionPio0>();
    pins.gpio18.into_mode::<FunctionPio0>();
    pins.gpio19.into_mode::<FunctionPio0>();
    pins.gpio20.into_mode::<FunctionPio0>();
    pins.gpio21.into_mode::<FunctionPio0>();
    pins.gpio22.into_mode::<FunctionPio0>();
    //
    pins.gpio26.into_mode::<FunctionPio0>();
    pins.gpio27.into_mode::<FunctionPio0>();
    pins.gpio28.into_mode::<FunctionPio0>();

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut display = Display::new(&mut pio, sm0);
    let mut rim = RimSpacewar::new();
    let mut cm = CoreMemory::new();
    let pc = rim.bootstrap(&mut cm);
    let mut cpu = Pdp1::new(&mut cm, &mut rim, &mut display, pc);
    loop {
        cpu.step();
    }
}

// End of file
