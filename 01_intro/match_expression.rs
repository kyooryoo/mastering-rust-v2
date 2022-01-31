fn req_status() -> usize {
    200
}

fn main() {
    let status = req_status();
    // each branch of match must return the same type
    // match must cover all possible returned values
    match status {
        200 => println!("Success"),
        404 => println!("Not Found"),
        other => {
            println!("Request failed with code: {}", other);
        }
    }
}