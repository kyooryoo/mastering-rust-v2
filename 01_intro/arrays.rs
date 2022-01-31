// arrays in Rust have unchangable size
// can only have items in one data type

fn main() {
    // specify size and type on the left of =
    let numbers: [u8; 12] = [0,1,2,3,4,5,6,7,8,9,10,11];
    // specify data, size, and type on the right of =
    let floats = [0.1f64, 0.2, 0.3];

    println!("Numbers: {}", numbers[5]);
    println!("Float: {}", floats[2]);
}