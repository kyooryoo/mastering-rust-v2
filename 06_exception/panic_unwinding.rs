// RUST_BACKTRACE=full ./panic_unwinding

use std::thread;

fn alice() -> thread::JoinHandle<()> {
    // generate a new thread
    thread::spawn(move || {
        bob();
    })
}

fn bob() {
    malice();
}

fn malice() {
    panic!("malice is panicking!");
}

fn main() {
    let child = alice();
    let _ = child.join();

    bob();
    println!("this is unreachable code");
}