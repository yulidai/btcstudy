// result < max
pub fn check_range_add(a: usize, b: usize, max: usize) -> Result<usize, &'static str> {
    let r = a + b;
    if r < max {
        Ok(r)
    } else {
        Err("result is out of range within math::check_range_add")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn math_check_range_add_1() {
        let result = super::check_range_add(8, 8, 17);
        assert_eq!(result, Ok(16));
    }

    #[test]
    fn math_check_range_add_2() {
        let result = super::check_range_add(8, 8, 16);
        assert!(result.is_err());
    }
}
