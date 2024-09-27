use regex::Regex;
pub fn process_part_1(input: &str) -> u32 {
    let re = Regex::new(r"[A-Za-z]").unwrap();
    input.lines().map(|line| {
        let result = re.replace_all(line, "");
        let first_char = result.chars().nth(0).unwrap();
        let last_char = result.chars().nth(result.len() - 1).unwrap();
        first_char.to_digit(10).unwrap() * 10 + last_char.to_digit(10).unwrap()
    }).sum::<u32>() as u32
}

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const DIGITS: [&str; 10] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];


pub fn process_part_2(input: &str) -> u32{
    input.lines().map(|line| {
        let first_and_last_number = NUMBERS.iter().enumerate().fold((0, usize::MAX, 0, usize::MIN), |mut acc, (index, number)| {
            let first = line.find(number);
            let last = line.rfind(number);
            if let Some(pos) = first {
                if pos <= acc.1 {
                    acc.0 = index;
                    acc.1 = pos;
                }
            }
            if let Some(pos) = last {
                if pos >= acc.3 {
                    acc.2 = index;
                    acc.3 = pos;
                }
            }
            acc
        });
        let first_and_last_digit = DIGITS.iter().enumerate().fold((0, usize::MAX, 0, usize::MIN), |mut acc, (index, number)| {
            let first = line.find(number);
            let last = line.rfind(number);
            if let Some(pos) = first {
                if pos <= acc.1 {
                    acc.0 = index;
                    acc.1 = pos;
                }
            }
            if let Some(pos) = last {
                if pos >= acc.3 {
                    acc.2 = index;
                    acc.3 = pos;
                }
            }
            acc
        });
        let first = if first_and_last_number.1 < first_and_last_digit.1 {
            first_and_last_number.0
        } else {
            first_and_last_digit.0
        };
        let last = if first_and_last_number.3 > first_and_last_digit.3 {
            first_and_last_number.2
        } else {
            first_and_last_digit.2
        };
        println!("{} {:?} {:?} {}", line, first_and_last_number, first_and_last_digit, first as u32 * 10 + last as u32);
        first as u32 * 10 + last as u32
    }).sum::<u32>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const EXAMPLE_PUZZLE_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 142, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT_2), 281, "Failed example 2");
    }
}

