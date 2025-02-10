fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("\t0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("\t      0.3: {:x}", (abc.2).to_bits());

    println!("xyz (f64)");
    println!("\t0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("\t      0.3: {:x}", (xyz.2).to_bits());

    let x = (-42_f32).sqrt();
    assert_eq!(x, x);
    assert!((1.0_f32 / 0.0).is_finite());
}
