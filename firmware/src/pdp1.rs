use crate::CoreMemory;
use crate::Rim;

pub struct Pdp1<'a> {
    pc: u16,
    ir: u16,
    y: u16,
    inst: u16,
    ind: u16,
    io: u16,
    xct: u16,
    ac: u16,
    ov: u16,
    dump: u16,
    test: u16,
    pf: u16,
    cm: &'a mut CoreMemory,
    rim: &'a mut dyn Rim,
}

impl<'a> Pdp1<'a> {
    pub fn new(cm: &'a mut CoreMemory, rim: &'a mut dyn Rim, pc: u16) -> Self {
        Pdp1 {
            pc: pc,
            ir: 0,
            y: 0,
            inst: 0,
            ind: 0,
            io: 0,
            xct: 0,
            ac: 0,
            ov: 0,
            dump: 0,
            test: 0,
            pf: 0,
            cm: cm,
            rim: rim,
        }
    }

    pub fn step(&mut self) {}

    /*
    pub fn trace(&mut self) {
        println!(
            "PC: {:06o}, IO: {:06o}, AC: {:06o}, OV: {}, I: {}, M[Y]: {:06o}, PF: {:06o}",
            self.pc,
            self.io,
            self.ac,
            self.ov,
            (self.cm.read(self.pc) >> 12) & 1,
            self.cm.read(self.y),
            self.pf
        );
    }
    */
}
