fn get_nth(items: &Vec<usize>, nth: usize) -> Option<usize> {
    if nth < items.len() {
        Some(items[nth])
    } else {
        None
    }
}

fn double(val: usize) -> usize {
    val * val
}

fn main() {
    let items = vec![7,6,4,3,5,3,10,3,2,4];
    println!("{}", items.len());

    let doubled1 = get_nth(&items, 4).map(double);
    println!("{:?}", doubled1);

    let doubled2 = get_nth(&items, 5).map(|v| v * v);
    println!("{:?}", doubled2);
}