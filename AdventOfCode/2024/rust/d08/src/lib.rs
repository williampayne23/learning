use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Antenna {
    x: i32,
    y: i32,
}

impl Antenna {
    fn reflect(&self, other: &Antenna) -> (i32, i32) {
        let ans = self.sub(other).add(self);
        return (ans.x, ans.y);
    }

    fn scale(&self, n: i32) -> Antenna {
        Antenna { x: self.x * n, y: self.y * n }
    }

    fn sub(&self, other: &Antenna) -> Antenna {
        Antenna { x: self.x - other.x, y: self.y - other.y }
    }

    fn add(&self, other: &Antenna) -> Antenna {
        Antenna { x: self.x + other.x, y: self.y + other.y }
    }
}


fn parse_input(input: &str) -> HashMap<char, Vec<Antenna>> {
    let mut res = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                res.entry(c).or_insert(Vec::new()).push(Antenna { x: x as i32, y: y as i32 });
            }
        })
    });
    res
}

fn ray(start: Antenna, direction: Antenna, width: i32, height: i32) -> Vec<Antenna> {
    let mut res = Vec::new();
    let mut current = start;
    while current.x >= 0 && current.x < width && current.y >= 0 && current.y < height {
        res.push(current.clone());
        current = current.add(&direction);
    }
    res
}

fn make_ray(start: Antenna, end: Antenna, width: i32, height: i32) -> Vec<Antenna> {
    let direction = end.sub(&start);
    ray(start, direction, width, height)
}

pub fn process_part_1(input: &str) -> u32 {
    let antennas = parse_input(input);
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    println!("{} {}", width, height);
    antennas.values().map(|a| {
        a.iter().cartesian_product(a.iter()).filter_map(|(a1, a2)| {
            if a1.x == a2.x && a1.y == a2.y {
                return None;
            }
            Some(a1.reflect(a2))
        }).filter(|(x, y)| {
            *x >= 0 && *x < width && *y >= 0 && *y < height
        })
    }).flatten().unique().inspect(|(x, y)| {
        println!("{} {}", x, y);
    }).count() as u32
}


pub fn process_part_2(input: &str) -> u32{
    let antennas = parse_input(input);
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    println!("{} {}", width, height);
    antennas.values().map(|a| {
        a.iter().cartesian_product(a.iter()).filter_map(|(a1, a2)| {
            if a1.x == a2.x && a1.y == a2.y {
                return None;
            }
            Some(make_ray(a1.clone(), a2.clone(), width, height))
        }).flatten().collect::<Vec<Antenna>>()
    }).flatten().unique().count() as u32
    
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 14, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 34, "Failed example 2");
    }
}

