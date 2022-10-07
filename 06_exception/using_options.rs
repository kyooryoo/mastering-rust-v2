use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let incremented_value1 = match map.get("one") {
        Some(val) => val + 1,
        None => 0
    };
    println!("{}", incremented_value1);

    // 
    let incremented_value2 = if let Some(v) = map.get("two") {
        v + 1
    } else {
        0
    };
    println!("{}", incremented_value2);

    // not safe
    let incremented_value3 = map.get("three").unwrap() + 1;
    println!("{}", incremented_value3);
}