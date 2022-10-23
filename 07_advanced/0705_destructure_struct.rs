#[allow(dead_code)]
#[derive(Debug)]
enum Food {
    Pizza,
    Salad
}

#[allow(dead_code)]
#[derive(Debug)]
enum PaymentMode {
    Bitcoin,
    Credit
}

#[allow(dead_code)]
#[derive(Debug)]
struct Order {
    count: u8,
    item: Food,
    payment: PaymentMode
}

fn main() {
    let food_order = Order { count: 2, item: Food::Salad, payment: PaymentMode::Credit };
    // 假设从某处得到了一个 Order 的实例 food_order 
    // 可以用以下方式拿到该实例的每个属性并赋值给多个变量
    let Order { count, payment, .. } = &food_order; // 不关心的字段可以用 .. 忽略掉
    // 另外一种方式是使用 ref 关键字，在每个变量上添加，如下
    let Order { ref item, .. } = food_order;

    println!("{:?}", food_order);
    println!("count: {}", count);
    println!("payment: {:?}", payment);
    println!("item: {:?}", item);
}