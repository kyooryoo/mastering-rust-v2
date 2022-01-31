fn main() {
    let result = if 1 == 2 {
        "Something is wrong" // return string
    } else {
        "Rust makes sense" // return string
    };

    println!("You know what? {}.", result);
}