fn main() {
    let mut v = vec![]; // Rust 的类型检查器还无法确定 v 的类型
    v.push(2); // Rust 的类型检查器根据环境和使用方式推断 v 的类型是 Vec<i32>
    v.push(2.4f32); // 编译器会警告类型不匹配 error[E0308]: mismatched types
}