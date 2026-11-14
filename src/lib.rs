pub mod f64 {
    use core::f64;
    use std::cmp::Ordering;

    pub fn abs(x: f64) -> f64 {
        if x < 0.0 { -x } else { x }
    }

    pub fn signum(x: f64) -> f64 {
        if x < 0.0 { -1.0 } else { 1.0 }
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

    const EXP_TAYLOR_SERIES_LEN: u32 = 20;

    pub fn exp(x: f64) -> f64 {
        if x < 0.0 {
            1.0 / exp(-x)
        } else {
            let floor_x = floor(x);
            powi(f64::consts::E, floor_x as i64) * exp_taylor(x - floor_x, EXP_TAYLOR_SERIES_LEN)
        }
    }

    pub fn powi(x: f64, n: i64) -> f64 {
        match n.cmp(&0) {
            Ordering::Equal => 1.0,
            Ordering::Less => 1.0 / powi(x, -n),
            Ordering::Greater => {
                let result = powi(x * x, n / 2);
                if n % 2 == 0 { result } else { result * x }
            }
        }
    }

    fn exp_taylor(x: f64, series_len: u32) -> f64 {
        assert!((0.0..1.0).contains(&x));

        let mut result = 0.0;
        let mut term = 1.0;
        for i in 0..series_len {
            result += term;
            term *= x / (i + 1) as f64;
        }
        result
    }

    const REMAINDER_ABS: f64 = 10e-3;
    const NEWTON_ITER_MAX: i32 = 100;
    const LN_TAYLOR_SERIES_LEN: u32 = 20;

    pub fn ln(x: f64) -> f64 {
        assert!(x > 0.0);

        let mut guess = ln_approx(x);
        let mut remainder = 1.0 - x / exp(guess);

        let mut iter = 0;
        while abs(remainder) >= REMAINDER_ABS {
            if iter > NEWTON_ITER_MAX {
                break;
            }
            guess -= exp(guess) / x - 1.0;
            remainder = 1.0 - x / exp(guess);
            iter += 1;
        }

        guess + ln_taylor(remainder, LN_TAYLOR_SERIES_LEN)
    }

    fn ln_approx(x: f64) -> f64 {
        let mut x = x;
        let mut approx = 0.0;

        while x > f64::consts::E {
            x /= f64::consts::E;
            approx += 1.0;
        }

        approx
    }

    fn ln_taylor(x: f64, series_len: u32) -> f64 {
        assert!((-REMAINDER_ABS..REMAINDER_ABS).contains(&x));

        let mut result = 0.0;
        let mut pow = x;
        for i in 1..=series_len {
            result += pow / i as f64;
            pow *= x;
        }
        -result
    }

    pub fn powf(x: f64, y: f64) -> f64 {
        assert!(x > 0.0);

        exp(y * ln(x))
    }

    pub fn log(base: f64, x: f64) -> f64 {
        assert!(x > 0.0 && base > 0.0);

        ln(base) / ln(x)
    }

    pub fn root(base: f64, x: f64) -> f64 {
        assert!(base != 0.0 && x >= 0.0);

        if x == 0.0 { 0.0 } else { powf(x, 1.0 / base) }
    }
}
