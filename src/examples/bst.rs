/*
Scalar Type: Single Value type
*/
fn main() {
    print_integer();
    print_float();
    print_bool();
    print_chars();
}

/*
8 bit: i8, u8.
16 bit: i16, u16,
32 bit: i32, u32.
64 bit: i64, u64
128 bit: i128, u128

i: sgined: postive and negative values.
u: unsigned: can only non-negative values.

Decimal: 98222
Hexadecimal: 255
Octal: 63
Binary: 240
Byte: 65
*/
fn print_integer() {
    // Integer Types
    let decimal = 98_222; // Decimal
    let hex = 0xff; // Hexadecimal
    let octal = 0o77; // Octal
    let binary = 0b1111_0000; // Binary
    let byte = b'A'; // Byte (u8 only)

    println!("Decimal: {}", decimal);
    println!("Hexadecimal: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}", binary);
    println!("Byte: {}", byte);
}

fn print_float() {
    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32 with explicit type annotation

    // Basic arithmetic operations
    let sum = x + (y as f64);
    let difference = x - (y as f64);
    let product = x * (y as f64);
    let quotient = x / (y as f64);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}

fn print_bool() {
    let t = true;
    let f: bool = false; // With explicit type annotation

    // Using booleans in conditional statements
    if t {
        println!("This will print");
    }

    if !f {
        println!("This will also print");
    }
}

fn print_chars() {
    // 4 byte in size
    // it can represent letters, accented chars, and digits, and emojis, etc.

    let c = 'z'; // ASCII character
    let z = 'Z'; // Another ASCII character
    let heart = '❤'; // Unicode character
    let emoji = '😻'; // Emoji (also Unicode)

    println!("Characters: {}, {}, {}, {}", c, z, heart, emoji);

    // Iterating over characters in a string
    for character in "Hello, 世界!".chars() {
        println!("{}", character);
    }
}
