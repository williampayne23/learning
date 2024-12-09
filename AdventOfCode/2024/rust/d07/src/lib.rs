use nom::IResult;

#[derive(Debug)]
struct Equation {
    value: u64,
    parts: Vec<u64>,
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    let (input, number_str) = nom::character::complete::digit1(input)?;
    let number = number_str.parse().unwrap();
    Ok((input, number))
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, numbers) =
        nom::multi::separated_list1(nom::character::complete::space1, parse_number)(input)?;
    Ok((input, numbers))
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, value) = parse_number(input)?;
    let (input, _) = nom::bytes::complete::tag(": ")(input)?;
    let (input, parts) = parse_number_list(input)?;
    Ok((input, Equation { value, parts }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    let (input, equations) =
        nom::multi::separated_list1(nom::character::complete::line_ending, parse_equation)(input)?;
    Ok((input, equations))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operators {
    Add,
    Multiply,
    Concatenate,
}

impl Operators {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operators::Add => a + b,
            Operators::Multiply => a * b,
            Operators::Concatenate => format!("{}{}", a, b).parse().unwrap(),
        }
    }

    fn from_char(c: char) -> Option<Operators> {
        match c {
            '+' => Some(Operators::Add),
            '*' => Some(Operators::Multiply),
            '|' => Some(Operators::Concatenate),
            _ => None,
        }
    }

    fn apply_from_char(c: char, a: u64, b: u64) -> u64 {
        match Operators::from_char(c) {
            Some(operator) => operator.apply(a, b),
            None => panic!("Invalid operator"),
        }
    }
}


fn dfs(target: u64, values: &[u64], index: usize, sum: u64, operator_functions: &str) -> bool {
    if index == values.len() {
        return sum == target;
    }
    if sum > target {
        return false;
    }
    operator_functions.chars().any(|operator| {
        dfs(target, values, index + 1, Operators::apply_from_char(operator, sum, values[index]), operator_functions)
    })
}

pub fn process_part_1(input: &str) -> u64 {
    let (_, equations) = parse_input(input).unwrap();
    equations.iter().filter(|equation| {
        let target = equation.value;
        let values = &equation.parts;
        dfs(target, values, 1, values[0], "+*")
    }).map(|equation| equation.value).sum()
}

pub fn process_part_2(input: &str) -> u64 {
    let (_, equations) = parse_input(input).unwrap();
    equations.iter().filter(|equation| {
        let target = equation.value;
        let values = &equation.parts;
        dfs(target, values, 1, values[0], "+*|")
    }).map(|equation| equation.value).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_1() {
        assert_eq!(
            process_part_1(EXAMPLE_PUZZLE_INPUT),
            3749,
            "Failed example 1"
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 11387, "Failed example 2");
    }
}
