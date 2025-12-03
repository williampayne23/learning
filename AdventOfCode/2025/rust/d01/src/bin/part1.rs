use d01::process_part_1;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("{}", process_part_1(&file));
}
