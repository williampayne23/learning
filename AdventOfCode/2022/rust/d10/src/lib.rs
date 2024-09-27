use nom::{
    branch::alt, bytes::complete::tag, character::complete, combinator::map,
    multi::separated_list0, sequence::preceded, IResult, Parser,
};

#[derive(Debug, PartialEq)]
enum Op {
    Add(i32),
    Noop,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Op>> {
    let (input, ops) = separated_list0(
        tag("\n"),
        alt((
            map(tag("noop"), |_| Op::Noop),
            preceded(tag("addx "), complete::i32).map(|num| Op::Add(num)),
        )),
    )(input)?;
    Ok((input, ops))
}

pub fn process_part_1(input: &str) -> i32 {
    let mut x = 1;

    let (_, ops) = parse_input(input).unwrap();
    let mut pos = 0;
    let mut sub_tick = 0;
    let mut tick = 0;
    let mut total = 0;
    let positions = [20, 60, 100, 140, 180, 220];
    while pos < ops.len() {
        tick += 1;
        if positions.contains(&tick) {
            total += x * tick;
            // println!("{} {}", tick, x);
        }
        if let Op::Add(num) = ops[pos] {
            if sub_tick == 0 {
                sub_tick += 1;
            } else {
                x += num;
                sub_tick = 0;
                pos += 1;
            }
            continue;
        }

        pos += 1;
    }

    total
}

pub fn process_part_2(input: &str) -> String {
    let mut x = 1;
    let (_, ops) = parse_input(input).unwrap();
    let mut pos = 0;
    let mut sub_tick = 0;
    let mut tick = 0;
    let mut screen = "".to_string();
    while pos < ops.len() {
        let x_scan = tick % 40;
        let is_on_screen = x_scan >= x - 1 && x_scan <= x + 1;
        if x_scan == 0 && tick != 0 {
            screen = format!("{}\n", screen);
        }
        screen = if is_on_screen {
            format!("{}#", screen)
        } else {
            format!("{}.", screen)
        };
        // println!("Tick: {}; X_scan: {}; X: {} {}", tick, x_scan, x, if is_on_screen { "ON" } else { "OFF" });
        tick += 1;

        if let Op::Add(num) = ops[pos] {
            if sub_tick == 0 {
                sub_tick += 1;
            } else {
                x += num;
                sub_tick = 0;
                pos += 1;
            }
            continue;
        }

        pos += 1;
    }
    return screen.to_string();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let file = fs::read_to_string("test.txt").expect("Unable to read file");
        assert_eq!(process_part_1(&file), 13140, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        let file = fs::read_to_string("test.txt").expect("Unable to read file");
        assert_eq!(
            process_part_2(&file),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string(),
            "Failed example 2"
        );
    }
}
