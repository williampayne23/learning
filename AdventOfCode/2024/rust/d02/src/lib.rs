use std::collections::HashSet;


pub fn is_safe(numbers: &Vec<i32>) -> bool {
    let set = numbers.windows(2).map(|pair| {
        pair[0] - pair[1]
    }).collect::<HashSet<i32>>();
    // Set should either be a subset of 1,2,3 or -1,-2,-3
    set.is_subset(&[-1, -2, -3].iter().cloned().collect()) || set.is_subset(&[1, 2, 3].iter().cloned().collect())
}


pub fn process_part_1(input: &str) -> u32 {
    input.lines().filter(|line| {
        let numbers: Vec<i32> = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        is_safe(&numbers)
    }).count() as u32
}


pub fn process_part_2(input: &str) -> u32{
    input.lines().filter(|line| {
        let numbers: Vec<i32> = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        (0..numbers.len()).any(|i| {
            // Create a slice with the element at index `i` removed
            let slice = [&numbers[..i], &numbers[i+1..]].concat();
            is_safe(&slice)
        })
    }).count() as u32

}
// d0 = n1 - n0
// d1 = n2 - n1
// d2 = n3 - n2
// d1 + d0 = n2 - n0
//

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 2, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 4, "Failed example 2");
    }
}

