// when number of fields is limited
// tuple struct is not a bad choice

struct Color(u8, u8, u8);

fn main() {
    // tuple does not have field name
    let white = Color(255, 255, 255);
    // field value is referred by index
    let red = white.0;
    let green = white.1;
    let blue = white.2;
    println!("Red value: {}", red);
    println!("Green value: {}", green);
    println!("Blue value: {}", blue);

    let orange = Color(255, 165, 0);
    // destruct values from struct
    let Color(r, g, b) = orange;
    println!("R: {}, G: {}, B: {} (orange)", r, g, b);
    // ignore some desctructed value
    let Color(r, g, _) = orange;
    println!("R: {}, G: {}, B: {} (orange)", r, g, b);
}