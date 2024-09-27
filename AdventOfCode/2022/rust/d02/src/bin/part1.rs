use d02::process_part_1;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    process_part_1(&file);
}
