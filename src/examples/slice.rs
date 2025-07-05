/*
Rust Slices:
* Dynamic sized view into a contigous sequence of elements: [T]
* Refernce a section of array, vector, or String without ownership.
*/
fn main() {
    /*
    let s1 = &[T]; // default is immutable
    let s2 = &mut [T] // mutable slice
    let s3 = &str; // a String slice.
    */

    let numbers = [1, 2, 5, 6];
    let num = &numbers[1..3];
    println!("num: {:?}", num);

    let num1 = &numbers[2..]; // start from 5
    let num2 = &numbers[..]; // refer all elemnts
    let num3 = &numbers[..3]; // refer first 3 elements.
    println!("{:?}, {:?}, {:?}", num1, num2, num3);

    let msg = "Hello, World".to_string();
    let hello = &msg[..5]; // Hello, first 5 elements in the msg String.
    println!("{hello}");

    let mut colors = ["red", "green", "yellow", "white"];
    let color1 = &mut colors[1..3]; // refernce elements in index 1, 2
    println!("slice colors: {:?}", color1);

    color1[1] = "purple"; // change the value at index 1
    println!("updated color1: {:?}", color1);
}
