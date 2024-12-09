use std::collections::HashSet;


pub fn parse_input(input: &str) -> Grid {
    let mut guard_x: Option<usize> = None;
    let mut guard_y: Option<usize> = None; 
    let mut guard_facing: Option<Direction> = None;
    let width = input.lines().map(|line| line.len()).max().unwrap() as i32;
    let height = input.lines().count() as i32;
    let obstacles = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(|(x, c)| {
            // Get only '#' characters
            // If we find one of '^<>V', then we have found the starting point
            if c == '#' {
                Some((x as i32, y as i32))
            } else if c == '^' {
                guard_x = Some(x);
                guard_y = Some(y);
                guard_facing = Some(Direction::Up);
                None
            } else if c == '<' {
                guard_x = Some(x);
                guard_y = Some(y);
                guard_facing = Some(Direction::Left);
                None
            } else if c == '>' {
                guard_x = Some(x);
                guard_y = Some(y);
                guard_facing = Some(Direction::Right);
                None
            } else if c == 'V' {
                guard_x = Some(x);
                guard_y = Some(y);
                guard_facing = Some(Direction::Down);
                None
            } else {
                None
            }
        }).collect::<Vec<(i32, i32)>>()
    }).collect::<HashSet<(i32, i32)>>();
    let x = guard_x.unwrap();
    let y = guard_y.unwrap();
    let facing = guard_facing.unwrap();
    let mut visited = HashSet::new();
    let guard = Guard {
        x: x as i32,
        y: y as i32,
        facing: facing.clone(),
    };
    visited.insert(guard.clone());
    Grid {
        width,
        height,
        obstacles,
        guard: Guard {
            x: x as i32,
            y: y as i32,
            facing,
        },
        visited,
    }
}

#[derive(Clone, PartialEq)]
pub struct Grid {
    width: i32,
    height: i32,
    obstacles: HashSet<(i32, i32)>,
    guard: Guard,
    visited: HashSet<Guard>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Guard {
    x: i32,
    y: i32,
    facing: Direction,
}


#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Grid {
    fn take_step(&mut self) -> bool {
        let (dx, dy) = match self.guard.facing {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let new_x = self.guard.x + dx;
        let new_y = self.guard.y + dy;
        if !self.obstacles.contains(&(new_x, new_y)) {
            self.guard.x = new_x;
            self.guard.y = new_y;
            if !self.visited.contains(&self.guard) {
                self.visited.insert(self.guard.clone());
                return false;
            }
            return true; 
        } else {
            // Turn right
            self.guard.facing = match self.guard.facing {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            return self.take_step();
        }
    }

    fn is_outside(&self) -> bool {
        self.guard.x == 0 || self.guard.x == self.width - 1 || self.guard.y == 0 || self.guard.y == self.height - 1
    }


    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut grid = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.guard.x == x && self.guard.y == y {
                    let c = match self.guard.facing {
                        Direction::Up => '^',
                        Direction::Down => 'V',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    };
                    grid.push(c);
                } else if self.obstacles.contains(&(x, y)) {
                    grid.push('#');
                } else {
                    grid.push('.');
                }
            }
            grid.push('\n');
        }
        grid
    }

    #[allow(dead_code)]
    fn to_string_with_obstacle(&self, ox: i32, oy: i32) -> String {
        let mut grid = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.guard.x == x && self.guard.y == y {
                    let c = match self.guard.facing {
                        Direction::Up => '^',
                        Direction::Down => 'V',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    };
                    grid.push(c);
                } else if ox == x && oy == y {
                    grid.push('O');
                } else if self.obstacles.contains(&(x, y)) {
                    grid.push('#');
                } else {
                    grid.push('.');
                }
            }
            grid.push('\n');
        }
        grid
    }

}


pub fn process_part_1(input: &str) -> u32 {
    let mut grid = parse_input(input);
    does_loop(&mut grid);
    grid.visited.iter().map(|guard| (guard.x, guard.y)).collect::<HashSet<(i32, i32)>>().len() as u32
}

pub fn does_loop(grid: &mut Grid) -> bool {
    while !grid.is_outside() {
        if grid.take_step() {
            return true;
        }
    }
    false
}

pub fn process_part_2(input: &str) -> u32{
    let initial_grid = parse_input(input);
    let mut test_grid = initial_grid.clone();
    does_loop(&mut test_grid);
    test_grid.visited.iter().filter_map(|guard| {
        let mut check_grid = initial_grid.clone();
        check_grid.obstacles.insert((guard.x, guard.y));
        if does_loop(&mut check_grid){
            Some((guard.x, guard.y))
        } else {
            None
        }
    }).collect::<HashSet<(i32, i32)>>().len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 41, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 6, "Failed example 2");
    }
}

