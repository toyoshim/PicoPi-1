use pico_pi_1::CoreMemory;
use pico_pi_1::Pdp1;
use pico_pi_1::Rim;
use pico_pi_1::RimSpacewar;

fn main() {
    let mut rim = RimSpacewar::new();
    let mut cm = CoreMemory::new();
    let pc = rim.bootstrap(&mut cm);
    println!("PC: {:06o}", pc);
    let mut cpu = Pdp1::new(&mut cm, &mut rim, pc);
    cpu.step();
}
