pub fn process_part_1(input: &str) -> u32 {
    let mut left_numbers = vec![];
    let mut right_numbers = vec![];
    input.lines().for_each(|line| {
        let mut numbers = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
        left_numbers.push(numbers.next().unwrap());
        right_numbers.push(numbers.next().unwrap());
    });
    left_numbers.sort();
    right_numbers.sort();
    left_numbers.iter().zip(right_numbers.iter()).map(|(l, r)| r.abs_diff(l.clone())).sum()
}

use std::collections::HashMap;

pub fn process_part_2(input: &str) -> u32{
    let mut left_numbers = vec![];
    let mut right_counter = HashMap::new();
    input.lines().for_each(|line| {
        let mut numbers = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
        left_numbers.push(numbers.next().unwrap());
        right_counter.entry(numbers.next().unwrap()).and_modify(|e| *e += 1).or_insert(1);
    });
    left_numbers.iter().fold(0, |acc, &n| {
        acc + right_counter.get(&n).unwrap_or(&0) * &n
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 11, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 31, "Failed example 2");
    }
}

