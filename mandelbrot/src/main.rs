fn main() {
    println!("Hello, world!");
}

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    hight: usize
) -> Vec<Vec<usize>> {
    let mut rows = Vec::with_capacity(width);
    rows
}
