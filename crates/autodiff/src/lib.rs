pub fn diff<T>(f: T, val: f64) -> (f64, f64)
where
    T: Fn(f64) -> f64,
{
    return (val, 1.0);
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
}
