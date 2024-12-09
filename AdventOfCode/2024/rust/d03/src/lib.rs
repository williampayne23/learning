enum Operation {
    Mul,
    Do,
    Dont,
}

struct Instruction {
    op: Operation,
    arga: i32,
    argb: i32,
}


struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Parser {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn try_parse_number(&mut self, size: usize) -> Option<i32> {
        let mut num = 0;
        let mut num_digits = 0;
        let start_pos = self.pos;
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                num_digits += 1;
                num = num * 10 + c.to_digit(10).unwrap() as i32;
                self.next();
            } else {
                break;
            }
        }
        if num_digits <= size && num_digits > 0 {
            Some(num)
        } else {
            self.pos = start_pos;
            None
        }
    }

    fn try_parse_literal(&mut self, literal: &str) -> Option<()> {
        let start_pos = self.pos;
        for c in literal.chars() {
            if self.next() != Some(c) {
                self.pos = start_pos;
                return None;
            }
        }
        Some(())
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn try_parse_mul(&mut self) -> Option<Instruction> {
        let start_pos = self.pos;
        if self.try_parse_literal("mul(").is_none() {
            self.pos = start_pos;
            return None;
        }
        let arga = self.try_parse_number(3)?;
        if self.try_parse_literal(",").is_none() {
            self.pos = start_pos;
            return None;
        }
        let argb = self.try_parse_number(3)?;
        if self.try_parse_literal(")").is_none() {
            self.pos = start_pos;
            return None;
        }
        Some(Instruction { op: Operation::Mul, arga, argb })
    }

    fn try_parse_do(&mut self) -> Option<Instruction> {
        let start_pos = self.pos;
        if self.try_parse_literal("do()").is_none() {
            self.pos = start_pos;
            return None;
        }
        Some(Instruction { op: Operation::Do, arga: 0, argb: 0 })
    }

    fn try_parse_dont(&mut self) -> Option<Instruction> {
        let start_pos = self.pos;
        if self.try_parse_literal("don't()").is_none() {
            self.pos = start_pos;
            return None;
        }
        Some(Instruction { op: Operation::Dont, arga: 0, argb: 0 })
    }


}

fn parse_program(input: &str) -> Vec<Instruction> {
    let mut parser = Parser::new(input);
    let mut instructions = Vec::new();
    while !parser.is_eof() {
        if let Some(instr) = parser.try_parse_mul() {
            instructions.push(instr);
            continue;
        }

        if let Some(instr) = parser.try_parse_do() {
            instructions.push(instr);
            continue;
        }

        if let Some(instr) = parser.try_parse_dont() {
            instructions.push(instr);
            continue;
        }

        parser.next();
    }
    instructions
}



pub fn process_part_1(input: &str) -> u32 {
    let instructions = parse_program(input);
    let mut result = 0;
    for instr in instructions {
        match instr.op {
            Operation::Mul => {
                result += instr.arga * instr.argb;
            }
            _ => (),
        }
    }
    result as u32
}


pub fn process_part_2(input: &str) -> u32{
    let instructions = parse_program(input);
    let mut result = 0;
    let mut do_mul = true;
    for instr in instructions {
        match instr.op {
            Operation::Mul => {
                if do_mul {
                    result += instr.arga * instr.argb;
                }
            }
            Operation::Do => {
                do_mul = true;
            }
            Operation::Dont => {
                do_mul = false;
            }
            _ => (),
        }
    }
    result as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_PUZZLE_INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 161, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT2), 48, "Failed example 2");
    }
}

