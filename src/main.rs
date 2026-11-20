use core::f64;

fn main() {
    let x = f64::consts::E;
    println!("x             = {}", x);
    println!("|x|           = {}", compl3x_math::f64::abs(x));
    println!("floor(x)      = {}", compl3x_math::f64::floor(x));
    println!("ceil(x)       = {}", compl3x_math::f64::ceil(x));
    println!("round(x)      = {}", compl3x_math::f64::round(x));
    println!("e^x           = {}", compl3x_math::f64::exp(x));
    println!("Rust e^x      = {}\n", x.exp());

    let x = f64::consts::PI;
    println!("x             = {}", x);
    println!("ln(x)         = {}", compl3x_math::f64::ln(x));
    println!("Rust ln(x)    = {}", x.ln());
    println!("sqrt(x)       = {}", compl3x_math::f64::root(2.0, x));
    println!("Rust sqrt(x)  = {}\n", x.sqrt());

    let y = f64::consts::E;
    println!("x             = {}", x);
    println!("y             = {}", y);
    println!("x^y           = {}", compl3x_math::f64::powf(x, y));
    println!("Rust x^y      = {}", x.powf(y));
    println!("logx(y)       = {}", compl3x_math::f64::log(x, y));
    println!("Rust logx(y)  = {}\n", x.log(y));

    let x = 0.3 + 2.0 * f64::consts::PI;
    println!("x             = {}", x);
    println!("Rust sin(x)   = {}", x.sin());
    println!("cos(x)        = {}", compl3x_math::f64::cos(x));
    println!("Rust cos(x)   = {}", x.cos());
    println!("tan(x)        = {}", compl3x_math::f64::tan(x));
    println!("Rust tan(x)   = {}\n", x.tan());

    let x = 0.5;
    println!("x                 = {}", x);
    println!("arcsin(x)         = {}", compl3x_math::f64::arcsin(x));
    println!("Rust arcsin(x)    = {}", x.asin());
    println!("arccos(x)         = {}", compl3x_math::f64::arccos(x));
    println!("Rust arccos(x)    = {}", x.acos());
    println!("arctan(x)         = {}", compl3x_math::f64::arctan(x));
    println!("Rust arctan(x)    = {}", x.atan()); 
}
