// items in tuple could be various data types

fn main() {
    let num_and_str: (u8, &str) = (40, "You are not old!");
    println!("{:?}", num_and_str);
    // destruct a tuple to variables
    let (num, string) = num_and_str;
    println!("From tuple - Number: {}, String: {}", num, string);
}