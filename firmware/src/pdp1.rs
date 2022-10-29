use crate::CoreMemory;
use crate::Rim;

pub struct Pdp1<'a> {
    t: u32,  // 5usec tick
    io: u32, // 18bit
    ac: u32, // 18bit
    pc: u16, // 12bit
    y: u16,  // 12bit
    pf: u8,  // 3bit
    xct: bool,
    ov: bool,
    cm: &'a mut CoreMemory,
    rim: &'a mut dyn Rim,
}

impl<'a> Pdp1<'a> {
    pub fn new(cm: &'a mut CoreMemory, rim: &'a mut dyn Rim, pc: u16) -> Self {
        Pdp1 {
            t: 0,
            io: 0,
            ac: 0,
            pc: pc,
            y: 0,
            pf: 0,
            xct: false,
            ov: false,
            cm: cm,
            rim: rim,
        }
    }

    pub fn step(&mut self) {
        let ir;
        if !self.xct {
            self.trace();
            ir = self.cm.read(self.pc);
            self.pc += 1;
        } else {
            ir = self.cm.read(self.y);
            self.xct = false;
        }
        let inst = ((ir >> 12) & 0x3e) as u8;
        self.y = (ir & 0x0fff) as u16;
        self.t += 1;
        match inst {
            0o002 => {
                // 002 AND (10 usec)
                self.get_operand(ir);
                self.ac &= self.cm.read(self.y);
            }
            0o004 => {
                // 004 IOR (10 usec)
                self.get_operand(ir);
                self.ac |= self.cm.read(self.y);
            }
            0o006 => {
                // 006 XOR (10 usec)
                self.get_operand(ir);
                self.ac ^= self.cm.read(self.y);
            }
            0o010 => {
                // 010 XCT (5 usec)
                self.get_operand(ir);
                self.xct = true;
            }
            0o016 => {
                if 0 == (ir & (1 << 12)) {
                    // 016 CAL (10 usec)
                    // == JDA 0100
                    self.cm.write(0x040, self.ac);
                    self.ac = self.pc as u32;
                    if self.ov {
                        self.ac += 1 << 17;
                    }
                    self.pc = 0x41;
                } else {
                    // 017 JDA (10 usec)
                    // == DAC Y + JSP Y+1
                    self.cm.write(self.y, self.ac);
                    self.ac = self.pc as u32;
                    if self.ov {
                        self.ac += 1 << 17;
                    }
                    self.pc = self.y + 1;
                }
            }
            0o020 => {
                // 020 LAC (10 usec)
                self.get_operand(ir);
                self.ac = self.cm.read(self.y);
            }
            0o022 => {
                // 022 LIO (10 usec)
                self.get_operand(ir);
                self.io = self.cm.read(self.y);
            }
            0o024 => {
                // 024 DAC (10 usec)
                self.get_operand(ir);
                self.cm.write(self.y, self.ac);
            }
            0o026 => {
                // 026 DAP (15 usec)
                self.get_operand(ir);
                let cm = self.cm.read(self.y);
                self.cm.write(self.y, (cm & 0x3f000) | (self.ac & 0x00fff));
            }
            0o032 => {
                // 032 DIO (10 usec)
                self.get_operand(ir);
                self.cm.write(self.y, self.io);
            }
            0o034 => {
                // 034 DZM (10 usec)
                self.get_operand(ir);
                self.cm.write(self.y, 0);
            }
            0o040 => {
                // 040 ADD (10 usec)
                self.get_operand(ir);
                let ac = self.ac;
                let cm = self.cm.read(self.y);
                self.ac += cm;
                if self.ac > 0x3ffff {
                    self.ac = (self.ac + 1) & 0x3ffff;
                }
                if 0 != ((cm ^ !ac) & (self.ac ^ ac) & 0x20000) {
                    self.ov = true;
                }
                if 0x3ffff == self.ac {
                    self.ac = 0;
                }
            }
            0o042 => {
                // 042 SUB (10 usec)
                self.get_operand(ir);
                let ac = self.ac ^ 0x3ffff;
                let cm = self.cm.read(self.y);
                self.ac = ac + cm;
                if self.ac > 0x3ffff {
                    self.ac = (self.ac + 1) & 0x3ffff;
                }
                if 0 != ((cm ^ !ac) & (self.ac ^ ac) & 0x20000) {
                    self.ov = true;
                }
                self.ac ^= 0x3ffff;
            }
            0o044 => {
                // 044 IDX (10 usec)
                self.get_operand(ir);
                self.ac = self.cm.read(self.y) + 1;
                if 0x3ffff <= self.ac {
                    self.ac = (self.ac + 1) & 0x3ffff;
                }
                self.cm.write(self.y, self.ac);
            }
            0o046 => {
                // 046 ISP (10 usec)
                self.get_operand(ir);
                self.ac = self.cm.read(self.y) + 1;
                if 0x3ffff <= self.ac {
                    self.ac = (self.ac + 1) & 0x3ffff;
                }
                self.cm.write(self.y, self.ac);
                if 0 == self.ac & 0x20000 {
                    self.pc += 1;
                }
            }
            0o050 => {
                // 050 SAD (10 usec)
                self.get_operand(ir);
                if self.cm.read(self.y) != self.ac {
                    self.pc += 1;
                }
            }
            0o052 => {
                // 052 SAS (10 usec)
                self.get_operand(ir);
                if self.cm.read(self.y) == self.ac {
                    self.pc += 1
                }
            }
            0o054 => {
                // 054 MUS (10 usec)
                self.get_operand(ir);
                if 0 != (self.io & 1) {
                    self.ac += self.cm.read(self.y);
                    self.ac += self.ac >> 18;
                    self.ac &= 0x3ffff;
                    if 0x3ffff == self.ac {
                        self.ac = 0;
                    }
                }
                self.io = ((self.io >> 1) | (self.ac << 17)) & 0x3ffff;
                self.ac >>= 1;
            }
            0o056 => {
                // 056 DIV (10 usec)
                self.get_operand(ir);
                let t = self.ac >> 17;
                self.ac = ((self.ac << 1) | (self.io >> 17)) & 0x3ffff;
                self.io = ((self.io << 1) | (t ^ 1)) & 0x3ffff;
                let cm = self.cm.read(self.y);
                if 0 != self.io & 1 {
                    self.ac += cm ^ 0x3ffff;
                } else {
                    self.ac += cm + 1;
                }
                if self.ac >= 0x3ffff {
                    self.ac = (self.ac + 1) & 0x3ffff;
                }
            }
            0o060 => {
                // 060 JMP (5 usec)
                self.get_operand(ir);
                self.pc = self.y;
            }
            0o062 => {
                // 062 JSP (5 usec)
                self.get_operand(ir);
                self.ac = self.pc as u32;
                if self.ov {
                    self.ac |= 1 << 17;
                }
                self.pc = self.y;
            }
            0o064 => {
                // 064 SKP
                // TODO
            }
            0o066 => {
                // 066 SFT (5 usec)
                self.sft(ir);
            }
            0o070 => {
                // 070 LAW (5 usec)
                if 0 == (ir & (1 << 12)) {
                    self.ac = self.y as u32;
                } else {
                    self.ac = self.y as u32 ^ 0x3ffff;
                }
            }
            0o072 => {
                // 072 IOT (5 usec + I/O wait)
                self.iot();
            }
            0o076 => {
                // 076 OPR (5 usec)
                self.opr();
            }
            _ => {
                panic!("Illegal Instruction: {:03o}", inst);
            }
        }
    }

    fn get_operand(&mut self, ir: u32) {
        let mut ind = (ir & (1 << 12)) != 0;
        while ind {
            let ir = self.cm.read(self.y);
            self.t += 1;
            self.y = (ir & 0x0fff) as u16;
            ind = (ir & (1 << 12)) != 0;
        }
    }

    fn opr(&mut self) {
        let cmd = self.y;
        if 0 != cmd & 0x080 {
            // CLA
            self.ac = 0;
        }
        if 0 != cmd & 0x100 {
            // HLT
            panic!("Halt");
        }
        if 0 != cmd & 0x200 {
            // CMA
            self.ac ^= 0x3ffff;
        }
        if 0 != cmd & 0x400 {
            // LAT
            // TODO: Load Accumulator from Test Word.
        }
        if 0 != cmd & 0x800 {
            // CLI
            self.io = 0;
        }
        if 0 != cmd & 0x007 {
            static FLAGS: [u8; 8] = [0x00, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01, 0x3f];
            let mask = FLAGS[(cmd & 7) as usize];
            if 0 == cmd & 0x008 {
                // CLF
                self.pf &= mask;
            } else {
                // STF
                self.pf |= mask;
            }
        }
        if 0 != cmd & 0x070 {
            panic!("Illegal Operation: {:04o}", cmd);
        }
    }

    fn sft(&mut self, ir: u32) {
        let mode: u8 = ((ir as u16 & (1 << 12) | self.y) >> 9) as u8;
        let mut popc: u8 = 0;
        let mut y = self.y & 0x1ff;
        for _ in 0..9 {
            if 0 != (y & 1) {
                popc += 1;
            }
            y >>= 1;
        }
        match mode {
            0o01 => {
                // 01 RAL
                self.ac = ((self.ac << popc) | (self.ac >> (18 - popc))) & 0x3ffff;
            }
            0o02 => {
                // 02 RIL
                self.io = ((self.io << popc) | (self.io >> (18 - popc))) & 0x3ffff;
            }
            0o03 => {
                // 03 RCL
                let t = ((self.ac << popc) | (self.io >> (18 - popc))) & 0x3ffff;
                self.io = ((self.io << popc) | (self.ac >> (18 - popc))) & 0x3ffff;
                self.ac = t;
            }
            0o05 => {
                // 05 SAL
                let t;
                if 0 != (self.ac & 0x20000) {
                    t = 0x3ffff;
                } else {
                    t = 0;
                }
                self.ac = (self.ac & 0x20000) | ((self.ac << popc) & 0x1ffff) | (t >> (18 - popc));
            }
            0o06 => {
                // 06 SIL
                let t;
                if 0 != (self.io & 0x20000) {
                    t = 0x3ffff;
                } else {
                    t = 0;
                }
                self.io = (self.io & 0x20000) | ((self.io << popc) & 0x1ffff) | (t >> (18 - popc));
            }
            0o07 => {
                // 07 SCL
                let t;
                if 0 != (self.ac & 0x20000) {
                    t = 0x3ffff;
                } else {
                    t = 0;
                }
                self.ac =
                    (self.ac & 0x20000) | ((self.ac << popc) & 0x1ffff) | (self.io >> (18 - popc));
                self.io = ((self.io << popc) & 0x3ffff) | (t >> (18 - popc));
            }
            0o11 => {
                // 11 RAR
                self.ac = ((self.ac >> popc) | (self.ac << (18 - popc))) & 0x3ffff;
            }
            0o12 => {
                // 12 RIR
                self.io = ((self.io >> popc) | (self.io << (18 - popc))) & 0x3ffff;
            }
            0o13 => {
                // 13 RCR
                let t = ((self.ac >> popc) | (self.io << (18 - popc))) & 0x3ffff;
                self.io = ((self.io >> popc) | (self.ac << (18 - popc))) & 0x3ffff;
                self.ac = t;
            }
            0o15 => {
                // 15 SAR
                let t;
                if 0 != (self.ac & 0x20000) {
                    t = 0x3ffff;
                } else {
                    t = 0;
                }
                self.ac = ((self.ac >> popc) | (t << (18 - popc))) & 0x3ffff;
            }
            0o16 => {
                // 16 SIR
                let t;
                if 0 != (self.ac & 0x20000) {
                    t = 0x3ffff;
                } else {
                    t = 0;
                }
                self.io = ((self.io >> popc) | (t << (18 - popc))) & 0x3ffff;
            }
            0o17 => {
                // 17 SCR
                let t;
                if 0 != (self.ac & 0x20000) {
                    t = 0x3ffff;
                } else {
                    t = 0;
                }
                self.io = ((self.io >> popc) | (self.ac << (18 - popc))) & 0x3ffff;
                self.ac = ((self.ac >> popc) | (t << (18 - popc))) & 0x3ffff;
            }
            _ => {
                panic!("Unknown Shift: {:03o}", mode);
            }
        }
    }

    fn iot(&mut self) {
        let target = self.y & 0x3f;
        match target {
            0o000 => {}
            0o002 => {
                self.io = self.rim.next();
            }
            0o007 => {
                // TODO: Display
            }
            0o011 => {
                // TODO: Spacewar Controllers
            }
            _ => {}
        }
    }

    fn trace(&mut self) {
        /*
         println!(
             "PC: {:06o}, IO: {:06o}, AC: {:06o}, OV: {}, I: {:03o}, M[Y]: {:06o}, PF: {:06o}",
             self.pc,
             self.io,
             self.ac,
             self.ov,
             (self.cm.read(self.pc) >> 12) & 0x3f, //1,
             self.cm.read(self.y),
             self.pf
         );
        */
    }
}
