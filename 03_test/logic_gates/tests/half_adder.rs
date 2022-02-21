#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411926-200.png")]
use logic_gates::{and, xor};

pub type Sum = u8;
pub type Carry = u8;

pub fn half_adder_input_output() -> Vec<((u8, u8), (Sum, Carry))> {
    vec![
        ((0, 0), (0, 0)),
        ((0, 1), (1, 0)),
        ((1, 0), (1, 0)),
        ((1, 1), (0, 1)),
    ]
}

/// 该函数使用原始的逻辑门实现了一个半加法器
fn half_adder(a: u8, b: u8) -> (Sum, Carry) {
    (xor(a, b), and(a, b))
}

#[test]
fn one_bit_adder() {
    for (inn, out) in half_adder_input_output() {
        let (a, b) = inn;
        println!("Testing: {}, {} -> {:?}", a, b, out);
        assert_eq!(half_adder(a, b), out);
    }
}