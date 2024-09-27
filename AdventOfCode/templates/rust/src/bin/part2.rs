use DAYPLACEHOLDER::process_part_2;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("{}", process_part_2(&file));
}
