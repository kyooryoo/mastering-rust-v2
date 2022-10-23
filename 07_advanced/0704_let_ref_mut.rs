#[derive(Debug)]
struct Items(u32);

fn main() {
    let items = Items(2);
    let items_ptr = &items; // 使用普通的运算符 & 创建引用
    let ref items_ref = items; // 使用关键字 ref 创建相同引用

    // 转换为 *const Items 即 Items 的原始指针类型是为了比较两个指针的内存地址
    assert_eq!(items_ptr as *const Items, items_ref as *const Items);

    let mut a = Items(20);
    // 也可以像这样使用可变引用
    let ref mut b = a; // 等同于 let b = &mut a;
    b.0 += 25;

    println!("{:?}", items);
    println!("{:?}", a);
}