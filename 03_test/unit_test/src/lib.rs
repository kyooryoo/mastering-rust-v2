#![allow(dead_code)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn sum_input_output() -> Vec<((i8, i8), i8)> {
        vec![((1,1),2),((0,0),0),((2,-2),0)]
    }

    #[test]
    fn test_sum() {
        for (input, output) in sum_input_output() {
            assert_eq!(crate::sum(input.0, input.1),output);
        }
    }

    #[test]
    #[should_panic]
    fn this_panics() {
        assert_eq!(1,2);
    }

    #[test]
    #[ignore]
    pub fn test_silly_loop() {
        crate::silly_loop();
    }
}

// the function to be tested
fn sum(a: i8, b: i8) -> i8 {
    a + b
}

pub fn silly_loop() {
    for _ in 1..1_000_000_000 {};
}