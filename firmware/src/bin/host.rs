use heapless::Vec;
use pico_pi_1::Rim;
use pico_pi_1::RimSpacewar;

fn main() {
    let mut rim = RimSpacewar::new();
    let mut cm: Vec<u32, 8192> = Vec::new();
    if let Ok(_) = cm.resize_default(8192) {
        let pc = rim.bootstrap(&mut cm);
        println!("PC: {:06o}", pc);
    }
}
