// for exposing to external usage:
// module item and function are pub

pub struct Bar;

impl Bar {
    pub fn hello() {
        println!("Hello from Bar!");
    }
}