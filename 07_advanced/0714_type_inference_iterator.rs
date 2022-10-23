use std::fs::File;
use std::io::Read;

fn main() {
    let file = File::open("foo.txt").unwrap();
    
    // // file.bytes()是一个迭代器，
    // // 其上的collect()聚合器方法收集的结果集可以是任何类型
    // // 如下代码会报错 error[E0282]: type annotations needed
    // let bytes = file.bytes().collect();

    // 为 bytes 指定数据类型后，编译和运行可以通过了
    let bytes: Vec<Result<u8, _>> = file.bytes().collect();
    println!("{:?}", bytes);
}