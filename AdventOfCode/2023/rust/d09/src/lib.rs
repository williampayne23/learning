use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};


fn get_next_number(numbers: &Vec<i32>) -> i32 {
    if numbers.iter().all(|&n| n == 0) {
        return 0;
    }

    let new_seq = numbers
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<i32>>();
    let inc = get_next_number(&new_seq);
    return numbers.last().unwrap() + inc;
}

fn process_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, lines) =
        separated_list1(line_ending, separated_list1(space1, complete::i32))(input)?;
    Ok((input, lines))
}

pub fn process_part_1(input: &str) -> i32 {
    let (_, lines) = process_input(input).unwrap();
    lines.iter().map(|l| get_next_number(&l)).sum()
}

pub fn process_part_2(input: &str) -> i32 {
    let (_, mut lines) = process_input(input).unwrap();
    lines.iter_mut().map(|l| {
        l.reverse();
        get_next_number(&l)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_1() {
        assert_eq!(
            process_part_1(EXAMPLE_PUZZLE_INPUT),
            114,
            "Failed example 1"
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 2, "Failed example 2");
    }
}
