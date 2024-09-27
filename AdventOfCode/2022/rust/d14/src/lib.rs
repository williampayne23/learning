use std::collections::HashMap;

use nom::{IResult, multi::separated_list0, bytes::complete::tag, sequence::separated_pair, character::complete};


fn parse_path_segment(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, nums) = separated_pair(complete::u32, tag(","), complete::u32)(input)?;
    Ok((input, nums))

}

fn parse_path(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, path) = separated_list0(tag(" -> "), parse_path_segment)(input)?;
    Ok((input, path))
}

fn process_puzzle_input(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    let (input, paths) = separated_list0(tag("\n"), parse_path)(input)?;
    Ok((input, paths))
}


fn range_from_unordered_pair(first: u32, second: u32) -> std::ops::RangeInclusive<u32> {
    let mut min = first;
    let mut max = second;
    if first > second {
        min = second;
        max = first;
    }
    min..=max
}

fn line_from_pair(first: (u32, u32), second: (u32, u32)) -> Vec<(u32, u32)> {
    let mut line: Vec<(u32, u32)> = Vec::new();
    if first.0 == second.0 {
        for y in range_from_unordered_pair(first.1, second.1) {
            line.push((first.0, y));
        }
    } else if first.1 == second.1 {
        for x in range_from_unordered_pair(first.0, second.0) {
            line.push((x, first.1));
        }
    }
    line
}

fn construct_hash_map(input: Vec<Vec<(u32, u32)>>) -> HashMap<(u32, u32), bool> {
    let mut grid: HashMap<(u32, u32), bool> = HashMap::new();
    for path in input {
        for window in path.windows(2){
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];
            let line = line_from_pair((x1, y1), (x2, y2));
            for (x, y) in line {
                grid.insert((x, y), true);
            }
        }
    }
    grid
}

fn draw_grid(grid: &HashMap<(u32, u32), bool>) {
    let mut min_x = u32::MAX;
    let mut max_x = 0;
    let mut min_y = u32::MAX;
    let mut max_y = 0;
    for (x, y) in grid.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    for y in (min_y-1)..=(max_y+1) {
        for x in (min_x-1)..=(max_x+1) {
            if let Some(wall) = grid.get(&(x, y)) {
                if *wall {
                    print!("#");
                } else {
                    print!("o");
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

pub fn process_part_1(input: &str) -> u32 {
    let (_, paths) = process_puzzle_input(input).unwrap();
    let mut grid = construct_hash_map(paths);
    let mut sand = (500, 0);
    let mut n = 0;
    let max_y = grid.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    // draw_grid(&grid);
    while sand.1 <= max_y {
        let goal_pos = (sand.0, sand.1 + 1);
        if !grid.contains_key(&goal_pos) {
            sand = goal_pos;
            continue
        }
        let goal_pos = (sand.0 - 1, sand.1 + 1);
        if !grid.contains_key(&goal_pos) {
            sand = goal_pos;
            continue
        }
        let goal_pos = (sand.0 + 1, sand.1 + 1);
        if !grid.contains_key(&goal_pos) {
            sand = goal_pos;
            continue
        }
        n = n + 1;
        grid.insert(sand, true);
        sand = (500, 0);
    }
    n
}


pub fn process_part_2(input: &str) -> u32{
    let (_, paths) = process_puzzle_input(input).unwrap();
    let mut grid = construct_hash_map(paths);
    let mut sand = (500, 0);
    let mut n = 0;
    let max_y = grid.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min_x = grid.keys().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0 - 5;
    let max_x = grid.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0 + 5;
    let floor = max_y + 2;
    for x in min_x..=max_x {
        grid.insert((x, floor), true);
    }
    // draw_grid(&grid);
    loop {
        let goal_pos = (sand.0, sand.1 + 1);
        if !grid.contains_key(&goal_pos) && goal_pos.1 < floor {
            sand = goal_pos;
            continue
        }
        let goal_pos = (sand.0 - 1, sand.1 + 1);
        if !grid.contains_key(&goal_pos) && goal_pos.1 < floor  {
            sand = goal_pos;
            continue
        }
        let goal_pos = (sand.0 + 1, sand.1 + 1);
        if !grid.contains_key(&goal_pos) && goal_pos.1 < floor  {
            sand = goal_pos;
            continue
        }
        n = n + 1;
        if sand == (500, 0) {
            break
        }
        grid.insert(sand, false);
        sand = (500, 0);
    }
    // draw_grid(&grid);
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 24, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 93, "Failed example 2");
    }
}

