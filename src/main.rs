fn main() {
    println!("Hello, world!");
    another_function(5);
    print_measurement(20, 30);
    let x = return_number(800);
    println!("x is {x}")
}

fn another_function(x: u32) {
    println!("x is {x}")
}

fn print_measurement(h: u32, w: u32) {
    println!("height: {h}, width: {w}");
}

fn return_number(num: u32) -> u32 {
    num
}
