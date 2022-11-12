// Copyright 2022 Takashi Toyoshima <toyoshim@gmail.com>.
// Use of this source code is governed by a BSD-style license that can be found
// in the LICENSE file.
use pio;
use rp_pico::hal::pac::PIO0;
use rp_pico::hal::pio::{
    Buffers, PIOBuilder, PinDir, ShiftDirection, Tx, UninitStateMachine, PIO, SM0,
};

pub struct Display {
    tx: Tx<(PIO0, SM0)>,
}

impl Display {
    pub fn new(pio: &mut PIO<PIO0>, sm0: UninitStateMachine<(PIO0, SM0)>) -> Self {
        let mut code = pio::Assembler::<{ pio::RP2040_MAX_PROGRAM_SIZE }>::new();
        let mut loop_label = code.label();
        code.bind(&mut loop_label);
        code.pull(false, true);
        code.out(pio::OutDestination::PINS, 20 + 3);
        code.jmp(pio::JmpCondition::Always, &mut loop_label);
        let program = code.assemble_program();
        let installed = pio.install(&program).unwrap();
        let (mut sm, _, tx) = PIOBuilder::from_program(installed)
            .buffers(Buffers::OnlyTx)
            .out_pins(6, 20 + 3)
            .out_shift_direction(ShiftDirection::Right)
            .autopull(false)
            .clock_divisor(1.0)
            .build(sm0);
        sm.set_pindirs([
            (6, PinDir::Output),
            (7, PinDir::Output),
            (8, PinDir::Output),
            (9, PinDir::Output),
            (10, PinDir::Output),
            (11, PinDir::Output),
            (12, PinDir::Output),
            (13, PinDir::Output),
            (14, PinDir::Output),
            (15, PinDir::Output),
            (16, PinDir::Output),
            (17, PinDir::Output),
            (18, PinDir::Output),
            (19, PinDir::Output),
            (20, PinDir::Output),
            (21, PinDir::Output),
            (22, PinDir::Output),
            //
            (26, PinDir::Output),
            (27, PinDir::Output),
            (28, PinDir::Output),
        ]);
        sm.start();
        Display { tx }
    }

    pub fn set(&mut self, x: u16, y: u16) {
        let y_9_7: u32 = ((y as u32) << 7) & 0x700000;
        let y_6_0: u32 = ((y as u32) << 4) & 0x01fc00;
        let x_9_0: u32 = ((x as u32) >> 6) & 0x0003ff;
        self.tx.write(y_9_7 | y_6_0 | x_9_0);
    }
}
