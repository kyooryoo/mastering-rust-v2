// use mut for function args if it needs to change
fn increase_by(mut val: usize, how_mucn: usize) {
    val += how_mucn;
    println!("You made {} points", val);
}

fn main() {
    let score = 2048;
    increase_by(score, 30);
}