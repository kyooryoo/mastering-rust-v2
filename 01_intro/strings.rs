fn main() {
    let questions = "How are you?"; // &str type
    let person: String = "Bob".to_string();
    let youkoso = String::from("ようこそ"); // unicode is supported
    println!("{}! {} {}", youkoso, questions, person);
}