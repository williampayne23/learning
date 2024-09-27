struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let [start, end] = s.split("-").collect::<Vec<&str>>()[..] else {
            todo!()
        };
        Self {
            start: start.parse::<u32>().unwrap(),
            end: end.parse::<u32>().unwrap(),
        }
    }
    fn contains(self: &Self, other: &Range) -> bool {
        self.start <= other.start && other.end <= self.end
    }
    fn overlaps(self: &Self, other: &Range) -> bool {
        ( self.start >= other.start && self.start <= other.end )
            || ( self.end >= other.start && self.end <= other.end )
    }
}


pub fn process_part_1(input: &str) {
    let mut total = 0;
    for line in input.lines() {
        let [elf1, elf2] = line.split(",").collect::<Vec<&str>>()[..] else {
            todo!()
        };
        let range1 = Range::from_str(elf1);
        let range2 = Range::from_str(elf2);
        if range1.contains(&range2) || range2.contains(&range1) {
            total += 1;
        }
    }
    println!("{}", total);
}

pub fn process_part_2(input: &str) {
    let mut total = 0;
    for line in input.lines() {
        let [elf1, elf2] = line.split(",").collect::<Vec<&str>>()[..] else {
            todo!()
        };
        let range1 = Range::from_str(elf1);
        let range2 = Range::from_str(elf2);
        if range1.overlaps(&range2) || range1.contains(&range2) || range2.contains(&range1) {
            total += 1;
        }
    }
    println!("{}", total);
}
