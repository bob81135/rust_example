fn main() {
    println!("Hello, world!");
    let _x = 5;
    let y = {
        let x = 3;
        plus_one(x)
    };
    let z = five() + 1;
    println!("y={}", y);
    println!("z={}", z);
    a_function();
    b_function(5);
}

fn a_function() {
    println!("Another function.");
}
fn b_function(x: i32) {
    println!("The value of x is: {}", x);
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}