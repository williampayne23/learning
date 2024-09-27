use std::collections::HashMap;

use nom::{IResult, multi::{separated_list0, many1}, character::complete::newline};

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug, PartialEq)]
enum Dir {
    Left,
    Right,
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, name) = nom::character::complete::alphanumeric1(input)?;
    let (input, _) = nom::bytes::complete::tag(" = (")(input)?;
    let (input, left) = nom::character::complete::alphanumeric1(input)?;
    let (input, _) = nom::bytes::complete::tag(", ")(input)?;
    let (input, right) = nom::character::complete::alphanumeric1(input)?;
    let (input, _) = nom::bytes::complete::tag(")")(input)?;
    Ok((input, Node {
        name: name.to_string(),
        left: left.to_string(),
        right: right.to_string(),
    }))
}

fn parse_instruction(input: &str) -> IResult<&str, Dir> {
    let (input, dir) = nom::branch::alt((nom::bytes::complete::tag("L"), nom::bytes::complete::tag("R")))(input)?;
    Ok((input, match dir {
        "L" => Dir::Left,
        "R" => Dir::Right,
        _ => unreachable!(),
    }))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Dir>, HashMap<String, String>, HashMap<String, String>)> {
    let (input, instructions) = many1(parse_instruction)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, nodes) = separated_list0(newline, parse_node)(input)?;
    let left_map = nodes.iter().map(|n| (n.name.clone(), n.left.clone())).collect::<HashMap<_, _>>();
    let right_map = nodes.iter().map(|n| (n.name.clone(), n.right.clone())).collect::<HashMap<_, _>>();
    Ok((input, (instructions, left_map, right_map)))
}

pub fn process_part_1(input: &str) -> u64 {
    let (_, (instructions, left_map, right_map)) = parse_input(input).unwrap();
    let mut current_node = "AAA";
    let mut index = 0;
    while current_node != "ZZZ" {
        let dir = &instructions[index % instructions.len()];
        index += 1;
        match dir {
            Dir::Left => current_node = &left_map[current_node],
            Dir::Right => current_node = &right_map[current_node],
        }
    }
    index as u64
}

fn prime_factors(n: usize) -> HashMap<usize, usize> {
    let mut factors = HashMap::new();
    let mut n = n;
    let mut i = 2;
    while i <= n {
        if n % i == 0 {
            *factors.entry(i).or_insert(0) += 1;
            n /= i;
        } else {
            i += 1;
        }
    }
    factors
}

pub fn process_part_2(input: &str) -> u64{
    let (_, (instructions, left_map, right_map)) = parse_input(input).unwrap();
    let mut current_nodes = left_map.keys().filter(|&n| &n[2..] == "A").map(|n| n.clone()).collect::<Vec<_>>();
    let mut cycles: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut index = 0;
    while cycles.len() != current_nodes.len() || cycles.values().any(|v| v.len() < 4) { 
        current_nodes.iter().enumerate().for_each(|(i, n)| {
            if n.contains("Z") {
                cycles.entry(i).or_insert_with(Vec::new).push(index);
            }
        });
        let dir = &instructions[index % instructions.len()];
        index += 1;
        current_nodes = match dir {
            Dir::Left => current_nodes.iter().map(|n| left_map[n].clone()).collect::<Vec<_>>(),
            Dir::Right => current_nodes.iter().map(|n| right_map[n].clone()).collect::<Vec<_>>(),
        }
    }
    let cycles = cycles.into_iter().map(|(k, v)| {
        let diffs = v.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        let diffs = diffs.iter().skip(1).fold(Some(diffs[0]), |acc, &d| if acc == Some(d) { acc } else { None });
        if let Some(diff) = diffs {
            return (k,prime_factors(diff));
        }
        println!("{} {:?}", k, v);
        panic!("No cycle found");
    }).collect::<HashMap<_, _>>();
    let common_products = cycles.values().fold(HashMap::new(), |mut acc, v| {
        v.iter().for_each(|(k, v)| {
            if let Some(existing) = acc.get_mut(k) {
                *existing = std::cmp::max(*existing, *v);
            } else {
                acc.insert(*k, *v);
            }
        });
        acc
    });
    common_products.iter().map(|(k,v)| (*k as u64).pow(*v as u32)).product::<u64>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 6, "Failed example 1");
    }

    const EXAMPLE_PUZZLE_INPUT_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT_2), 6, "Failed example 2");
    }
}

