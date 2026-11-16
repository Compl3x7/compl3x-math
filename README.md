# A not at all complex math library: compl3x-math

Despite its name, it is neither complex nor does it support complex numbers. 

```Rust
fn main() {
    let x = f64::consts::PI;
    println!("x             = {}", x);
    println!("e^x           = {}", compl3x_math::f64::exp(x));
    println!("ln(x)         = {}", compl3x_math::f64::ln(x));
    println!("sqrt(x)       = {}", compl3x_math::f64::root(2.0, x));
    println!("sin(x)        = {}", compl3x_math::f64::sin(x));
    println!("cos(x)        = {}", compl3x_math::f64::cos(x));
    println!("tan(x)        = {}", compl3x_math::f64::tan(x));
    
    let y = f64::consts::E;
    println!("y             = {}", y);    
    println!("x^y           = {}", compl3x_math::f64::powf(x, y));
    println!("logx(y)       = {}", compl3x_math::f64::log(x, y));
}
```

## Overview

A math library written without resorting to any standard library math functions, only using provided constants such as `f64::consts::E` or `f64::consts::PI`, otherwise clippy would complain. Project built with the intent of getting started with the Rust programming language.

## Features

Currently it supports the following functions for the `f64` data type:

- Exponential, $f(x) = e^x$, `exp(x)`
- Natural logarithm, $f(x) = ln(x)$, `ln(x)`
- Exponents, $f(x, y) = x^y$, `powf(x, y)`
- Logarithms of a given base `base`, $f(b, x) = log_{b}(x)$, `log(base, x)`
- Roots of a given base `base`, $f(b, x) = \sqrt[b]{a}$, `root(base, x)`
- The trignometric trio: $\sin(x),\ \cos(x),\ \tan(x)$, `sin(x)`, `cos(x)`, `tan(x)`

## AI Usage

In a personal effort to reduce my AI usage, AI tools were strictly forbbiden to be used in this project.

##

Still here :3