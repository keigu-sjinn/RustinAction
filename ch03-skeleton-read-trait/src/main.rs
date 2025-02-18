#![allow(unused_variables)]

#[derive(Debug)]
struct File;

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

fn main() {
    let f = File;
    let mut buffer = Vec::<u8>::new();
    let n_bytes = f.read(&mut buffer).unwrap();

    println!("{} bytes read from {:?}", n_bytes, f);
}
