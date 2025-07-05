/*
Rust Tuples:
A Tuple is compound data type
Group together a number of values with potenial different types into one compound type.
Tuple have fixed length.
*/
fn main() {
    let person = (String::from("Alice"), 30, 5.8);
    println!("{:?}", person);

    let name = person.0; // the access the first element in the tuple
    let age = person.1; // access 2nd element.
    let height = person.2; // access 3rd element.
    println!("{}, {}, {}", name, age, height);

    let coordinates = (10, 20);
    println!("{:?}", coordinates);

    let (x, y) = coordinates;
    println!("{}, {}", x, y);

    // unit
    let unit = ();
    println!("{:?}", unit);
}
