struct Person(String);

#[allow(unreachable_patterns, unused_variables)]
fn main() {
    // // 以下变量 a 的初始化方式会报错，类型不匹配，需要转换为字符串
    // // error[E0308]: mismatched types - expected struct `String`, found `&str`
    // let a = Person("Richard Feynman");
    let a = Person("Richard Feynman".to_string());
    
    match a {
        // // 以下直接使用变量 name 会造成原变量 a 不再可用，后面赋值给 b 会报错
        // // error[E0382]: use of partially moved value: `a`
        // Person(name) => println!("{} was a great physicist!", name),
        
        // // 以下使用运算符 & 的引用方式也会报错，期待字符串结构但找到了引用
        // // expected struct `String`, found reference
        // Person(&name) => println!("{} was a great physicist!", name),

        // 正确的打开方式是使用 ref 来表示借用，记住在 match 中的这个 ref 用法
        Person(ref name) => println!("{} was a great physicist!", name),
        _ => panic!("Oh no!")
    }

    let b = a;
}