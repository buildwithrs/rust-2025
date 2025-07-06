/*
Rust Control Flow
* if: execcute different code blocks based on conditions.
* loop, while, for
*/
fn main() {
    test_if();
    if_else();
    if_else_if_else();
    if_values();
    check_option();

    if_match();
}

fn test_if() {
    let num = 10;
    if num > 0 {
        println!("num {} is greater than 0", num);
    }
}

fn if_else() {
    let num = -2;
    if num > 0 {
        println!("num {} is greater than 0", num);
    } else {
        println!("num: {} is less than or equal to 0", num);
    }
}

fn if_else_if_else() {
    let num = 0;
    if num > 0 {
        println!("num {} is greater than 0", num);
    } else if num == 0 {
        println!("num {} is equal to 0", num);
    } else {
        println!("num: {} is less than 0", num);
    }
}

// in rust, if blocks are also expressions, then can return values.
fn if_values() {
    let n = 10;
    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
    println!("{} -> {}", n, big_n); // 10, 5
}

// if let: used to check if an Option contains a value
fn check_option() {
    let opt_value: Option<i32> = None;
    if let Some(val) = opt_value {
        println!("val is: {}", val);
    } else {
        println!("the option value is None");
    }
}

// use if in the match expression
fn if_match() {
    let val = 0;
    match val {
        1 => println!(" val is 1"),
        2 => println!(" val is 2"),
        _ if val > 2 => println!(" val is: {} greater than 2", val),
        _ => println!("val is: {}", val),
    }
}
