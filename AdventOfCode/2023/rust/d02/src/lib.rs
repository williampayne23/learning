use std::cmp;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending},
    combinator::map,
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse_color(input: &str) -> IResult<&str, (u32, Color)> {
    let (input, count) = complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alt((
        map(tag("red"), |_| Color::Red),
        map(tag("green"), |_| Color::Green),
        map(tag("blue"), |_| Color::Blue),
    ))(input)?;
    Ok((input, (count, color)))
}

fn parse_round(input: &str) -> IResult<&str, Vec<(u32, Color)>> {
    let (input, colors) = separated_list1(tag(", "), parse_color)(input)?;
    Ok((input, colors))
}

fn parse_game(input: &str) -> IResult<&str, Vec<Vec<(u32, Color)>>> {
    let (input, _) = tag("Game ")(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, games) = separated_list1(tag("; "), parse_round)(input)?;
    Ok((input, games))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Vec<Vec<(u32, Color)>>>> {
    let (input, games) = separated_list1(line_ending, parse_game)(input)?;
    Ok((input, games))
}

pub fn process_part_1(input: &str) -> u32 {
    let (_, games) = parse_games(input).unwrap();
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    games
        .iter()
        .enumerate()
        .filter_map(|(i, game)| {
            let result = game.iter().all(|round| {
                round.iter().all(|color| match color.1 {
                    Color::Red => color.0 <= red_max,
                    Color::Green => color.0 <= green_max,
                    Color::Blue => color.0 <= blue_max,
                })
            });
            if result {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>() as u32
}

pub fn process_part_2(input: &str) -> u32 {
    let (_, games) = parse_games(input).unwrap();
    games
        .iter()
        .map(|game| {
            let maxs = game.iter().fold((0, 0, 0), |acc, round| {
                round.iter().fold(acc, |acc, color| match color.1 {
                    Color::Red => (cmp::max(acc.0, color.0), acc.1, acc.2),
                    Color::Green => (acc.0, cmp::max(acc.1, color.0), acc.2),
                    Color::Blue => (acc.0, acc.1, cmp::max(acc.2, color.0)),
                })
            });
            maxs.0 * maxs.1 * maxs.2
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 8, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 2286, "Failed example 2");
    }
}
