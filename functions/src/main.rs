fn main() {
    hello_world();
    let x = five();
    println!("the number of x is -> {x}");
    let sum = add(x);
    println!("the number of sum is -> {sum}");
}

fn hello_world() {
    println!("Hello, world!");
}

fn five() -> i32 {
    5
}

fn add(x: i32) -> i32 {
    x + 1
}
