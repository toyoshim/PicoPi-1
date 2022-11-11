// Copyright 2022 Takashi Toyoshima <toyoshim@gmail.com>.
// Use of this source code is governed by a BSD-style license that can be found
// in the LICENSE file.
use pico_pi_1::CoreMemory;
use pico_pi_1::Pdp1;
use pico_pi_1::Rim;
use pico_pi_1::RimSpacewar;

fn main() {
    let mut rim = RimSpacewar::new();
    let mut cm = CoreMemory::new();
    let pc = rim.bootstrap(&mut cm);
    let mut cpu = Pdp1::new(&mut cm, &mut rim, pc);
    loop {
        cpu.step();
    }
}
