use second::run;
use std::fs;

fn main() {
    let file_path = String::from("input.txt");
    let contents = fs::read_to_string(file_path).expect("Incorrect file path");
    let sum = run(&contents);
    println!("{}", sum);
}
