fn add(a: usize, b: usize) -> usize {
    a + b // value is returned
}

fn main() {
    let a: usize = 17;
    let b = 3;
    let result = add(a, b);
    println!("Result: {}", result);
}