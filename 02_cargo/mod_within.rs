#![allow(dead_code, unused_variables)]
// import module with keyword use
use self::food::Cake;

mod food {
    // use keyword pub for accessible
    pub struct Cake;
    struct Smottie;
    struct Pizza;
}

fn main() {
    let eatable = Cake;
}