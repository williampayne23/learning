use nom::{IResult, bytes::complete::tag, character::complete::{multispace1, digit1}, character::complete, multi::separated_list1};

fn parse_number_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, numbers) = separated_list1(multispace1, complete::u64)(input)?;
    Ok((input, numbers))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)>{
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, times) = parse_number_list(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distances) = parse_number_list(input)?;
    Ok((input, (times, distances)))
}

fn  range(t: u64, d: u64) -> u64 {
    let ft = t as f64;
    let fd = d as f64;
    
    let root_term = ((ft * ft - 4. * fd) as f64).sqrt();
    let upper = (ft + root_term) / 2.;
    let lower = (ft - root_term) / 2.;
    ( upper.ceil() - lower.floor() - 1.) as u64
}

pub fn process_part_1(input: &str) -> u64 {
    let (_, (times, distances)) = parse_input(input).unwrap();
    times.iter().zip(distances.iter()).map(|(t, d)| {
        let r = range(*t, *d);
        println!("{} {} {}", t, d, r);
        r
    }).product()
}

fn unkerned_num(input: &str) -> IResult<&str, u64> {
    let (input, nums) = separated_list1(multispace1, digit1)(input)?;
    let number = nums.join("").parse::<u64>().unwrap();
    Ok((input, number))
}

fn parse_part_2(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, time) = unkerned_num(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distance) = unkerned_num(input)?;

    Ok((input, (time, distance)))
}

pub fn process_part_2(input: &str) -> u64{
    let (_, (time, distance)) = parse_part_2(input).unwrap();
    range(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 288, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 71503, "Failed example 2");
    }
}

