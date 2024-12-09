fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}


pub fn process_part_1(input: &str) -> u32 {
    let grid = parse_input(input);
    let mut count = 0;
    // Per line
    // Check if there is XMAS forward or backward per row
    count += grid.iter().map(|row| {
        let n = row.windows(4).filter(|window| **window == ['X', 'M', 'A', 'S'] || **window == ['S', 'A', 'M', 'X']).count();
        n as u32
    }).sum::<u32>();
    // Per column
    // Check if there is XMAS forward or backward per column
    count += grid.iter().enumerate().map(|(i, _)| {
        let n = grid.windows(4).filter(|window| {
            window.iter().map(|row| row[i]).collect::<Vec<char>>() == ['X', 'M', 'A', 'S'] || window.iter().map(|row| row[i]).collect::<Vec<char>>() == ['S', 'A', 'M', 'X']
        }).count();
        n as u32
    }).sum::<u32>();
    // Per diagonal
    // Check if there is XMAS forward or backward per diagonal
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !(i + 4 > grid.len() || j + 4 > grid[i].len()) {
                // Make forward diagonal
                let forward_diagonal = (0..4).map(|k| grid[i + k][j + k]).collect::<Vec<char>>();
                if forward_diagonal == ['X', 'M', 'A', 'S'] || forward_diagonal == ['S', 'A', 'M', 'X'] {
                    count += 1;
                }
            }
            if !(i + 4 > grid.len() || j < 3) {
                // Make backward diagonal
                let backward_diagonal = (0..4).map(|k| grid[i + k][j - k]).collect::<Vec<char>>();
                if backward_diagonal == ['X', 'M', 'A', 'S'] || backward_diagonal == ['S', 'A', 'M', 'X'] {
                    count += 1;
                }
            }
        }
    }

    count
}


pub fn process_part_2(input: &str) -> u32{
    let grid = parse_input(input);
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = grid[i][j];
            if c != 'A' {
                continue;
            }
            let nw = if i > 0 && j > 0 { grid[i - 1][j - 1] } else { ' ' };
            let n = if i > 0 { grid[i - 1][j] } else { ' ' };
            let ne = if i > 0 && j + 1 < grid[i].len() { grid[i - 1][j + 1] } else { ' ' };
            let w = if j > 0 { grid[i][j - 1] } else { ' ' };
            let e = if j + 1 < grid[i].len() { grid[i][j + 1] } else { ' ' };
            let sw = if i + 1 < grid.len() && j > 0 { grid[i + 1][j - 1] } else { ' ' };
            let s = if i + 1 < grid.len() { grid[i + 1][j] } else { ' ' };
            let se = if i + 1 < grid.len() && j + 1 < grid[i].len() { grid[i + 1][j + 1] } else { ' ' };

            let ns_mas = (n == 'M' && s == 'S') || (s == 'M' && n == 'S');
            let ew_mas = (e == 'M' && w == 'S') || (w == 'M' && e == 'S');
            let nw_se_mas = (nw == 'M' && se == 'S') || (se == 'M' && nw == 'S');
            let ne_sw_mas = (ne == 'M' && sw == 'S') || (sw == 'M' && ne == 'S');

            if nw_se_mas && ne_sw_mas {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 18, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 9, "Failed example 2");
    }
}

