// an example of returning unexpected empty result
// because of ; the specified string is not returned

fn main() {
    let result = if 1 == 2 {
        "Nothing makes sense"; // return ()
    } else {
        "Sanity reigns"; // return ()
    };
    println!("Result of computation: {:?}", result);
}