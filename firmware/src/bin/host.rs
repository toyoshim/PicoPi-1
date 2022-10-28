mod rim;
mod rim_spacewar;

use crate::rim::Rim;
use crate::rim_spacewar::RimSpacewar;

fn main() {
    let mut rim = RimSpacewar::new();
    let mut cm: Vec<u32> = vec![0 as u32; 8192];
    let pc = rim.bootstrap(&mut cm);
    println!("PC: {:06o}", pc);
}
