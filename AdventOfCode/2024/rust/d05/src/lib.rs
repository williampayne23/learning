use std::{collections::{HashMap, HashSet}, str::FromStr};
use nom::{character::complete::digit1, character::complete::line_ending, combinator::map_res, multi::{separated_list1}, sequence::separated_pair, IResult};
use itertools::Itertools;

// Parser for a single unsigned integer
fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(parse_u32, nom::character::complete::char('|'), parse_u32)(input)
}

fn newline(input: &str) -> IResult<&str, &str> {
    line_ending(input)
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>)> {
    let (input, pairs) = separated_list1(newline, parse_pair)(input)?;
    let mut after_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    pairs.into_iter().for_each(|(a, b)| {
        // b must be after a
        if after_map.contains_key(&a) {
            after_map.get_mut(&a).unwrap().insert(b);
        } else {
            let mut set = HashSet::new();
            set.insert(b);
            after_map.insert(a, set);
        }
    });
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, ns) = separated_list1(newline, separated_list1(nom::character::complete::char(','), parse_u32))(input)?;
    return Ok((input, (after_map, ns)));
}

fn check_overlap(a: &HashSet<u32>, b: &HashSet<u32>) -> bool {
    a.iter().any(|n| b.contains(n))
}

pub fn process_part_1(input: &str) -> u32 {
    let (_, (after_map, ns)) = parse_input(input).unwrap();
    println!("{:?}", after_map);
    ns.iter().filter(|list| {
        let mut visited = HashSet::new();
        let valid = list.iter().all(|n| {
            let val = if let Some(after_set) = after_map.get(n) {
                // after_set is a set of numbers that must come after n
                // If any of these numbers have been visited, then list is not valid
                !check_overlap(&visited, after_set)
            } else {
                true
            };
            visited.insert(*n);
            val
        });
        valid
    })
    .inspect(|list| {
        println!("{:?}", list);
    }).map(|list| {
            // Return middle item
            let len = list.len();
            list[len / 2]
    }).sum()
}


pub fn process_part_2(input: &str) -> u32{
    let (_, (after_map, ns)) = parse_input(input).unwrap();
    ns.iter().filter(|list| {
        let mut visited = HashSet::new();
        let valid = list.iter().all(|n| {
            let val = if let Some(after_set) = after_map.get(n) {
                // after_set is a set of numbers that must come after n
                // If any of these numbers have been visited, then list is not valid
                !check_overlap(&visited, after_set)
            } else {
                true
            };
            visited.insert(*n);
            val
        });
        !valid

    })
    .map(|list| {
        list.iter().sorted_by(|a, b| {
            let empty_set = HashSet::new();
            let a_set = after_map.get(a).unwrap_or(&empty_set);
            let b_set = after_map.get(b).unwrap_or(&empty_set);
            if a_set.contains(b) {
                std::cmp::Ordering::Greater
            } else if b_set.contains(a) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        }).collect::<Vec<&u32>>()
    }).map(|list| {
            // Return middle item
            let len = list.len();
            list[len / 2]
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 143, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 123, "Failed example 2");
    }
}

