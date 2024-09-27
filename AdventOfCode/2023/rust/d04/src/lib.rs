use std::collections::{HashSet, HashMap};

use nom::{IResult, bytes::complete::tag, character::complete::{self, space1}, multi::separated_list0};


fn parse_line(input: &str) -> IResult<&str, (HashSet<u32>, Vec<u32>)> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, numbers) = separated_list0(space1, complete::u32)(input)?;
    let (input, _) = tag(" |")(input)?;
    let (input, _) = space1(input)?;
    let (input, numbers2) = separated_list0(space1, complete::u32)(input)?;
    Ok((input, (numbers.into_iter().collect::<HashSet<u32>>(), numbers2)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(HashSet<u32>, Vec<u32>)>> {
    separated_list0(tag("\n"), parse_line)(input)
}

pub fn process_part_1(input: &str) -> u32 {
    let (input, cards) = parse_input(input).unwrap();
    println!("{:?}", cards);
    println!("{:?}", input);
    cards.iter().map(|(winning, ours)| {
        let n = ours.iter().filter(|&n| winning.contains(n)).count();
        if n == 0 {
            return 0;
        }
        2_u32.pow(n as u32 - 1)
    }).sum()
}

fn visit(card_id: usize, maps: &HashMap<usize, Vec<usize>>, seen: &mut HashMap<usize, u32>) -> u32 {
    if let Some(cards) = seen.get(&card_id) {
        return *cards;
    }
    if let Some(next) = maps.get(&card_id) {
        let mut total = 1;
        for next_id in next {
            total += visit(*next_id, maps, seen);
        }
        seen.insert(card_id, total);
        return total;
    }
    seen.insert(card_id, 1);
    1
}

pub fn process_part_2(input: &str) -> u32{
    let (_, cards) = parse_input(input).unwrap();
    let maps = cards.iter().enumerate().map(|(i, (winning, ours))| {
        let n = ours.iter().filter(|&n| winning.contains(n)).count();
        let range = (i + 1)..(i + 1 + n);
        (i, range.collect::<Vec<usize>>())
    }).filter(|(i, next)| next.len() > 0).collect::<HashMap<usize, Vec<usize>>>();
    let mut total = 0;
    let mut seen: HashMap<usize, u32> = HashMap::new();
    for i in (0..cards.len()).rev() {
        total += visit(i, &maps, &mut seen);
    }
    total

}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 13, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 30, "Failed example 2");
    }
}

