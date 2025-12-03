use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, one_of},
    combinator::map,
    multi::separated_list0,
    number::complete::be_i32,
    IResult,
};

enum Operation {
    Add,
    Sub,
}

struct Instruction {
    op: Operation,
    amount: i32,
}

impl Instruction {
    fn apply(&self, value: i32) -> i32 {
        let res = match self.op {
            Operation::Add => (value + self.amount),
            Operation::Sub => (value - self.amount),
        };
        println!("{}", self);
        println!("{}", res);
        return res;
    }

    fn count_zeros(&self, value: i32) -> i32 {
        return 0;
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = match self.op {
            Operation::Add => "+",
            Operation::Sub => "-",
        };

        write!(f, "{}{}", icon, self.amount)
    }
}

fn parse_op(input: &str) -> IResult<&str, Operation> {
    alt((
        map(tag("L"), |_| Operation::Sub),
        map(tag("R"), |_| Operation::Add),
    ))(input)
}

pub fn parse_digits_as_u32(input: &str) -> IResult<&str, u32> {
    let (input, digit) = digit1(input)?;
    let num = digit.parse().unwrap();
    Ok((input, num))
}
fn parse_single_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, operation) = parse_op(input)?;
    let (input, number) = parse_digits_as_u32(input)?;
    let instr = Instruction {
        op: operation,
        amount: number as i32,
    };
    Ok((input, instr))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, operations) = separated_list0(newline, parse_single_instruction)(input)?;
    Ok((input, operations))
}

pub fn process_part_1(input: &str) -> u32 {
    match parse_input(input) {
        Ok((_, ops)) => {
            println!("50");
            // for item in &ops {
            //     println!("{}", item);
            // }

            ops.iter()
                .scan(50, |acc, x: &Instruction| Some(x.apply(*acc)))
                // .inspect(|v| println!("{}", v))
                .filter(|v| *v == 0)
                .count() as u32
        }
        Err(_) => panic!(),
    }
}

pub fn process_part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 3, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 6, "Failed example 2");
    }
}
