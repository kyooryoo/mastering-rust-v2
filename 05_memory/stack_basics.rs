// stack sample

// step 2: stack content updated:
// [[a=12, result=0], [b=12, temp=2*x, x=0]]
fn double_of(b: i32) -> i32 {
    let x = 2 * b;
    x
}

// step 1: main called, create stack
// [[a=12, result=0]]
fn main() {
    let a = 12;
    // step 3: double_of returns and stack updated
    // [[a=12, result=24]]
    let result = double_of(a);
}

// step 4: main executed, stack clears: [] 