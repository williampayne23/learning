use std::collections::HashSet;

use nom::{multi::{many1, separated_list1}, branch::alt, IResult, combinator::map, character::complete};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GridCell {
    Empty,
    Symbol(char),
    Number(u32),
}

fn parse_line(line: &str) -> IResult<&str, Vec<GridCell>> {
    let (input, cells) = many1(alt((
        map(complete::char('.'), |_| GridCell::Empty),
        map(complete::u32, |d| GridCell::Number(d)),
        map(complete::none_of("\n"), |c| GridCell::Symbol(c)),
        )))(line)?;
    let cells = cells.into_iter().map(|c| match c {
        GridCell::Number(d) => {
            if d > 99 {
                vec![GridCell::Number(d), GridCell::Number(d), GridCell::Number(d)]
            } else if d > 9 {
                vec![GridCell::Number(d), GridCell::Number(d)]
            } else {
                vec![GridCell::Number(d)]
            }
        },
        _ => vec![c],
    }).flatten().collect();
    Ok((input, cells))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<GridCell>>> {
    let (input, lines) = separated_list1(complete::char('\n'), parse_line)(input)?;
    Ok((input, lines))
}

pub fn process_part_1(input: &str) -> u32 {
    let (_, grid) = parse_input(input).unwrap();
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut num_set = HashSet::new();
            if let GridCell::Symbol(_) = grid[i][j] {
                if i > 0 {
                    if let GridCell::Number(d) = grid[i-1][j] {
                        num_set.insert(d);
                    }
                }
                if i < grid.len() - 1 {
                    if let GridCell::Number(d) = grid[i+1][j] {
                        num_set.insert(d);
                    }
                }
                if j > 0 {
                    if let GridCell::Number(d) = grid[i][j-1] {
                        num_set.insert(d);
                    }
                }
                if j < grid[i].len() - 1 {
                    if let GridCell::Number(d) = grid[i][j+1] {
                        num_set.insert(d);
                    }
                }
                if i > 0 && j > 0 {
                    if let GridCell::Number(d) = grid[i-1][j-1] {
                        num_set.insert(d);
                    }
                }
                if i > 0 && j < grid[i].len() - 1 {
                    if let GridCell::Number(d) = grid[i-1][j+1] {
                        num_set.insert(d);
                    }
                }
                if i < grid.len() - 1 && j > 0 {
                    if let GridCell::Number(d) = grid[i+1][j-1] {
                        num_set.insert(d);
                    }
                }
                if i < grid.len() - 1 && j < grid[i].len() - 1 {
                    if let GridCell::Number(d) = grid[i+1][j+1] {
                        num_set.insert(d);
                    }
                }
                total += num_set.iter().sum::<u32>()
            }
        }
    }
    total
}


pub fn process_part_2(input: &str) -> u32{
    let (_, grid) = parse_input(input).unwrap();
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut num_set = HashSet::new();
            if  GridCell::Symbol('*') == grid[i][j] {
                if i > 0 {
                    if let GridCell::Number(d) = grid[i-1][j] {
                        num_set.insert(d);
                    }
                }
                if i < grid.len() - 1 {
                    if let GridCell::Number(d) = grid[i+1][j] {
                        num_set.insert(d);
                    }
                }
                if j > 0 {
                    if let GridCell::Number(d) = grid[i][j-1] {
                        num_set.insert(d);
                    }
                }
                if j < grid[i].len() - 1 {
                    if let GridCell::Number(d) = grid[i][j+1] {
                        num_set.insert(d);
                    }
                }
                if i > 0 && j > 0 {
                    if let GridCell::Number(d) = grid[i-1][j-1] {
                        num_set.insert(d);
                    }
                }
                if i > 0 && j < grid[i].len() - 1 {
                    if let GridCell::Number(d) = grid[i-1][j+1] {
                        num_set.insert(d);
                    }
                }
                if i < grid.len() - 1 && j > 0 {
                    if let GridCell::Number(d) = grid[i+1][j-1] {
                        num_set.insert(d);
                    }
                }
                if i < grid.len() - 1 && j < grid[i].len() - 1 {
                    if let GridCell::Number(d) = grid[i+1][j+1] {
                        num_set.insert(d);
                    }
                }
                if num_set.len() == 2 {
                    total += num_set.iter().product::<u32>()
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 4361, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 1, "Failed example 2");
    }
}

