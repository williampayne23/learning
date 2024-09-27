use std::cmp::Ordering;

use nom::{IResult, bytes::complete::tag, sequence::separated_pair, branch::alt, multi::{separated_list0, many1}};



#[derive(Debug, PartialEq)]
enum Item {
    Number(u32),
    List(Vec<Item>),
}

impl Item {
    fn compare(&self, item: &Item) -> Ordering {
        match self {
            Item::Number(n) => {
                if let Item::Number(m) = item {
                    if n < &m {
                        Ordering::Less
                    } else if n > &m {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                } else {
                    Item::List(vec![Item::Number(*n)]).compare(item)
                }
            },
            Item::List(items) => {
                match item {
                    Item::Number(m) => {
                        self.compare(&Item::List(vec![Item::Number(*m)]))
                    },
                    Item::List(other_items) => {
                        let res = items.iter().zip(other_items.iter()).map(|(a, b)| a.compare(b)).find(|comparison| {
                            match comparison {
                                Ordering::Equal => false,
                                Ordering::Greater => true,
                                Ordering::Less => true,
                            }
                        });
                        if let None = res {
                            if items.len() < other_items.len() {
                                Ordering::Less
                            } else if items.len() > other_items.len() {
                                Ordering::Greater
                            } else {
                                Ordering::Equal
                            }
                        } else {
                            res.unwrap()
                        }
                    }
                }
            }
        }
    }
}

fn list(input: &str) -> IResult<&str, Item> {
    let (input, _) = tag("[")(input)?;
    let (input, items) = separated_list0(tag(","), parse_item)(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Item::List(items)))
}

fn number(input: &str) -> IResult<&str, Item> {
    let (input, number) = nom::character::complete::digit1(input)?;
    Ok((input, Item::Number(number.parse().unwrap())))
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    let (input, items) = alt((list, number))(input)?;
    Ok((input, items))
}

fn parse_pair(input: &str) -> IResult<&str, (Item, Item)> {
    let (input, items) = separated_pair(parse_item, tag("\n"), parse_item)(input)?;
    Ok((input, items))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Item, Item)>> {
    let (input, items) = separated_list0(tag("\n\n"), parse_pair)(input)?;
    Ok((input, items))
}

fn parse_input2(input: &str) -> IResult<&str, Vec<Item>> {
    let (input, items) = separated_list0(many1(tag("\n")), parse_item)(input)?;
    Ok((input, items))
}

fn map_pairs(input: (usize, &(Item, Item))) -> usize {
    let (i, (a, b)) = input;
    match a.compare(b) {
        Ordering::Equal => 0,
        Ordering::Greater => 0,
        Ordering::Less => i + 1,
    }
}



trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Ordering {
    fn to_string(&self) -> String {
        match self {
            Ordering::Equal => "Equal".to_string(),
            Ordering::Greater => "Greater".to_string(),
            Ordering::Less => "Less".to_string(),
        }
    }
}

pub fn process_part_1(input: &str) -> u32 {
    let Ok((_, list)) = parse_input(input) else { panic!("Failed to parse input") };
    list.iter().enumerate().for_each(|(i, (a, b))| {
        println!("{}: {}", i, a.compare(b).to_string());
        println!("{:?}\n{:?}", a, b);
    });
    list.iter().enumerate().map(map_pairs).sum::<usize>() as u32
}


pub fn process_part_2(input: &str) -> u64{
    let Ok((_, mut list)) = parse_input2(input) else { panic!("Failed to parse input") };
    list.push(Item::List(vec![Item::List(vec![Item::Number(2)])]));
    list.push(Item::List(vec![Item::List(vec![Item::Number(6)])]));
    list.sort_by(|a, b| a.compare(b));
    list.iter().enumerate().filter_map(|(i, item)| {
        if let Item::List(items) = item {
            if items.len() != 1 {
                return None;
            }
            if let Some(Item::List(items)) = items.first() {
                if items.len() != 1 {
                    return None;
                }
               if let Some(Item::Number(n)) = items.first() {
                    if n == &2 {
                        return Some((i+1) as u64);
                    }
                    if n == &6 {
                        return Some((i+1) as u64);
                    }
                }
            }
        }
        None
    }).product::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 13, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 140, "Failed example 2");
    }
}

