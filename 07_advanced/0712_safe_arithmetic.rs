fn main() {
    let foo: u32 = 5;
    let bar: i32 = 6;
    // 以下算数计算混合了两种数据类型，需要显示转换，报错：
    // error[E0308]: mismatched types
    // error[E0277]: cannot subtract `i32` from `u32`
    let difference = foo - bar;
    println!("{}", difference);
}