use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
struct Dual {
    real: f64,
    dual: f64,
}

impl Dual {
    fn new(real: f64) -> Self {
        return Self { real, dual: 1.0 };
    }
}

impl From<f64> for Dual {
    fn from(value: f64) -> Self {
        Self { real: value, dual: 0.0 }
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
}
