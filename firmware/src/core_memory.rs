use heapless::Vec;

pub struct CoreMemory {
    data: Vec<u32, 8192>,
}

impl CoreMemory {
    pub fn new() -> Self {
        let mut data = Vec::new();
        if let Ok(_) = data.resize_default(8192) {
            CoreMemory { data: data }
        } else {
            panic!("NoMem");
        }
    }

    pub fn read(&self, addr: u16) -> u32 {
        return self.data[addr as usize];
    }

    pub fn write(&mut self, addr: u16, data: u32) {
        self.data[addr as usize] = data;
    }
}
