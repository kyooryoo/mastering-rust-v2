fn compute(i: i32) -> i32 {
    2 * i
}

fn main() {
    let result_msg = "done";
    // if表达式赋值
    let result = if result_msg == "done" {
        let some_work = compute(8);
        let stuff = compute(4);
        compute(2) + stuff // 这里返回的值最终会赋值给result
    } else {
        compute(1)
    };

    println!("{}", result);
}