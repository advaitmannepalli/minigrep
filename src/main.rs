use std::env;
use std::fs;

fn main() {
    println!("---Hello, minigrep!---");

    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let query = &args[1];
    let file_name = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    println!("With text: \n{}", contents);
}
