fn main() {
    println!("Hello, world!");

    let x = five();
    printx(x);
    let x = plus_one(x);
    printx(x);
}

fn printx(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
