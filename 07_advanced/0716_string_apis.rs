#[allow(unused_mut, unused_variables)]

fn main() {
    // 以下四种方法创建了同样的字符串类型，具有相同性能特征
    let a: String = "hello".to_string(); // 调用to_string()方法
    let b = String::from("hello"); // 调用from()方法
    let c = "hello".to_owned(); // 调用to_owned特征方法
    let d = c.clone(); // 复制现有字符串，因开销昂贵应尽量避免

    // 以下演示String在标准库中的一些便捷方法
    let mut empty_string = String::new();
    // 分配一个预定义大小的空字符串，在预先知道字符串大小时十分高效
    let empty_string_with_capacity = String::with_capacity(50);
    // 尝试从bytestring分配一个新的String类型，参数内容必须是UTF-8，返回Result的包装器类型
    let string_from_bytestring: String = String::from_utf8(vec![82,85,83,84]).expect("Create String from bytestring failed");

    println!("Length of the empty string is {}", empty_string.len()); // 返回字符串长度
    println!("Length of the empty string with capacity is {}", empty_string_with_capacity.len());
    println!("Length of the string from a bytestring is {}", string_from_bytestring.len());

    println!("Bytestring says {}", string_from_bytestring);

    empty_string.push('1'); // 添加字符到字符串
    println!("1) Empty string now contains {}", empty_string);
    empty_string.push_str("2345"); // 添加字符串到字符串
    println!("2) Empty string now contains {}", empty_string);
    println!("Length of the previously empty string is now {}", empty_string.len());
}