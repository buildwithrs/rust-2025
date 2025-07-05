/*
Rust Array:
* Allow you to store multiple values of the same type in a single data structure.
* Require all elements to be of the same type.
*/

use std::io;
use std::io::Read;

fn main() {
    create_arr();

    access_array();

    create_alice_from_array();

    create_2d_array();

    match read_data() {
        Ok(_) => println!("read success"),
        Err(e) => println!("error: {}", e),
    }
}

fn create_arr() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    let colors = ["red", "yellow", "green", "purple"];
    println!("{:?}", colors);

    let zeros = [0; 10]; // 10 zeros.
    println!("{:?}", zeros);
}

fn access_array() {
    let colors = ["red", "yellow", "green", "purple"];
    println!("{:?}", colors);

    println!("colors[3] = {}", colors[3]);
}

fn create_alice_from_array() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    let num_slice = &numbers[1..3]; // 2, 3
    println!("{:?}", num_slice);
}

fn create_2d_array() {
    let mut board = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];

    board[0][0] = 'X';
    board[1][1] = 'O';
    board[0][1] = 'X';
    for row in &board {
        println!("{:?}", row);
    }
}

fn read_data() -> io::Result<()> {
    let mut buffer = [0u8; 1024];
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let bytes_read = handle.read(&mut buffer)?;
    println!("read {} bytes: {:?}", bytes_read, &buffer[..bytes_read]);

    Ok(())
}
