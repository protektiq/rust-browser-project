use std::io; // This line imports the Input/Output library from the standard library.

fn main() {
    println!("Hello! Type something, and I'll respond!");

    let mut input_string = String::new(); // We're creating a place to store your input.

    io::stdin() // This accesses the standard input.
        .read_line(&mut input_string) // This reads what you type into `input_string`.
        .expect("Failed to read line"); // This line helps us catch errors if something goes wrong.

    let input_trimmed = input_string.trim(); // This removes any extra spaces or newline characters.

    // Now, let's decide what to say back!
    if input_trimmed == "hello" {
        println!("Hi there! Nice to meet you.");
    } else if input_trimmed == "bye" {
        println!("Goodbye! Have a great day!");
    } else {
        println!("You said: '{}'", input_trimmed); // If you type something else, it'll just repeat it back.
    }
}
