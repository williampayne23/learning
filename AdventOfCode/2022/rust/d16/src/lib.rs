use itertools::Itertools;
use nom::{IResult, bytes::complete::{tag, take_until}, character::{complete::{alpha1, self}}, multi::separated_list1, branch::alt};
use petgraph::{graph::UnGraph, algo};

#[derive(Debug)]
struct Valve {
    name: String,
    id: u32,
    flow_rate: u32,
    tunnels: Vec<u32>,
}

fn tunnel_name_to_index(name: &str) -> u32 {
    name.chars().enumerate().map(|(i, c)| (c as u32 - 'A' as u32) * (26 as u32).pow(i as u32)).collect::<Vec<_>>().iter().sum()
}

fn process_line(input: &str) -> IResult<&str, Valve> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = take_until(" has flow rate=")(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = complete::u32(input)?;
    let (input, _) = alt((tag("; tunnels lead to valves "), tag("; tunnel leads to valve ")))(input)?;
    let (input, tunnels) = separated_list1(tag(", "), alpha1)(input)?;
    Ok((input, Valve { name: name.to_string(), id: tunnel_name_to_index(name), flow_rate, tunnels: tunnels.iter().map(|s| tunnel_name_to_index(s)).collect()}))
}

fn process_input(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(tag("\n"), process_line)(input)
}

struct ValveGraph {
    valves: Vec<Valve>,
    graph: UnGraph::<u32, ()>,
}

impl ValveGraph {
    fn find_path(&self, start: u32, end: u32) -> Option<u32> {
        if let Some((cost, _)) = algo::astar(
            &self.graph,
            start.into(),               // start
            |n| n == end.into(),      // is_goal
            |_| 1, // edge_cost
            |_| 0,           // estimate_cost
        ) {
            Some(cost)
        } else {
            None
        }
    }

    fn get_paths(&self, start: u32) -> Vec<(u32, &Valve)> {
        self.valves.iter().filter_map(|v| {
            if v.id != start {
                if let Some(cost) = self.find_path(start, v.id) {
                    Some((cost, v))
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }

    fn get_best_paths(&self, node: u32, exclude: &Vec<u32>, time_left: u32, _n_paths: usize) -> Vec<(u32, &Valve, u32)> {
        let paths = self.get_paths(node);
        let paths = paths.into_iter().filter_map(|(cost, v)| {
            if cost + 1 <= time_left && !exclude.contains(&v.id) && v.flow_rate > 0 {
                let total = v.flow_rate * (time_left - cost - 1);
                Some((cost+1, v, total))
            } else {
                None
            }
        }).sorted_by(|a, b| b.2.cmp(&a.2)).collect::<Vec<(u32, &Valve, u32)>>();
        paths
    }
}

#[derive(Clone, PartialEq, Eq)]
struct PossiblePath {
    valves: Vec<u32>,
    time_left: u32,
    total: u32,
    no_more_paths: bool 
}

impl PossiblePath {
    fn next(&self, vg: &ValveGraph, n_paths: usize, exclude: Vec<u32>) -> Vec<PossiblePath> {
        let next_steps = vg.get_best_paths(*self.valves.last().unwrap(), &exclude, self.time_left, n_paths);
        if next_steps.is_empty() {
            let new_path = PossiblePath { valves: self.valves.clone(), time_left: 0, total: self.total, no_more_paths: true };
            return vec![new_path];
        }
        next_steps.into_iter().map(|(cost, valve, total)| {
            let mut valves = self.valves.clone();
            valves.push(valve.id);
            PossiblePath { valves, time_left: self.time_left - cost, total: self.total + total, no_more_paths: false }
        }).collect()
    }

    fn is_complete(&self) -> bool {
        self.time_left == 0 || self.no_more_paths
    }
}

fn generate(vg: &ValveGraph, paths: Vec<PossiblePath>, n_paths: usize) -> Vec<PossiblePath> {
    paths.into_iter().map(|path| path.next(vg, n_paths, path.valves.clone())).flatten().collect()
}

fn filter(paths: Vec<PossiblePath>, n_paths: usize) -> (Vec<PossiblePath>, Vec<PossiblePath>) {
    let paths = paths.into_iter().sorted_by(|a, b| b.total.cmp(&a.total)).collect::<Vec<PossiblePath>>();
    let ended_paths = paths.iter().filter(|path| path.is_complete()).map(|p| p.clone()).collect::<Vec<_>>();
    let active_paths = paths.into_iter().filter(|path| !path.is_complete()).take(n_paths).collect::<Vec<_>>();
    (active_paths, ended_paths)
}


fn generate2(vg: &ValveGraph, paths: Vec<(PossiblePath, PossiblePath)>, n_paths: usize) -> Vec<(PossiblePath, PossiblePath)> {
    paths.iter().map(|(path1, path2)| {
        let visited = path1.valves.iter().map(|&v| v).chain(path2.valves.iter().map(|&v| v)).collect::<Vec<_>>();
        let new_paths_a = if !path1.is_complete() {
            path1.next(vg, n_paths, visited.clone())
        } else {
            vec![PossiblePath { valves: path1.valves.clone(), time_left: path1.time_left, total: path1.total, no_more_paths: true }]
        };
        let new_paths_b = if !path2.is_complete() {
            path2.next(vg, n_paths, visited.clone())
        } else {
            vec![PossiblePath { valves: path2.valves.clone(), time_left: path2.time_left, total: path2.total, no_more_paths: true }]
        };
        new_paths_a.into_iter().cartesian_product(new_paths_b.into_iter()).filter(|(path_1, path2)| {
            path_1.valves.last().unwrap() != path2.valves.last().unwrap()
        }).collect::<Vec<_>>()
    }).flatten().collect()
}

fn filter2(paths: Vec<(PossiblePath, PossiblePath)>, n_paths: usize) -> (Vec<(PossiblePath, PossiblePath)>, Vec<(PossiblePath, PossiblePath)>) {
    let paths = paths.into_iter().sorted_by(|(a1, b1), (a2, b2)| (a2.total + b2.total).cmp(&(a1.total + b1.total)));
    let paths = paths.into_iter().fold(vec![], |mut acc, (path1, path2)| {
        let last = acc.last();
        if let Some((last_path1, last_path2)) = last {
            if last_path1 == &path2 && last_path2 == &path1 {
                acc
            } else {
                acc.push((path1, path2));
                acc
            }
        } else {
            acc.push((path1, path2));
            acc
        }
    });
    let ended_paths = paths.iter().filter(|(path1, path2)| path1.is_complete() && path2.is_complete()).map(|p| p.clone()).collect::<Vec<_>>();
    let active_paths = paths.into_iter().filter(|(path1, path2)| !path1.is_complete() || !path2.is_complete()).take(n_paths).collect::<Vec<_>>();
    (active_paths, ended_paths)
}

fn are_complete(paths: &Vec<PossiblePath>) -> bool {
    paths.iter().all(|path| path.is_complete())
}

pub fn process_part_1(input: &str) -> u32 {
    let (_, valves) = process_input(input).unwrap();
    let edges = valves.iter().map(|v| v.tunnels.iter().map(|tunnel| (v.id, tunnel.to_owned()))).flatten().collect::<Vec<_>>();
    let graph = UnGraph::<u32, ()>::from_edges(edges);
    let vg = ValveGraph { valves, graph };
    let mut possible_paths = vec![PossiblePath { valves: vec![0], time_left: 30, total: 0, no_more_paths: false }];
    let mut ended_paths = vec![];
    
    while !are_complete(&possible_paths) && possible_paths.len() > 0 {
        possible_paths = generate(&vg, possible_paths, 2000);
        let (new_possible_paths, new_ended) = filter(possible_paths, 2000);
        possible_paths = new_possible_paths;
        ended_paths.extend(new_ended);
    }
    possible_paths.extend(ended_paths);
    possible_paths.sort_by(|a, b| b.total.cmp(&a.total));
    possible_paths.iter().take(400).for_each(|path| {
        path.valves.iter().for_each(|v| print!("{} ", vg.valves.iter().find(|valve| valve.id == *v).unwrap().name));
        println!("{} {}", path.time_left, path.total);
    });
    possible_paths[0].total
}


pub fn process_part_2(input: &str) -> u32{
    let (_, valves) = process_input(input).unwrap();
    let edges = valves.iter().map(|v| v.tunnels.iter().map(|tunnel| (v.id, tunnel.to_owned()))).flatten().collect::<Vec<_>>();
    let graph = UnGraph::<u32, ()>::from_edges(edges);
    let vg = ValveGraph { valves, graph };
    let mut possible_paths = vec![
        (
            PossiblePath { valves: vec![0], time_left: 26, total: 0, no_more_paths: false },
            PossiblePath { valves: vec![0], time_left: 26, total: 0, no_more_paths: false }
        )];
    let mut ended_paths = vec![];
    
    while possible_paths.len() > 0 {
        possible_paths = generate2(&vg, possible_paths, 2000);
        let (new_possible_paths, new_ended) = filter2(possible_paths, 2000);
        possible_paths = new_possible_paths;
        ended_paths.extend(new_ended);
    }
    ended_paths.sort_by(|(a1, b1), (a2, b2)| (a2.total + b2.total).cmp(&(a1.total + b1.total)));
    ended_paths.iter().take(4).for_each(|(path1, path2)| {
        println!("{}", path1.total + path2.total);
        path1.valves.iter().for_each(|v| print!("{} ", vg.valves.iter().find(|valve| valve.id == *v).unwrap().name));
        println!("{} {}", path1.time_left, path1.total);
        path2.valves.iter().for_each(|v| print!("{} ", vg.valves.iter().find(|valve| valve.id == *v).unwrap().name));
        println!("{} {}", path2.time_left, path2.total);
        println!();
    });
    ended_paths[0].0.total + ended_paths[0].1.total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 1651, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 1707, "Failed example 2");
    }
}

