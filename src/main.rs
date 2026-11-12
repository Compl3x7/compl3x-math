fn main() {
    let x: f64 = 3.1415;
    println!("x = {}", x);
    println!("|x| = {}", compl3x_math::abs(x));
    println!("floor(x) = {}", compl3x_math::floor(x));
    println!("ceil(x) = {}", compl3x_math::ceil(x));
    println!("round(x) = {}", compl3x_math::round(x));
}
