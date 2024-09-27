enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum WLD {
    Win,
    Lose,
    Draw,
}

impl WLD {
    pub fn from_str(s: &str) -> WLD {
        match s {
            "X" => WLD::Lose,
            "Y" => WLD::Draw,
            "Z" => WLD::Win,
            _ => panic!("Invalid input"),
        }
    }

    pub fn play_game(their_move: RPS, goal: WLD) -> i32 {
        match (their_move, goal) {
            (RPS::Rock, WLD::Win) => 8,
            (RPS::Rock, WLD::Lose) => 3,
            (RPS::Rock, WLD::Draw) => 4,
            (RPS::Paper, WLD::Win) => 9,
            (RPS::Paper, WLD::Lose) => 1,
            (RPS::Paper, WLD::Draw) => 5,
            (RPS::Scissors, WLD::Win) => 7,
            (RPS::Scissors, WLD::Lose) => 2,
            (RPS::Scissors, WLD::Draw) => 6,
        }
    }
}

impl RPS {
    pub fn from_str(s: &str) -> RPS {
        match s {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!("Invalid input"),
        }
    }

    pub fn play_game(my_move: RPS, their_move: RPS) -> i32 {
        match (my_move, their_move) {
            (RPS::Rock, RPS::Rock) => 4,
            (RPS::Rock, RPS::Paper) => 1,
            (RPS::Rock, RPS::Scissors) => 7,
            (RPS::Paper, RPS::Rock) => 8,
            (RPS::Paper, RPS::Paper) => 5,
            (RPS::Paper, RPS::Scissors) => 2,
            (RPS::Scissors, RPS::Rock) => 3,
            (RPS::Scissors, RPS::Paper) => 9,
            (RPS::Scissors, RPS::Scissors) => 6,
        }
    }

}


pub fn process_part_1(input: &str) {
    let mut total = 0;
    for line in input.lines() {
        let [i, j] = line.split(" ").collect::<Vec<&str>>()[..] else {
            todo!()
        };
        let opponent_move = RPS::from_str(i);
        let recommended_move = RPS::from_str(j);

        total += RPS::play_game(recommended_move, opponent_move);
    }
    println!("{}", total);
}

pub fn process_part_2(input: &str) {
    let mut total = 0;
    for line in input.lines() {
        let [i, j] = line.split(" ").collect::<Vec<&str>>()[..] else {
            todo!()
        };
        let opponent_move = RPS::from_str(i);
        let recommended_state = WLD::from_str(j);

        total += WLD::play_game(opponent_move, recommended_state);
    }
    println!("{}", total);
}
