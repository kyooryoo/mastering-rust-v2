use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    // two ways to create Ok
    let _my_result1: Result<_, ()> = Ok(64);
    let _my_result2 = Ok::<_,()>(64);

    // two ways to create Err
    let _my_err1 = Err::<(), f32>(345.3);
    let _my_err2: Result<bool, String> = Err("Wait, what?".to_string());

    let path = Path::new("data.txt");
    // open file will change internal pointer
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => { panic!("Error while opening file: {}", err); }
    };
    let mut s = String::new();
    // ignore the possible returned error
    let _ = file.read_to_string(&mut s);
    println!("Message: {}", s);
}
