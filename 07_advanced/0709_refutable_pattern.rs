#[allow(dead_code)]
enum Container {
    Item(u64),
    Empty
}

fn main() {
    // 因为解构体中除了 Item 还有一个 Empty 所以用 let 匹配结果是不明确的
    // error[E0005]: refutable pattern in local binding: `Empty` not covered
    // https://doc.rust-lang.org/book/ch18-02-refutability.html
    // let Container::Item(it) = Container::Item(56);

    // 使用如下 if let 方法可以忽略没有匹配的变体
    if let Container::Item(it) = Container::Item(56) {
        println!("it: {}", it);
    }
}