use std::string::FromUtf8Error;

fn str_upper_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = match String::from_utf8(str) {
        Ok(str) => str.to_uppercase(),
        Err(err) => return Err(err)
    };

    println!("Conversion successded: {}", ret);
    Ok(ret)
}

fn main() {
    let valid_str = str_upper_match(vec![121, 97, 89]);
    println!("{:?}", valid_str);

    let invalid_str = str_upper_match(vec![197, 198]);
    println!("{:?}", invalid_str);
}