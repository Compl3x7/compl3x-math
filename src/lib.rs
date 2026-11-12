pub mod f64 {
    use core::f64;

    pub fn abs(x: f64) -> f64 {
        if x < 0.0 { -x } else { x }
    }

    pub fn floor(x: f64) -> f64 {
        let x_truncated = x as i64 as f64;
        let diff = x - x_truncated;
        if diff < 0.0 {
            x_truncated - 1.0
        } else {
            x_truncated
        }
    }

    pub fn ceil(x: f64) -> f64 {
        let floor_x = floor(x);
        if x == floor_x { floor_x } else { floor_x + 1.0 }
    }

    pub fn round(x: f64) -> f64 {
        let floor_x = floor(x);
        let diff = x - floor_x;
        if diff < 0.5 { floor_x } else { ceil(x) }
    }

    pub fn exp(x: f64) -> f64 {
        if x < 0.0 {
            1.0 / exp(-x)
        } else {
            let floor_x = floor(x);
            exp_whole(floor_x) * exp_frac(x - floor_x as f64)
        }
    }

    fn exp_whole(x: f64) -> f64 {
        pow_whole(f64::consts::E, x)
    }

    fn pow_whole(x: f64, n: f64) -> f64 {
        match n {
            0.0 => 1.0,
            _ => {
                let result = pow_whole(x * x, n / 2.0);
                if n as i64 % 2 == 0 {
                    result
                } else {
                    result * x
                }
            }
        }
    }

    const TAYLOR_SERIES_LEN: i32 = 10;
    fn exp_frac(x: f64) -> f64 {
        let mut result = 0.0;
        let mut exp = 1.0;
        let mut fact = 1.0;
        for i in 0..=TAYLOR_SERIES_LEN {
            result += exp / fact;
            exp *= x;
            fact *= (i + 1) as f64
        }
        result
    }
}
