use std::io::{self, Write};

mod even_or_odd;

pub use self::even_or_odd::{is_even, is_odd};

fn main() {
    let mut buffer = String::new();

    println!("Hello, Rust!");

    print!("Enter a number to test: ");

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut buffer)
        .expect("Failed to read from stdin");

    let num = buffer.trim().parse::<usize>().expect("Enter a valid number");

    if is_even(num) {
        println!("{} is an even number", num);
    } else if is_odd(num) {
        println!("{} is an odd number", num);
    } else {
        println!("Impossible is nothing 😮!");
    }
}
