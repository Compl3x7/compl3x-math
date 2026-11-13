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

    const EXP_TAYLOR_SERIES_LEN: u32 = 21;

    pub fn exp(x: f64) -> f64 {
        if x < 0.0 {
            1.0 / exp(-x)
        } else {
            let floor_x = floor(x);
            pow_whole(f64::consts::E, floor_x as i64)
                * exp_taylor(x - floor_x, EXP_TAYLOR_SERIES_LEN)
        }
    }

    fn pow_whole(x: f64, n: i64) -> f64 {
        match n {
            0 => 1.0,
            _ => {
                let result = pow_whole(x * x, n / 2);
                if n % 2 == 0 { result } else { result * x }
            }
        }
    }

    fn exp_taylor(x: f64, series_len: u32) -> f64 {
        let mut result = 0.0;
        let mut exp = 1.0;
        let mut fact = 1.0;
        for i in 0..series_len {
            result += exp / fact;
            exp *= x;
            fact *= (i + 1) as f64
        }
        result
    }

    const LN_TAYLOR_SERIES_LEN: u32 = 20;

    pub fn ln(x: f64) -> f64 {
        let mut x = x;
        let mut result = 0.0;
        let mut dist = 1.0 - x;

        while abs(dist) >= 1.0 {
            x /= f64::consts::E;
            result += 1.0;
            dist = 1.0 - x;
        }

        // TODO: Improve dist calculation so that abs(dist) <= 0.1

        result + ln_taylor(dist, LN_TAYLOR_SERIES_LEN)
    }

    fn ln_taylor(x: f64, series_len: u32) -> f64 {
        let mut result = 0.0;
        let mut exp = x;
        for i in 1..=series_len {
            result += exp / i as f64;
            exp *= x;
        }
        -result
    }

    pub fn pow(x: f64, y: f64) -> f64 {
        exp(y * ln(x))
    }

    pub fn log(x: f64, y: f64) -> f64 {
        ln(x) / ln(y)
    }
}
