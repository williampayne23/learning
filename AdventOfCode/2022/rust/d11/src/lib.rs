use std::iter::zip;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, one_of},
    multi::separated_list0,
    sequence::preceded,
    IResult, Parser,
};

#[derive(Debug)]
struct Monkey {
    id: u64,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    true_case: u64,
    false_case: u64,
    count: u64,
}

impl Monkey {
    fn op(self: &Self, old: u64) -> u64 {
        let n = match self.operation.rhs {
            RHS::Num(x) => x,
            RHS::Old => old,
        };
        match self.operation.operation {
            OpType::Add => old + n,
            OpType::Mult => old * n,
        }
    }

    fn process_item(self: &mut Self, item: u64) -> (u64, u64) {
        let item = self.op(item);
        let item = item / 3;
        let test = item % self.test == 0;
        let next = if test {
            self.true_case
        } else {
            self.false_case
        };
        self.count = self.count + 1;
        (item, next)
    }

    fn process_item2(self: &mut Self, item: u64, magic_number: u64) -> (u64, u64) {
        let item = self.op(item);
        let item = item % magic_number;
        let test = item % self.test == 0;
        let next = if test {
            self.true_case
        } else {
            self.false_case
        };
        self.count = self.count + 1;
        (item, next)
    }
}

#[derive(Clone, Debug)]
struct Nimber {
    ns: Vec<(u64, u64)>,
}

impl Nimber {
    fn add(self: Self, rhs: &Nimber) -> Nimber {
        let mut new_ns: Vec<(u64, u64)> = vec![];
        for ((left, base), (right, base2)) in zip(self.ns.iter(), rhs.ns.iter()) {
            if base != base2 {
                panic!("Unmatched bases")
            }
            new_ns.push((((left % base) + (right % base)) % base, *base))
        }
        Nimber { ns: new_ns }
    }

    fn mult(self: Self, rhs: &Nimber) -> Nimber {
        let mut new_ns: Vec<(u64, u64)> = vec![];
        for ((left, base), (right, base2)) in zip(self.ns.iter(), rhs.ns.iter()) {
            if base != base2 {
                panic!("Unmatched bases")
            }
            new_ns.push((((left % base) * (right % base)) % base, *base))
        }
        Nimber { ns: new_ns }
    }

    fn from_number(self: &Self, n: u64) -> Nimber {
        let mut ns = vec![];
        for (_, base) in self.ns.iter() {
            ns.push((n % base, base.to_owned()))
        }
        Nimber { ns }
    }

    fn from_bases(bases: Vec<u64>, n: u64) -> Nimber {
        let mut ns = vec![];
        for base in bases.iter() {
            ns.push((n % base, base.to_owned()))
        }
        Nimber { ns }
    }

    fn modulo(self: &Self, n: u64) -> Result<u64, String> {
        for (num, base) in self.ns.iter() {
            if base == &n {
                return Ok(*num % base);
            }
        }
        Err("Not a modulo operator of Nimber".to_owned())
    }
}

#[derive(Debug)]
struct Monkey2 {
    id: u64,
    items: Vec<Nimber>,
    operation: Operation,
    test: u64,
    true_case: u64,
    false_case: u64,
    count: u64,
}

impl Monkey2 {
    fn op(self: &Self, item: Nimber) -> Nimber {
        let n = match self.operation.rhs {
            RHS::Num(x) => item.from_number(x),
            RHS::Old => item.clone(),
        };

        match self.operation.operation {
            OpType::Add => item.add(&n),
            OpType::Mult => item.mult(&n),
        }
    }
    fn process_item(self: &mut Self, item: Nimber) -> (Nimber, usize) {
        let item = self.op(item);
        //Not dividing anymore. How to keep the numbers manageable?
        //Copout-y large number library
        //Other option, items now keep track of operations performed and can use that to solve
        //modulo test.
        //Other option, get rid of information not relevant to modulo.
        let test = item.modulo(self.test).unwrap() == 0;
        let next = if test {
            self.true_case
        } else {
            self.false_case
        };
        self.count = self.count + 1;
        (item, next as usize)
    }

    fn from_monkey(m: &Monkey, bases: &Vec<u64>) -> Monkey2 {
        Monkey2 {
            id: m.id,
            items: m
                .items
                .iter()
                .map(|i| Nimber::from_bases(bases.clone(), *i))
                .collect(),
            operation: m.operation.clone(),
            test: m.test,
            true_case: m.true_case,
            false_case: m.false_case,
            count: m.count,
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    operation: OpType,
    rhs: RHS,
}

#[derive(Debug, Clone)]
enum OpType {
    Add,
    Mult,
}

impl OpType {
    fn from_char(i: char) -> OpType {
        match i {
            '+' => OpType::Add,
            _default => OpType::Mult,
        }
    }
}

#[derive(Debug, Clone)]
enum RHS {
    Num(u64),
    Old,
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, nums) = separated_list0(tag(", "), complete::u64)(input)?;
    Ok((input, nums))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, operation) = one_of("+*")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, rhs) = alt((
        tag("old").map(|_| RHS::Old),
        complete::u64.map(|x| RHS::Num(x)),
    ))(input)?;
    let op = Operation {
        rhs,
        operation: OpType::from_char(operation),
    };
    Ok((input, op))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = preceded(tag("Monkey "), complete::u64)(input)?;
    let (input, items) = preceded(tag(":\n  Starting items: "), parse_number_list)(input)?;
    let (input, operation) = preceded(tag("\n  Operation: new = old "), parse_operation)(input)?;
    let (input, test) = preceded(tag("\n  Test: divisible by "), complete::u64)(input)?;
    let (input, true_case) =
        preceded(tag("\n    If true: throw to monkey "), complete::u64)(input)?;
    let (input, false_case) =
        preceded(tag("\n    If false: throw to monkey "), complete::u64)(input)?;

    Ok((
        input,
        Monkey {
            id,
            items,
            operation,
            test,
            true_case,
            false_case,
            count: 0,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    Ok(separated_list0(tag("\n\n"), parse_monkey)(input)?)
}

pub fn process_part_1(input: &str) -> u64 {
    let (_, mut monkeys) = parse_input(input).unwrap();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            for item in items.iter() {
                let (item, target) = monkeys[i].process_item(item.clone());
                monkeys[target as usize].items.push(item);
            }
            monkeys[i].items = vec![];
        }
        monkeys.iter().for_each(|m| println!("{:?}", m));
    }

    let mut counts: Vec<u64> = monkeys.iter().map(|m| m.count).collect();
    println!("{:?}", counts);
    counts.sort();
    counts.iter().rev().take(2).for_each(|x| println!("{}", x));
    counts.iter().rev().take(2).product()
}

pub fn process_part_2(input: &str) -> u64 {
    let (_, mut monkeys) = parse_input(input).unwrap();
    let magic_number = monkeys.iter().map(|m| m.test).product();
    // The monkey 2 and Nimber stuff is my overengineered solution after I missed the obvious trick
    // of modding by the product of the test primes. I did eventually get it though
    // Leaving the Nimber and Monkey2 stuff in for posterity 
    // let mut monkeys: Vec<Monkey2> = monkeys
    //     .iter()
    //     .map(|m| Monkey2::from_monkey(m, &bases))
    //     .collect();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            for item in items.iter() {
                let (item, target) = monkeys[i].process_item2(item.clone(), magic_number);
                monkeys[target as usize].items.push(item);
            }
            monkeys[i].items = vec![];
        }
        monkeys.iter().for_each(|m| println!("{:?}", m));
    }

    let mut counts: Vec<u64> = monkeys.iter().map(|m| m.count).collect();
    println!("{:?}", counts);
    counts.sort();
    counts.iter().rev().take(2).for_each(|x| println!("{}", x));
    counts.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_1() {
        assert_eq!(
            process_part_1(EXAMPLE_PUZZLE_INPUT),
            10605,
            "Failed example 1"
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 2713310158, "Failed example 2");
    }
}
