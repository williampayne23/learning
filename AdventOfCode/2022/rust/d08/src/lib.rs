pub fn process_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn visible(input: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut out_grid = vec![vec![0; input[0].len()]; input.len()];
    let mut rd_max_column: Vec<Option<usize>> = vec![None; input[0].len()];
    let mut lu_max_column: Vec<Option<usize>> = vec![None; input[0].len()];
    for (i, row) in input.iter().enumerate() {
        let mut rd_max_row: Option<usize> = None;
        let mut lu_max_row: Option<usize> = None;
        for (j, _) in row.iter().enumerate() {
            let rd_v = &input[i][j];
            if rd_max_row.is_none() || rd_v > &rd_max_row.unwrap() {
                rd_max_row = Some(*rd_v);
                out_grid[i][j] = 1;
            }
            if rd_max_column[j].is_none() || rd_v > &rd_max_column[j].unwrap() {
                rd_max_column[j] = Some(*rd_v);
                out_grid[i][j] = 1;
            }

            let lu_i = input.len() - i - 1;
            let lu_j = input[0].len() - j - 1;
            let lu_v = &input[lu_i][lu_j];
            if lu_max_row.is_none() || lu_v > &lu_max_row.unwrap() {
                lu_max_row = Some(*lu_v);
                out_grid[lu_i][lu_j] = 1;
            }
            if lu_max_column[lu_j].is_none() || lu_v > &lu_max_column[lu_j].unwrap() {
                lu_max_column[lu_j] = Some(*lu_v);
                out_grid[lu_i][lu_j] = 1;
            }
        }
    }

    return out_grid;
}

#[allow(dead_code)]
fn pretty_print(input: &Vec<Vec<usize>>) {
    for row in input {
        for v in row {
            print!("{} ", v);
        }
        println!();
    }
}

pub fn process_part_1(input: &str) -> usize {
    let input = process_input(input);
    let out_grid = visible(input.clone());
    out_grid.iter().map(|row| row.iter().sum::<usize>()).sum()
}

#[derive(Debug)]
struct Scores {
    i: usize,
    j: usize,
    v: usize,
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

fn scenic_scores(i: usize, j: usize, input: &Vec<Vec<usize>>) -> Scores {
    let mut left_score = 0;
    let mut right_score = 0;
    let mut up_score = 0;
    let mut down_score = 0;
    let mut stop_left = false;
    let mut stop_right = false;
    let mut stop_up = false;
    let mut stop_down = false;

    let val = input[i][j];
    for n in 1..input.len() {
        if n > i {
            stop_up = true;
        }
        let down = i + n;
        if down > input.len() - 1 {
            stop_down = true;
        }
        if n > j {
            stop_left = true;
        }
        let right = j + n;
        if right > input[0].len() - 1 {
            stop_right = true;
        }
        if !stop_left {
            let left = j - n;
            left_score = n;
            if input[i][left] >= val || left == 0 {
                stop_left = true;
            }
        }
        if !stop_right {
            right_score = n;
            if right >= input[0].len() - 1 || input[i][right] >= val {
                stop_right = true;
            }
        }
        if !stop_up {
            let up = i - n;
            up_score = n;
            if input[up][j] >= val || up == 0 {
                stop_up = true;
            }
        }
        if !stop_down {
            down_score = n;
            if down >= input.len() - 1 || input[down][j] >= val {
                stop_down = true;
            }
        }
        if stop_left && stop_right && stop_up && stop_down {
            break;
        }
    }
    // println!(
    //     "{}: {} {} {} {}",
    //     val, left_score, right_score, up_score, down_score
    // );
    #[allow(dead_code)]
    Scores {
        #[allow(dead_code)]
        i,
        #[allow(dead_code)]
        j,
        #[allow(dead_code)]
        v: val,
        left: left_score,
        right: right_score,
        up: up_score,
        down: down_score,
    }
}

pub fn scenic_score(i: usize, j: usize, input: &Vec<Vec<usize>>) -> usize {
    let scores = scenic_scores(i, j, input);
    scores.left * scores.right * scores.up * scores.down
}

pub fn process_part_2(input: &str) -> usize {
    let input = process_input(input);
    scenic_score(1, 2, &input);
    let mut scores = vec![vec![0; input[0].len()]; input.len()];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            scores[i][j] = scenic_score(i, j, &input);
        }
    }
    let max_score = scores
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap();
    *max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 21, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 8, "Failed example 2");
    }
}
