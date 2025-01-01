use nom::{IResult, character::complete::{digit1, line_ending, one_of}, bytes::complete::tag};


fn parse_u32(input: &str) -> IResult<&str, u32> {
    let (input, num) = digit1(input)?;
    Ok((input, num.parse().unwrap()))
}

fn parse_list_of_u32(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, nums) = nom::multi::separated_list1(tag(","), parse_u32)(input)?;
    Ok((input, nums))
}

fn parse_register(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Register ")(input)?;
    let (input, _) = one_of("ABC")(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, reg) = digit1(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, reg.parse().unwrap()))
}

fn parse_computer(input: &str) -> IResult<&str, Computer> {

    let (input, reg_a) = parse_register(input)?;
    let (input, reg_b) = parse_register(input)?;
    let (input, reg_c) = parse_register(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = tag("Program: ")(input)?;
    let (input, instructions) = parse_list_of_u32(input)?;

    Ok((input, Computer {
        instructions,
        reg_a,
        reg_b,
        reg_c,
        instruction_pointer: 0,
        output: vec![],
    }))
}

struct Computer {
    instructions: Vec<u32>,
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    instruction_pointer: u32,
    output: Vec<u32>,
}

const ADV: u32 = 0;
const BXL: u32 = 1;
const BST: u32 = 2;
const JNZ: u32 = 3;
const BXC: u32 = 4;
const OUT: u32 = 5;
const BDV: u32 = 6;
const CDV: u32 = 7;
const TWO: u32 = 2;

impl Computer {
    fn parse_combo_operand(&self, operand: u32) -> u32 {
        assert!(operand < 8, "Invalid operand");
        match operand {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("Invalid operand 7"),
            _ => operand,
        }
    }

    fn execute_instruction(&mut self, instruction: u32, operand: u32) {
        match instruction {
            ADV => self.reg_a = self.reg_a / (TWO.pow(self.parse_combo_operand(operand))),
            BXL => self.reg_b ^= operand,
            BST => self.reg_b = self.parse_combo_operand(operand) % 8,
            JNZ => {
                if self.reg_a != 0 {
                    self.instruction_pointer = operand;
                }
            },
            BXC => self.reg_b ^= self.reg_c, 
            OUT => self.output.push(self.parse_combo_operand(operand) % 8),
            BDV => self.reg_b = self.reg_a / (TWO.pow(self.parse_combo_operand(operand))),
            CDV => self.reg_c = self.reg_a / (TWO.pow(self.parse_combo_operand(operand))),
            _ => panic!("Invalid instruction"),
        }
    }

    fn run(&mut self) {
        while self.instruction_pointer < self.instructions.len() as u32 {
            self.step_one();
        }
        println!("Output: {:?}", self.output);
    }

    fn step_one(&mut self) {
        let instruction = self.instructions[self.instruction_pointer as usize];
        let operand = self.instructions[self.instruction_pointer as usize + 1];
        let last_instruction_pointer = self.instruction_pointer;
        self.execute_instruction(instruction, operand);
        if last_instruction_pointer == self.instruction_pointer {
            self.instruction_pointer += 2;
        }
    }

    fn search_for_spline(&mut self) -> u32 {
        let mut a = 117440;
        let start_b = self.reg_b;
        let start_c = self.reg_c;
        loop {
            self.reg_a = a;
            self.reg_b = start_b;
            self.reg_c = start_c;
            if self.loop_until_spline() {
                break;
            }
            a += 1;
        }
        a
    }

    fn loop_until_spline(&mut self) -> bool {
        while self.instruction_pointer < self.instructions.len() as u32 {
            let last_output_len = self.output.len();
            self.step_one();
            if last_output_len != self.output.len() {
                // Check output matches instructions so far
                let mut match_so_far = true;
                for i in 0..self.output.len() {
                    if self.output[i] != self.instructions[i] {
                        match_so_far = false;
                        break;
                    }
                }
                if !match_so_far {
                    return false;
                }
            }
        }
        if self.output.len() != self.instructions.len() {
            return false;
        }
        for i in 0..self.output.len() {
            if self.output[i] != self.instructions[i] {
                return false;
            }
        }
        return true;
    }
}

pub fn process_part_1(input: &str) -> String {
    let (_, computer) = parse_computer(input).unwrap();
    let mut computer = computer;
    computer.run();

    let out = computer.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    out
}


pub fn process_part_2(input: &str) -> u32 {
    let (_, computer) = parse_computer(input).unwrap();
    let mut computer = computer;
    computer.search_for_spline()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE_PUZZLE_INPUT2: &str = "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), "4,6,3,5,6,3,5,2,1,0", "Failed example 1");
    }

    #[test]
    fn test_part_2a() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT2), "0,3,5,4,3,0", "Failed example 2");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT2), 117440, "Failed example 2");
    }
}

