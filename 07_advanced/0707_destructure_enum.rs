#[allow(dead_code)]

enum Container {
    Item(u64),
    Empty
}

fn main() {
    let maybe_item = Container::Item(0u64);
    // 使用以下方法有条件的将值分配给变量 has_item
    // if let <destructure pattern> = expression {} 
    let has_item = if let Container::Item(0) = maybe_item { true } else { false };
    println!("has item: {}", has_item);
}