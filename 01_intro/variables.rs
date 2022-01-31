fn main() {
    // variables are immutable by default
    // use mut keyword if they may change
    let mut target = "world";
    let mut greeting = "Hello";
    println!("{}, {}!", greeting, target);
    greeting = "How are you doing";
    target = "mate";
    println!("{}, {}!", greeting, target);
}