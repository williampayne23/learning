use std::collections::{HashMap, HashSet};

use nom::IResult;

fn parse_single_digit(input: &str) -> IResult<&str, char> {
    nom::character::complete::one_of("0123456789")(input)
}


pub fn parse_single_digit_as_u32(input: &str) -> IResult<&str, u32> {
    let (input, digit) = parse_single_digit(input)?;
    Ok((input, digit.to_digit(10).unwrap()))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, digs) = nom::multi::many1(parse_single_digit_as_u32)(input)?;
    Ok((input, digs))
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum File {
    Space,
    File(u64),
}

struct SizedFile {
    size: u64,
    file: File,
}

impl File {
    fn is_space(&self) -> bool {
        match self {
            File::Space => true,
            _ => false,
        }
    }

    fn value(&self) -> u64 {
        match self {
            File::File(f) => *f,
            File::Space => panic!("Cannot get value of space"),
        }
    }
}

pub fn process_part_1(input: &str) -> u64 {
    let (_, nums) = parse_input(input).unwrap();
    let mut drive = nums.iter().enumerate().map(|(i, n)| {
        let is_space = i % 2 == 1;
        if is_space {
            vec![File::Space].repeat(*n as usize)
        } else {
            let file = i as u64 / 2;
            vec![File::File(file)].repeat(*n as usize)
        }
    }).flatten().collect::<Vec<_>>();
    // Do swaps
    let mut l = 0;
    let mut r = drive.len() - 1;
    while l < r {
        while l < r && !drive[l].is_space() {
            l += 1;
        }
        while l < r && drive[r].is_space() {
            r -= 1;
        }
        // l is on a space, r is on a file
        if l < r {
            drive.swap(l, r);
        }
    }
    drive.iter().enumerate().fold(0, |acc, (i, c)| {
        if let File::File(f) = c {
            acc + (f * i as u64)
        } else {
            acc
        }
    })
}

struct SizedFileSystem {
    files: Vec<SizedFile>,
    first_space_of_size: HashMap<u64, usize>,
}


impl SizedFileSystem {
    fn new(files: Vec<SizedFile>) -> Self {
        SizedFileSystem {
            files,
            first_space_of_size: HashMap::new(),
        }
    }

    fn combine_spaces(&mut self) {
        let mut i = 0;
        while i < self.files.len() - 1 {
            if self.files[i].file.is_space() && self.files[i + 1].file.is_space() {
                self.files[i].size += self.files[i + 1].size;
                self.files.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    fn insert_file(&mut self, from: usize, to: usize) -> Result<(), &str> {
        let from_size = self.files[from].size;
        let to_size = self.files[to as usize].size;
        if from_size > to_size {
            return Err("Cannot move file to smaller space");
        }

        if self.files[from].file.is_space() {
            return Err("Cannot move space");
        }

        if !self.files[to].file.is_space() {
            return Err("Cannot move to non space");
        }
        
        self.files.swap(from, to);
        self.files[from].size = from_size;
        self.files.insert(to + 1, SizedFile { size: to_size - from_size, file: File::Space });
        self.combine_spaces();
        self.calculate_first_space_of_size();
        Ok(())
    }

    fn to_legacy_files(&self) -> Vec<File> {
        self.files.iter().map(|f| {
            vec![f.file].repeat(f.size as usize)
        }).flatten().collect()
    }
    fn calculate_checksum(&self) -> u64 {
        let drive = self.to_legacy_files();
        drive.iter().enumerate().fold(0, |acc, (i, c)| {
            if let File::File(f) = c {
                acc + (f * i as u64) 
            } else {
                acc
            }
        })
    }

    fn calculate_first_space_of_size(&mut self) {
        self.first_space_of_size.clear();
        for (i, file) in self.files.iter().enumerate() {
            if file.file.is_space() {
                if self.first_space_of_size.contains_key(&file.size) {
                    continue;
                }
                self.first_space_of_size.insert(file.size, i);
            }
        }
    }

    #[allow(dead_code)]
    fn visualise(&self) {
        for file in &self.files {
            match file.file {
                File::Space => print!("{}", ".".repeat(file.size as usize)),
                File::File(f) => print!("{}", f.to_string().repeat(file.size as usize)),
            }
        }
        println!();
    }

    fn get_first_space_larger_than(&self, size: u64) -> Option<usize> {
        self.first_space_of_size.iter().filter(|(s, _)| **s >= size).map(|(_, i)| *i).min()
    }
}


pub fn process_part_2(input: &str) -> u64{
    let (_, nums) = parse_input(input).unwrap();
    let drive = nums.iter().enumerate().map(|(i, n)| {
        let is_space = i % 2 == 1;
        if is_space {
            SizedFile { size: *n as u64, file: File::Space }
        } else {
            let file = i as u64 / 2;
            SizedFile { size: *n as u64, file: File::File(file) }
        }
    }).collect::<Vec<_>>();
    let mut file_system = SizedFileSystem::new(drive);
    file_system.calculate_first_space_of_size();
    let mut r = file_system.files.len() - 1;
    let mut visited = HashSet::new();
    while r > 0 {
        let file = &file_system.files[r];
        // file_system.visualise();
        // println!("Looking at {:?}", file.file);
        if file.file.is_space() {
            r -= 1;
            continue;
        }
        let size = file.size;
        let option_space_pos = file_system.get_first_space_larger_than(size);
        if visited.contains(&file.file.value()) {
            r -= 1;
            continue;
        }
        visited.insert(file.file.value());
        if let Some(space_pos) = option_space_pos {
            if space_pos >= r {
                r -= 1;
                continue;
            }
            let res = file_system.insert_file(r, space_pos);
            if let Err(e) = res {
                println!("Error: {}", e);
            }
        }
        r -= 1;
    }
    // file_system.visualise();
    file_system.calculate_checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 1928, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 2858, "Failed example 2");
    }
}

