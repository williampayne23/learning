use std::collections::HashMap;

use nom::{character::complete::{alpha1, digit1}, IResult, multi::many1, bytes::complete::tag};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: u32
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, dir) = alpha1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, dist) = digit1(input)?;
    let (input, _) = tag("\n")(input)?;

    return Ok((input, Move {
        direction: match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction")
        },
        distance: dist.parse::<u32>().unwrap()
    }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Move>> {
    many1(parse_move)(input)
}

#[derive(Eq, Debug, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y
        }
    }
}

impl Point {
    fn adjacent(&self, other: &Point) -> bool {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();
        return x_dist < 2 && y_dist < 2;
    }
}

#[derive(Debug)]
struct Rope {
    start_history: HashMap<Point, u32>,
    knots: Vec<Point>,
}

impl Rope {
    fn default(length: usize) -> Rope {
        let mut knots = Vec::new();
        for _ in 0..length {
            knots.push(Point { x: 0, y: 0 });
        }
        Rope {
            start_history: HashMap::new(),
            knots 
        }
    }

    fn save_pos(&mut self) {
        let start = self.knots.last().unwrap().clone();
        if !self.start_history.contains_key(&start) {
            self.start_history.insert(start, 1);
        }
    }

    fn move_one(&mut self, m: &Move) {

        //Move first       
        let knot = &mut self.knots[0]; 
        match m.direction {
            Direction::Up => knot.y += 1 as i32,
            Direction::Down => knot.y -= 1 as i32,
            Direction::Left => knot.x -= 1 as i32,
            Direction::Right => knot.x += 1 as i32
        }
        let mut new = knot.clone();
        //Make the rest follow
        for knot in self.knots.iter_mut().skip(1){
            let x_dist = (knot.x - new.x).abs();
            let y_dist = (knot.y - new.y).abs();
            if !(x_dist < 2 && y_dist < 2) {
                knot.x = if x_dist != 0 { knot.x + (new.x - knot.x) / x_dist } else { knot.x };
                knot.y = if y_dist != 0 { knot.y + (new.y - knot.y) / y_dist } else { knot.y };
            }
            if !knot.adjacent(&new) {
                panic!("Not adjacent");
            }
            new = knot.clone();
        }
        self.save_pos();
    }

    fn move_many(&mut self, m: &Move) {
        for _ in 0..m.distance {
            self.move_one(m);
        }
    }
}

pub fn process_part_1(input: &str) -> u32 {
    //Add a new line to make sure we parse the end
    let (_, moves) = parse_input(format!("{}\n", input).as_str()).unwrap();
    let mut rope = Rope::default(2);
    moves.iter().for_each(|m| rope.move_many(m));
    rope.start_history.len() as u32
}


pub fn process_part_2(input: &str) -> u32{
    let (_, moves) = parse_input(format!("{}\n", input).as_str()).unwrap();
    let mut rope = Rope::default(10);
    moves.iter().for_each(|m| rope.move_many(m));
    rope.start_history.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 13, "Failed example 1");
    }
    
    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 1, "Failed example 2");
    }
//
    #[test]
    fn test_part_2b() {
        let input="R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(process_part_2(input), 36, "Failed example 2");
    }
}

