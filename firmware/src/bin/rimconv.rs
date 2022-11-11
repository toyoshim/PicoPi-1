// Copyright 2022 Takashi Toyoshima <toyoshim@gmail.com>.
// Use of this source code is governed by a BSD-style license that can be found
// in the LICENSE file.
use std::io;

fn read<R: io::Read>(read: &mut R) -> Result<u32, io::Error> {
    let mut head = [0; 1];
    loop {
        match read.read(&mut head) {
            Ok(size) => {
                if 1 != size {
                    return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
                }
                if 0 != (head[0] & 0x80) {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }
    let mut body = [0; 2];
    match read.read(&mut body) {
        Ok(size) => {
            if 2 != size {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            }
            return Ok(((head[0] as u32 & 0x3f) << 12)
                | ((body[0] as u32 & 0x3f) << 6)
                | ((body[1] as u32 & 0x3f) << 0));
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
    println!("// counts: {}", i + 1);
}
