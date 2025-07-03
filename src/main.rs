fn main() {
    // In Rust, variables are immutable by default, which means that once a value is bound to a name, you can't change that value. This is one of Rust's core design principles that helps provide safety and enables easier concurrency.

    let mut x = 5;
    println!("The value of x is: {}", x);

    // The following would cause a compiler error:
    x = 6; // error: cannot assign twice to immutable variable
    println!("{}", x);
}
/*
1. when the value won't change.
2. For better safety.
3. For easier concu:
4. Communicate intent:
*/

// to use mutable cases:
/*
1. change value over time: counter, state variable.
2. require in-place modification:
3. Require Internal Mutability: rand.
*/
