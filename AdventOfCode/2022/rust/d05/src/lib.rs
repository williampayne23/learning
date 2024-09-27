fn get_stacks() -> Stacks {
    Stacks([
           vec![ '.' ],
           vec![ 'B', 'L', 'D', 'T', 'W', 'C', 'F', 'M' ],
           vec![ 'N', 'B', 'L'  ],
           vec![ 'J', 'C', 'H', 'T', 'L', 'V'  ],
           vec![ 'S', 'P', 'J', 'W'  ],
           vec![ 'Z', 'S', 'C', 'F', 'T', 'L', 'R'  ],
           vec![ 'W', 'D', 'G', 'B', 'H', 'N', 'Z'  ],
           vec![ 'F', 'M', 'S', 'P', 'V', 'G', 'C', 'N'  ],
           vec![ 'W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z'  ],
           vec![ 'R', 'P', 'M', 'L', 'H'  ],
    ])
}

struct Stacks([Vec<char>; 10]);

impl Stacks {
    pub fn move_crate(self: &mut Self, from: usize, to: usize) -> &mut Self {
        let moved_crate = self.0[from].pop();
        self.0[to].push(moved_crate.unwrap());
        self
    }

    pub fn move_crates(self: &mut Self, from: usize, to: usize, num: usize) -> &mut Self {
        let duplicate = self.0[from].clone();
        let (new_stack, moved_crates) = duplicate.split_at(self.0[from].len() - num);
        self.0[from] = new_stack.to_vec();
        self.0[to].extend(moved_crates.to_vec());
        self
    }
    
    pub fn process_move(self: &mut Self, m: Move) -> &mut Self {
        for _ in 0..m.count {
            self.move_crate(m.from, m.to);
        }
        return self;
    }


    pub fn process_move_new(self: &mut Self, m: Move) -> &mut Self {
        self.move_crates(m.from, m.to, m.count);
        return self;
    }

    pub fn print(self: &Self) {
        for stack in self.0.iter() {
            println!("{:?}", stack);
        }
    }
}

struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl Move {
    pub fn from_string(input: &str) -> Move {
        let [_, num, _, from, _, to] = input.split(" ").collect::<Vec<&str>>()[..] else {
            todo!()
        };
        Move {
            from: from.parse::<usize>().unwrap(),
            to: to.parse::<usize>().unwrap(),
            count: num.parse::<usize>().unwrap(),
        }
    }

}


pub fn process_part_1(input: &str) {
    let mut stacks = get_stacks();
    for line in input.lines() {
        stacks.process_move(Move::from_string(line));
    }
    stacks.print();
}

pub fn process_part_2(input: &str) {
    let mut stacks = get_stacks();
    for line in input.lines() {
        stacks.process_move_new(Move::from_string(line));
    }
    stacks.print();
    stacks.0.iter().for_each(|stack| {
        println!("{:?}", stack.last().unwrap());
    });
}
