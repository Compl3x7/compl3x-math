use core::f64;

fn main() {
    let x: f64 = f64::consts::E;
    println!("x             = {}", x);
    println!("|x|           = {}", compl3x_math::f64::abs(x));
    println!("floor(x)      = {}", compl3x_math::f64::floor(x));
    println!("ceil(x)       = {}", compl3x_math::f64::ceil(x));
    println!("round(x)      = {}", compl3x_math::f64::round(x));
    println!("e^x           = {}", compl3x_math::f64::exp(x));
    println!("Rust e^x      = {}\n", x.exp());

    let y: f64 = f64::consts::PI;
    println!("y             = {}", y);
    println!("ln(y)         = {}", compl3x_math::f64::ln(y));
    println!("Rust ln(y)    = {}", y.ln());
    println!("x^y           = {}", compl3x_math::f64::powf(x, y));
    println!("Rust x^y      = {}", x.powf(y));
    println!("logx(y)       = {}", compl3x_math::f64::log(x, y));
    println!("Rust logx(y)  = {}", x.log(y))
}
