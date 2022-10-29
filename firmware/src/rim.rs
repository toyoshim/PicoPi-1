use crate::CoreMemory;

pub trait Rim {
    fn next(&mut self) -> u32;

    fn bootstrap(&mut self, cm: &mut CoreMemory) -> u16 {
        loop {
            let data: u32 = self.next();
            let ins: u8 = (data >> 12) as u8;
            let adr: u16 = (data & 0xfff) as u16;
            if 0o00 == ins {
                continue;
            } else if 0o60 == ins {
                return adr;
            } else if 0o32 == ins {
                cm.write(adr, self.next());
            }
        }
    }
}
