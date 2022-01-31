// vec is similar to array however
// the size of a vec can change
// Vec::new or vec![] can create vec

fn main() {
    // create vec with Vec::new
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(1);
    numbers_vec.push(2);

    // create vec with vec![]
    let mut vec_with_macro = vec![1,2];
    vec_with_macro.push(3);
    println!("Pops out: {:?}", vec_with_macro.pop());

    let message = if numbers_vec == vec_with_macro {
        "They are equal!"
    } else {
        "Nah! They look different to me."
    };

    println!("{} {:?} {:?}", message, numbers_vec, vec_with_macro);
}