fn main() {
    let big_endian: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
    let little_endia: [u8; 4] = [0xdd, 0xcc, 0xbb, 0xaa];

    let a: i32 = unsafe { std::mem::transmute(big_endian) };
    let b: i32 = unsafe { std::mem::transmute(little_endia) };

    println!("{} vs {}", a, b);
}
