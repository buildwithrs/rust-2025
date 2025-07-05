/*
Rust Functions
* building blocks in rust allow you organize code into resuable unit.
* lowercase and the name is seperated by underscores.

fn function_name(param1: type1, param2: typ2) -> return_type {
    // func body
}

*/
fn main() {
    greet("Alice");
    greet("Bob");

    print_info("Alice", 12);
    print_info("Bob", 16);

    println!("{}", add(1000, 24));
    println!("{}", add(1, 2));

    println!("max(1, 2): {}", max(1, 2));
    println!("max(0, -1): {}", max(0, -1));

    complex_functions();
}

fn greet(name: &str) {
    println!("Hello, {}", name);
}

fn print_info(name: &str, age: u32) {
    println!("{} is {} years old", name, age);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // the last exp value is the return value, and no ;
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    b
}

fn complex_functions() {
    let operations: [fn(i32, i32) -> i32; 4] = [add, subtract, multiply, divide_safe];
    let a = 10;
    let b = 5;
    for func in operations {
        println!("call {:?} result: {}", func, func(a, b));
    }
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide_safe(a: i32, b: i32) -> i32 {
    if b == 0 { 0 } else { a / b }
}
