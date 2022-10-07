fn main() {
    let mut foo = 1234;
    let bar = foo;
    println!("{:?} {:?}", foo, bar);
    foo = 5678;
    println!("{:?} {:?}", foo, bar);
}