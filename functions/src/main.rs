fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let x = five();
    println!("The value of five is: {x}");

    let y = plus_one(5);
    println!("5 + 1 = {y}")
}

fn another_function(x: i32, unit: char) {
    println!("Time is running out, you have {x}{unit} left.");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
