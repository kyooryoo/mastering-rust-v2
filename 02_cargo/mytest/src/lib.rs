#![allow(dead_code)]

pub fn pow(base: i64, exponent: usize) -> i64 {
    // // macro marks unimplemented code block
    // unimplemented!();

    let mut res = 1;
    if exponent == 0 {
        return 1;
    } else {
        for _ in 0..exponent {
            res *= base as i64;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    // // default out-of-the-box test
    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }

    use super::pow;
    #[test]
    fn minus_two_raised_three_in_minus_eight() {
        assert_eq!(pow(-2, 3), -8);
    }
}
