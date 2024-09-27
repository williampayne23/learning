use std::collections::BTreeSet;

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn process_part_1(input: &str) -> u32 {
    let map = parse_input(input);
    let width = map[0].len();
    let height = map.len();
    let mut empty_row_set = (0..height).into_iter().collect::<BTreeSet<usize>>();
    let mut empty_col_set = (0..width).into_iter().collect::<BTreeSet<usize>>();
    map.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, c)| {
            if *c == '#' {
                empty_row_set.remove(&row);
                empty_col_set.remove(&col);
            }
        })
    });
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut expanded_rows = 0;
    let mut expanded_cols = 0;
    print!("  ");
    (0..(width + empty_col_set.len()))
        .into_iter()
        .for_each(|i| {
            if empty_col_set.contains(&i) {
                print!("v")
            } else {
                print!(" ")
            }
        });
    println!();
    map.iter().enumerate().for_each(|(row, line)| {
        expanded_cols = 0;
        if empty_row_set.contains(&row) {
            expanded_rows += 1;
            print!(
                "> {} <",
                (0..(width + empty_col_set.len()))
                    .into_iter()
                    .map(|_| ".")
                    .collect::<String>()
            );
            println!();
            return;
        }
        print!("  ");
        line.iter().enumerate().for_each(|(col, c)| {
            if empty_col_set.contains(&col) {
                expanded_cols += 1;
                print!("..");
                return;
            }
            if *c == '#' {
                print!("*");
                galaxies.push((row + expanded_rows, col + expanded_cols));
            } else {
                print!(".");
            }
        });
        println!();
    });
    print!("  ");
    (0..(width + empty_col_set.len()))
        .into_iter()
        .for_each(|i| {
            if empty_col_set.contains(&i) {
                print!("^")
            } else {
                print!(" ")
            }
        });
    println!();
    galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            let (a, b) = (pair[0], pair[1]);
            let (x1, y1) = a;
            let (x2, y2) = b;
            let (dx, dy) = (*x2 as i32 - *x1 as i32, *y2 as i32 - *y1 as i32);
            dx.abs() + dy.abs()
        })
        .sum::<i32>() as u32
}

pub fn process_part_2(input: &str, expansion_rate: u64) -> u64 {
    let map = parse_input(input);
    let width = map[0].len();
    let height = map.len();
    let mut empty_row_set = (0..height).into_iter().collect::<BTreeSet<usize>>();
    let mut empty_col_set = (0..width).into_iter().collect::<BTreeSet<usize>>();
    map.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, c)| {
            if *c == '#' {
                empty_row_set.remove(&row);
                empty_col_set.remove(&col);
            }
        })
    });
    let mut galaxies: Vec<(u64, u64)> = vec![];
    let mut expanded_rows: u64 = 0;
    let mut expanded_cols: u64 = 0;
    map.iter().enumerate().for_each(|(row, line)| {
        expanded_cols = 0;
        if empty_row_set.contains(&row) {
            expanded_rows += 1;
            return;
        }
        line.iter().enumerate().for_each(|(col, c)| {
            if empty_col_set.contains(&col) {
                expanded_cols += 1;
                return;
            }
            if *c == '#' {
                galaxies.push(((row as u64) + expanded_rows * (expansion_rate - 1), (col as u64) + expanded_cols * (expansion_rate - 1)));
            } else {
            }
        });
    });
    galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            let (&a, &b) = (pair[0], pair[1]);
            let (x1, y1) = a;
            let (x2, y2) = b;
            let (dx, dy) = (x2.abs_diff(x1), y2.abs_diff(y1));
            dx + dy
        })
        .sum::<u64>() 
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part_1() {
        assert_eq!(
            process_part_1(EXAMPLE_PUZZLE_INPUT),
            374,
            "Failed example 1"
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT, 100), 8410, "Failed example 2");
    }
}
