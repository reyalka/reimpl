use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
struct Dual {
    real: f64,
    dual: f64,
}

impl Dual {
    fn new(real: f64) -> Self {
        return Self { real, dual: 1.0 };
    }

    fn exp(self) -> Self {
        Self {
            real: self.real.exp(),
            dual: self.dual * self.real.exp(),
        }
    }

    fn sin(self) -> Self {
        Self {
            real: self.real.sin(),
            dual: self.dual * self.real.cos(),
        }
    }

    fn cos(self) -> Self {
        Self {
            real: self.real.cos(),
            dual: -self.dual * self.real.sin(),
        }
    }

    fn ln(self) -> Self {
        Self {
            real: self.real.ln(),
            dual: self.dual / self.real,
        }
    }
}

impl From<f64> for Dual {
    fn from(value: f64) -> Self {
        Self {
            real: value,
            dual: 0.0,
        }
    }
}

impl Add for Dual {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            dual: self.dual + rhs.dual,
        }
    }
}

impl Sub for Dual {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            dual: self.dual - rhs.dual,
        }
    }
}

impl Neg for Dual {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            dual: -self.dual,
        }
    }
}

impl Mul for Dual {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // (a + bε)(c + dε) = ac + (ad + bc)ε
        Self {
            real: self.real * rhs.real,
            dual: (self.real * rhs.dual + self.dual * rhs.real),
        }
    }
}

impl Div for Dual {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // (a + bε) / (c + dε) = (a/c) + ((b*c - a*d)/c^2)ε
        Self {
            real: self.real / rhs.real,
            dual: (self.dual * rhs.real - self.real * rhs.dual) / (rhs.real * rhs.real),
        }
    }
}

impl Display for Dual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}ε", self.real, self.dual)
    }
}

fn diff<T>(f: T, val: f64) -> (f64, f64)
where
    T: Fn(Dual) -> Dual,
{
    let result = f(Dual::new(val));
    (result.real, result.dual)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_of_identify_function() {
        let (y, dy) = diff(|x| x, 3.0);

        assert_eq!(y, 3.0);
        assert_eq!(dy, 1.0);
    }

    #[test]
    fn diff_of_square_funciton() {
        let (y, dy) = diff(|x| x * x, 3.0);

        assert_eq!(y, 9.0);
        assert_eq!(dy, 6.0);
    }

    #[test]
    fn diff_of_cube_function() {
        let (y, dy) = diff(|x| x * x * x, 3.0);

        assert_eq!(y, 27.0);
        assert_eq!(dy, 27.0);
    }

    #[test]
    fn diff_of_double_function() {
        let (y, dy) = diff(|x| x + x, 3.0);

        assert_eq!(y, 6.0);
        assert_eq!(dy, 2.0);
    }

    #[test]
    fn test_of_linear_function() {
        let (y, dy) = diff(|x| x * Dual::from(2.0), 3.0);

        assert_eq!(y, 6.0);
        assert_eq!(dy, 2.0);
    }

    #[test]
    fn test_of_linear_function_reversed() {
        let (y, dy) = diff(|x| Dual::from(2.0) * x, 3.0);

        assert_eq!(y, 6.0);
        assert_eq!(dy, 2.0);
    }

    fn assert_approx_eq(a: f64, b: f64) {
        let tol = 1e-10;
        assert!(
            (a - b).abs() < tol,
            "Expected {} to be approximately equal to {}",
            a,
            b
        );
    }

    #[test]
    fn test_of_subtraction_function() {
        let (y, dy) = diff(|x| x - Dual::from(1.0), 3.0);
        assert_approx_eq(y, 2.0);
        assert_approx_eq(dy, 1.0);
    }

    #[test]
    fn test_of_exponential_function() {
        let (y, dy) = diff(|x| x.exp(), 2.0);
        assert_approx_eq(y, std::f64::consts::E.powf(2.0));
        assert_approx_eq(dy, std::f64::consts::E.powf(2.0));
    }

    #[test]
    fn test_of_exponential_function_with_multiplier() {
        let (y, dy) = diff(|x| Dual::from(2.0) * x.exp(), 1.0);
        assert_approx_eq(y, 2.0 * std::f64::consts::E);
        assert_approx_eq(dy, 2.0 * std::f64::consts::E);
    }

    #[test]
    // f(x) = exp(x^3 + x)
    // f'(x) = exp(x^3 + x) * (3x^2 + 1)
    fn test_of_composite_function() {
        let (y, dy) = diff(|x| (x * x * x + x).exp(), 2.0);
        let expected_y = (8.0 + 2.0 as f64).exp();
        let expected_dy = expected_y * (3.0 * 4.0 + 1.0); // exp(x^3 + x) * (3x^2 + 1) at x=2

        assert_approx_eq(y, expected_y);
        assert_approx_eq(dy, expected_dy);
    }

    #[test]
    fn test_of_negative_function() {
        let (y, dy) = diff(|x| -x, 3.0);
        assert_approx_eq(y, -3.0);
        assert_approx_eq(dy, -1.0);
    }

    #[test]
    fn test_of_negative_function_with_subtraction() {
        let (y, dy) = diff(|x| -x - Dual::from(2.0), 3.0);
        assert_approx_eq(y, -5.0);
        assert_approx_eq(dy, -1.0);
    }

    #[test]
    // f(x) = 1 / x
    // f'(x) = -1 / x^2
    fn test_of_diverse_function() {
        let (y, dy) = diff(|x| Dual::from(1.0) / x, 2.0);
        assert_approx_eq(y, 0.5);
        assert_approx_eq(dy, -0.25);
    }

    #[test]
    // f(x) = sin x
    // f'(x) = cos x
    fn test_of_sine_function() {
        let (y, dy) = diff(|x| (x.exp() - (-x).exp()) / Dual::from(2.0), 0.0);
        assert_approx_eq(y, 0.0);
        assert_approx_eq(dy, 1.0);
    }

    #[test]
    // f(x) = cos x
    // f'(x) = -sin x
    fn test_of_cosine_function() {
        let (y, dy) = diff(|x| (x.exp() + (-x).exp()) / Dual::from(2.0), 0.0);
        assert_approx_eq(y, 1.0);
        assert_approx_eq(dy, 0.0);
    }

    #[test]
    // f(x) = sin^2(x) + cos x
    // f'(x) = 2 sin(x) cos(x) - sin(x)
    fn test_of_combined_trigonometric_function() {
        let (y, dy) = diff(|x| x.sin() * x.sin() + x.cos(), std::f64::consts::PI);
        assert_approx_eq(y, -1.0);
        assert_approx_eq(dy, 0.0);
    }

    #[test]
    // f(x) = ln(x)
    // f'(x) = 1/x
    fn test_of_logarithm_function() {
        let (y, dy) = diff(|x| x.ln(), 2.0);
        assert_approx_eq(y, std::f64::consts::LN_2);
        assert_approx_eq(dy, 1.0 / 2.0);
    }
}
