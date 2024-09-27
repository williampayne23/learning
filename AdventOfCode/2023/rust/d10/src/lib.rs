use std::collections::HashMap;

use nom::IResult;

#[derive(Clone)]
struct Pipe {
    direction: char,
    connections: Vec<(i32, i32)>,
}

impl Pipe {
    fn new(direction: char, x: i32, y:i32) -> Pipe {
        match direction {
            '|'=> Pipe {
                direction: '│',
                connections: vec![(x, y - 1), (x, y + 1)],
            },
            '-'=> Pipe {
                direction: '─',
                connections: vec![(x - 1, y), (x + 1, y)],
            },
            'L'=> Pipe {
                direction: '└',
                connections: vec![(x, y - 1), (x + 1, y)],
            },
            'J'=> Pipe {
                direction: '┘',
                connections: vec![(x, y - 1), (x - 1, y)],
            },
            'F'=> Pipe {
                direction: '┌',
                connections: vec![(x, y + 1), (x + 1, y)],
            },
            'S'=> Pipe {
                direction,
                connections: vec![(x, y + 1), (x + 1, y), (x, y - 1), (x - 1, y)],
            },
            '7'=> Pipe {
                direction: '┐',
                connections: vec![(x, y + 1), (x - 1, y)],
            },
            '.'=> Pipe {
                direction: ' ',
                connections: vec![],
            },
            _ => panic!("Invalid pipe direction"),
        }
    }
}


fn parse_input(input: &str) -> IResult<&str, HashMap<(i32, i32), Pipe>> {
    let mut map = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        for c in line.chars() {
            map.insert((x, y), Pipe::new(c, x, y));
            x += 1;
        }
        y += 1;
        x = 0;
    }
    Ok((input, map))
}

fn draw_map(map: &HashMap<(i32, i32), Pipe>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in map.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match map.get(&(x, y)) {
                Some(pipe) => print!("{}", pipe.direction),
                None => print!("."),
            }
        }
        println!();
    }
}

fn get_interior_points(map: &HashMap<(i32, i32), Pipe>) -> Vec<(i32, i32)> {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in map.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    let mut interior_points = vec![];
    let mut vertical_count: HashMap<i32, u32> = HashMap::new();
    for y in min_y..=max_y {
        let mut horizontal_count = 0;
        for x in min_x..=max_x {
            match map.get(&(x, y)) {
                Some(pipe) => {
                    if pipe.direction == '│' || pipe.direction == '└' || pipe.direction == '┘' {
                        horizontal_count += 1;
                    }
                    if pipe.direction == '─' || pipe.direction == '┌' || pipe.direction == '└' {
                        vertical_count.entry(x).and_modify(|c| {
                            *c += 1;
                        }).or_insert(1);
                    }
                    print!("{}", pipe.direction);
                },
                None => {
                    if horizontal_count % 2 == 1 && vertical_count.get(&x).unwrap_or(&0) % 2 == 1 {
                        interior_points.push((x, y));
                        print!("I");
                    } else {
                        print!(".");
                    }
                },
            }
        }
        println!();
    }
    interior_points
}

fn get_start_connections(map: &HashMap<(i32, i32), Pipe>, x: i32, y: i32) -> Vec<(i32, i32)> {
    let connections = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]; 
    connections.into_iter().filter(|(cx, cy)| {
        match map.get(&(*cx, *cy)) {
            Some(pipe) => pipe.connections.contains(&(x, y)),
            None => false,
        }
    }).collect::<Vec<_>>()
}

pub fn process_part_1(input: &str) -> u32 {
    let (_, map) = parse_input(input).unwrap();
    draw_map(&map);
    let ((sx, sy), _) = map.iter().find(|(_, pipe)| pipe.direction == 'S').unwrap(); 
    let mut x = *sx;
    let mut y = *sy;
    let mut loop_map: HashMap<(i32,i32), (Pipe, u32)> = HashMap::new();
    let connections = get_start_connections(&map, x, y);
    let start = Pipe {
        direction: 'S',
        connections,
    };
    let (mut nextx, mut nexty) = start.connections[0];
    loop_map.insert((x, y), (start, 1));
    let mut dist = 1;
    while let Some(pipe) = map.get(&(nextx, nexty)) {
        if pipe.direction == ' ' || pipe.direction == 'S' {
            break;
        }
        loop_map.insert((nextx, nexty), (pipe.clone(), dist));
        dist += 1;
        let (nx, ny) = pipe.connections.iter().find(|(cx, cy)| {
            cx != &x || cy != &y
        }).unwrap();
        x = nextx;
        y = nexty;
        nextx = *nx;
        nexty = *ny;
    }
    //Loop other direction
    let mut x = *sx;
    let mut y = *sy;
    let mut dist = 1;
    let connections = get_start_connections(&map, x, y);
    let (mut nextx, mut nexty) = connections[1];
    while let Some(pipe) = map.get(&(nextx, nexty)) {
        if pipe.direction == ' ' || pipe.direction == 'S' {
            break;
        }
        loop_map.entry((nextx, nexty)).and_modify(|(_, d)| {
            *d = dist.min(*d);
        });
        dist += 1;
        let (nx, ny) = pipe.connections.iter().find(|(cx, cy)| {
            cx != &x || cy != &y
        }).unwrap();
        x = nextx;
        y = nexty;
        nextx = *nx;
        nexty = *ny;
    }

    let drawable_loop_map = loop_map.iter().map(|((x, y), (pipe, _))| {
        ((*x, *y), pipe.clone())
    }).collect::<HashMap<_, _>>();
    draw_map(&drawable_loop_map);

    loop_map.iter().map(|((_, _), (_, dist))| {
        *dist
    }).max().unwrap()
}


pub fn process_part_2(input: &str) -> u32{
    let (_, map) = parse_input(input).unwrap();
    let ((sx, sy), _) = map.iter().find(|(_, pipe)| pipe.direction == 'S').unwrap(); 
    let mut x = *sx;
    let mut y = *sy;
    let mut loop_map: HashMap<(i32,i32), (Pipe, u32)> = HashMap::new();
    let connections = get_start_connections(&map, x, y);
    let start = Pipe {
        direction: 'S',
        connections,
    };
    let (mut nextx, mut nexty) = start.connections[0];
    loop_map.insert((x, y), (start, 1));
    let mut dist = 1;
    while let Some(pipe) = map.get(&(nextx, nexty)) {
        if pipe.direction == ' ' || pipe.direction == 'S' {
            break;
        }
        loop_map.insert((nextx, nexty), (pipe.clone(), dist));
        dist += 1;
        let (nx, ny) = pipe.connections.iter().find(|(cx, cy)| {
            cx != &x || cy != &y
        }).unwrap();
        x = nextx;
        y = nexty;
        nextx = *nx;
        nexty = *ny;
    }

    let drawable_loop_map = loop_map.iter().map(|((x, y), (pipe, _))| {
        ((*x, *y), pipe.clone())
    }).collect::<HashMap<_, _>>();
    get_interior_points(&drawable_loop_map).len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    const EXAMPLE_PUZZLE_INPUT_2: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 8, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT_2), 10, "Failed example 2");
    }
}

