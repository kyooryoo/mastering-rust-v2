fn main() {
    // 使用裸代码块同时执行多个任务
    let precompute = {
        let a = (-34i64).abs();
        // println!("a: {}", a);
        let b = 345i64.pow(3);
        // println!("b: {}", b);
        let c = 3;
        a + b + c
    };

    // match表达式
    let result_msg = match precompute {
        42 => "done",
        a if a % 2 == 0 => "continue",
        _ => panic!("Oh no !")
    };

    println!("{}", result_msg);
}