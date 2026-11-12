fn main() {
    let x: f64 = std::f64::consts::PI;
    println!("x = {}", x);
    println!("|x| = {}", compl3x_math::f64::abs(x));
    println!("floor(x) = {}", compl3x_math::f64::floor(x));
    println!("ceil(x) = {}", compl3x_math::f64::ceil(x));
    println!("round(x) = {}", compl3x_math::f64::round(x));
    println!("e^x = {}", compl3x_math::f64::exp(x));
}
