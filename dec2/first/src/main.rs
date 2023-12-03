use first::run;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Incorrect file path");
    let score = run(&contents);
    println!("{}", score)
}
