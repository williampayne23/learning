use std::ops::RangeInclusive;

use rayon::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, (i32, i32, i32, i32)> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = complete::i32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, bx) = complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, by) = complete::i32(input)?;
    Ok((input, (x, y, bx, by)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(i32, i32, i32, i32)>> {
    let (input, lines) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, lines))
}

struct Space {
    x: i32,
    y: i32,
    manhattan_distance: i32,
}

impl Space {
    fn new(x: i32, y: i32, manhattan_distance: i32) -> Space {
        Space {
            x,
            y,
            manhattan_distance,
        }
    }
    fn within(&self, x: i32, y: i32) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.manhattan_distance
    }

    fn line_overlaps(&self, y: i32) -> bool {
        self.within(self.x, y)
    }

    fn overlapping_range(&self, y: i32) -> RangeInclusive<i32> {
        if !self.line_overlaps(y) {
            panic!("Space does not overlap line");
        }
        let manhattan_distance = self.manhattan_distance - (self.y - y).abs();
        self.x - manhattan_distance..=self.x + manhattan_distance
    }

    // fn quad_within(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    //     self.within(x1, y1) && self.within(x2, y2) && self.within(x1, y2) && self.within(x2, y1)
    // }
}

fn count_line(spaces: Vec<Space>, beacons: Vec<(i32, i32)>, line: i32) -> u32 {
    let min_x = spaces
        .iter()
        .map(|s| s.x - s.manhattan_distance)
        .min()
        .unwrap();
    let max_x = spaces
        .iter()
        .map(|s| s.x + s.manhattan_distance)
        .max()
        .unwrap();
    let mut count = 0;
    for x in min_x..=max_x {
        for space in &spaces {
            if space.within(x, line) {
                count += 1;
                break;
            }
        }
        for beacon in &beacons {
            if beacon.0 == x && beacon.1 == line {
                count -= 1;
                break;
            }
        }
    }
    count
}

pub fn process_part_1(input: &str, line: i32) -> u32 {
    let (_, lines) = parse_input(input).unwrap();
    let spaces = lines
        .iter()
        .map(|(x, y, bx, by)| {
            let manhattan_distance = (bx - x).abs() + (by - y).abs();
            Space::new(*x, *y, manhattan_distance)
        })
        .collect::<Vec<Space>>();
    let beacons = lines
        .iter()
        .map(|(_, _, bx, by)| (*bx, *by))
        .collect::<Vec<(i32, i32)>>();
    let _sensors = lines
        .iter()
        .map(|(x, y, _, _)| (*x, *y))
        .collect::<Vec<(i32, i32)>>();
    // draw_grid(&grid);
    count_line(spaces, beacons, line)
}

// fn split_quadrant(quad: (i32, i32, i32, i32)) -> Vec<(i32, i32, i32, i32)> {
//     let mut sub_quadrants = Vec::new();
//
//     let xmid = (quad.0 + quad.2) / 2;
//     let ymid = (quad.1 + quad.3) / 2;
//
//     if quad.2 - quad.0 > 0 && quad.3 - quad.1 > 0 {
//         sub_quadrants.push((quad.0, quad.1, xmid, ymid));
//         sub_quadrants.push((xmid + 1, quad.1, quad.2, ymid));
//         sub_quadrants.push((quad.0, ymid + 1, xmid, quad.3));
//         sub_quadrants.push((xmid + 1, ymid + 1, quad.2, quad.3));
//     } else if quad.2 - quad.0 > 0 {
//         sub_quadrants.push((quad.0, quad.1, xmid, quad.3));
//         sub_quadrants.push((xmid + 1, quad.1, quad.2, quad.3));
//     } else if quad.3 - quad.1 > 0 {
//         sub_quadrants.push((quad.0, quad.1, quad.2, ymid));
//         sub_quadrants.push((quad.0, ymid + 1, quad.2, quad.3));
//     } else {
//         sub_quadrants.push((quad.0, quad.1, quad.2, quad.3));
//     }
//
//     sub_quadrants
// }

// fn part_2_quad_attempt(input: &str, max_line: i32) -> u32 {
//     let (_, lines) = parse_input(input).unwrap();
//     let spaces = lines
//         .iter()
//         .map(|(x, y, bx, by)| {
//             let manhattan_distance = (bx - x).abs() + (by - y).abs();
//             Space::new(*x, *y, manhattan_distance)
//         })
//         .collect::<Vec<Space>>();
//     let mut quad_stack = vec![(0, 0, max_line, max_line)];
//     let mut n = 0;
//     while let Some(quad) = quad_stack.pop() {
//         println!("{} {:?} ", n, quad_stack.len());
//         n += 1;
//         // if n > 200 {
//         //     break;
//         // }
//         if quad.0 == quad.2 && quad.1 == quad.3 {
//             if !spaces.iter().any(|s| s.within(quad.0, quad.1)) {
//                 return quad.0 as u32 * 4000000 + quad.1 as u32;
//             }
//             continue;
//         }
//
//         if spaces
//             .iter()
//             .all(|s| s.quad_within(quad.0, quad.1, quad.2, quad.3))
//         {
//             continue;
//         }
//
//         let new_quad = split_quadrant(quad);
//         quad_stack.extend(new_quad);
//     }
//     1
// }

trait RangeInclusiveExt {
    fn overlaps(&self, other: &RangeInclusive<i32>) -> bool;
    fn adjacent(&self, other: &RangeInclusive<i32>) -> bool;
    fn merge(&self, other: &RangeInclusive<i32>) -> RangeInclusive<i32>;
}

impl RangeInclusiveExt for RangeInclusive<i32> {
    fn overlaps(&self, other: &RangeInclusive<i32>) -> bool {
        self.contains(other.start())
            || self.contains(other.end())
            || other.contains(self.start())
            || other.contains(self.end())
    }

    fn adjacent(&self, other: &RangeInclusive<i32>) -> bool {
        self.end() + 1 == *other.start() || other.end() + 1 == *self.start()
    }

    fn merge(&self, other: &RangeInclusive<i32>) -> RangeInclusive<i32> {
        *self.start().min(other.start())..=*self.end().max(other.end())
    }
}

pub fn process_part_2(input: &str, max_line: i32) -> u64 {
    let (_, lines) = parse_input(input).unwrap();
    let spaces = lines
        .iter()
        .map(|(x, y, bx, by)| {
            let manhattan_distance = (bx - x).abs() + (by - y).abs();
            Space::new(*x, *y, manhattan_distance)
        })
        .collect::<Vec<Space>>();

    (0..=max_line)
        .into_par_iter()
        .map(|line| {
            (line, spaces
                .iter()
                .filter_map(|s| {
                    if s.line_overlaps(line) {
                        Some(s.overlapping_range(line))
                    } else {
                        None
                    }
                })
                .collect::<Vec<RangeInclusive<i32>>>())
        }).map(|(y, line)| {
            let mut merged_list: Vec<RangeInclusive<i32>>= vec![];
            let mut unmerged: Vec<RangeInclusive<i32>> = line.iter().map(|r| r.clone()).collect();
            while let Some(range) = unmerged.pop() {
                let mut did_merge = false;
                for i in 0..merged_list.len() {
                    if merged_list[i].overlaps(&range) || merged_list[i].adjacent(&range) {
                        did_merge = true;
                        let merged = merged_list[i].merge(&range);
                        merged_list = [&merged_list[..i], &merged_list[i + 1..]].concat();
                        unmerged.push(merged.clone());
                        break;
                    }
                }
                if !did_merge {
                    merged_list.push(range.clone());
                }
            }
            (y, merged_list)
        })
        .find_map_any(|(line, ranges)| {
            if ranges.len() > 1 {
                let mut ns = vec![ranges[0].end(), ranges[1].start(), ranges[1].end(), ranges[0].start()];
                ns.sort();
                let x = (ns[1] + ns[2]) / 2;
                return Some(x as u64 * 4000000 + line as u64);
            }
            None
        }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part_1() {
        assert_eq!(
            process_part_1(EXAMPLE_PUZZLE_INPUT, 10),
            26,
            "Failed example 1"
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            process_part_2(EXAMPLE_PUZZLE_INPUT, 20),
            56000011,
            "Failed example 2"
        );
    }
}
