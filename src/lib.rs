pub fn abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

pub fn floor(x: f64) -> i64 {
    let xi64 = x as i64;
    let diff = x - xi64 as f64;
    if diff < 0.0 { xi64 - 1 } else { xi64 }
}

pub fn ceil(x: f64) -> i64 {
    let floor_x = floor(x);
    if x == floor_x as f64 { floor_x } else { floor_x + 1 }
}

pub fn round(x: f64) -> i64 {
    let floor_x = floor(x);
    let diff = abs(x - floor_x as f64);
    if diff < 0.5 { floor_x } else { ceil(x) }
}