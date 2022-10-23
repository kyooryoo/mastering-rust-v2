fn main() {
    let mut i = 0;
    let counter = loop {
        i += 1;
        if i == 10 {
            // 中断循环并返回 i
            break i;
        }
    };
    println!("{}", counter);
}