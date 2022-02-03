// Bar module and item are imported
mod Bar;
pub use self::bar::Bar;

pub fn do_foo() {
    println!("Hi from foo!");
}