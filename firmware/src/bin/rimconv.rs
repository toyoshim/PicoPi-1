use std::io;

fn read<R: io::Read>(read: &mut R) -> Result<u32, io::Error> {
    let mut buffer = [0; 3];
    match read.read(&mut buffer) {
        Ok(size) => {
            if 3 != size {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            }
            return Ok(((buffer[0] as u32 & 0x3f) << 12)
                | ((buffer[1] as u32 & 0x3f) << 6)
                | ((buffer[2] as u32 & 0x3f) << 0));
        }
        Err(e) => return Err(e),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut i = 0;
    let mut seek = true;
    println!("let memory: Vec<i32> = vec![");
    loop {
        match read(&mut reader) {
            Ok(data) => {
                if seek && 0x00 == data {
                    continue;
                }
                seek = false;
                if i % 8 == 0 {
                    print!("  ");
                }
                print!("0x{:06x},", data);
                if i % 8 == 7 {
                    println!("");
                }
                i += 1;
            }
            Err(_e) => break,
        }
    }
    println!("0x000000];");
}
