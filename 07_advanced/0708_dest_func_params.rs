#[derive(Debug)]
struct Container { items_count: u32 }

// // 以下函数旨在直接更新结构体属性值，但变量是解构后的，所以无法达成目的
// #[allow(unused_assignments, unused_variables)]
// fn increment_item(Container {mut items_count}: &mut Container) {
//     items_count =+ 1; // 这里的解构是成功的
// }

// 注意这里在 Container 和 items_count 前添加的 mut 关键词
fn increment_item(Container {items_count}: &mut Container) -> u32 {
    *items_count + 1
}

// 函数的参数 items_count 被解构为一个结构体，其字段被绑定到 items_count
fn calculate_cost(Container {items_count}: &Container) -> u32 {
    let rate = 67;
    rate * items_count
}

fn main() {
    let mut container = Container { items_count: 10 };
    println!("container before: {:?}", container);
    // // 这里试图直接更新结构体是失败的
    // increment_item(&mut container);
    container.items_count = increment_item(&mut container);
    println!("container after: {:?}", container);
    let total_cost = calculate_cost(&container);
    println!("Total cost: {}", total_cost);
}