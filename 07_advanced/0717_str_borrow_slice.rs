// 注意默认的字符串返回类型就是如下的&'static str
fn get_str_literal() -> &'static str {
    "from function"
}

// 如下函数可以接受多种形式的字符串输入
fn say_hello(to_whom: &str) {
    println!("Hello {}!", to_whom);
}

fn main() {
    // // 以下声明会出错，因为不定长的str类不可以直接初始化
    // let message: str = "Wait, but why?";

    let my_str = "from borrowed";
    let from_fun = get_str_literal();
    println!("{} {}", my_str, from_fun);

    let hello = String::from("hello");
    
    // // 如下打开方式是错误的：`String` cannot be indexed by `{integer}`
    // let first_char = hello[0];
    
    // 获取字符串片段 
    let first_three = &hello[0..3];
    println!("{:?}", first_three);

    // // 注意索引越界的问题，编译期不会报错，但会发生运行时错误
    // let first_ten = &hello[0..10];
    // println!("{:?}", first_ten);

    // 可以的话，使用迭代器可以避免索引越界错误
    for c in hello.chars() {
        println!("{}", c);
    }

    // 如果需要传递字符串给函数，使用字符串切片&str
    let string_slice: &'static str = "you"; // 实际为&str类型
    let string: String = string_slice.into();
    say_hello(string_slice);
    say_hello(&string); // &String自动转换为了&str

    // 不可直接拼接字符串类型的变量
    let foo = "Foo";
    let bar = "Bar";
    // let aaz = foo + bar; // 这样不可以
    // 因为以上变量实际为引用，需要转换为带所有权的字符类型
    let baz = foo.to_owned() + bar; // 这样可以
    let caz = foo.to_string() + bar; // 这样也可以
    println!("{} {}", baz, caz);
}