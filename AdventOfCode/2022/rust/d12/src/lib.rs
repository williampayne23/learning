static ASCII_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
fn char_to_height(c: char) -> u32 {
    if c == 'E' {
        return 25;
    }
    if c == 'S' {
        return 0;
    }
    return ASCII_LOWER.find(c).unwrap() as u32;
}

fn find_char(grid: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == c {
                return (i, j);
            }
        }
    }
    return (0, 0);
}

fn find_min(lengths: &Vec<Vec<u32>>, visited: &Vec<Vec<bool>>) -> (usize, usize) {
    let mut min = u32::MAX;
    let mut min_i = 0;
    let mut min_j = 0;
    for i in 0..lengths.len() {
        for j in 0..lengths[0].len() {
            if !visited[i][j] && lengths[i][j] <= min {
                min = lengths[i][j];
                min_i = i;
                min_j = j;
            }
        }
    }
    return (min_i, min_j);
}

pub fn process_part_1(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect()).collect();
    grid.pop();
    let mut lengths = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let (i, j) = find_char(&grid, 'E');
    lengths[i][j] = 0;
    let (si, sj) = find_char(&grid, 'S');
    while !visited[si][sj] {
        let (i, j) = find_min(&lengths, &visited);
        visited[i][j] = true;
        let height = char_to_height(grid[i][j]);
        dbg!(i, j, height, lengths[i][j]);
        if i > 0
            && char_to_height(grid[i - 1][j]) + 1 >= height
            && lengths[i - 1][j] > lengths[i][j] + 1
        {
            lengths[i - 1][j] = lengths[i][j] + 1;
        }
        if i < grid.len() - 1
            && char_to_height(grid[i + 1][j]) + 1 >= height
            && lengths[i + 1][j] > lengths[i][j] + 1
        {
            lengths[i + 1][j] = lengths[i][j] + 1;
        }
        if j > 0
            && char_to_height(grid[i][j - 1]) + 1 >= height
            && lengths[i][j - 1] > lengths[i][j] + 1
        {
            lengths[i][j - 1] = lengths[i][j] + 1;
        }
        if j < grid[0].len() - 1
            && char_to_height(grid[i][j + 1]) + 1 >= height
            && lengths[i][j + 1] > lengths[i][j] + 1
        {
            lengths[i][j + 1] = lengths[i][j] + 1;
        }
    }
    lengths[si][sj]
}

pub fn process_part_2(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect()).collect();
    grid.pop();
    let mut lengths = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let (i, j) = find_char(&grid, 'E');
    lengths[i][j] = 0;
    let (si, sj) = find_char(&grid, 'S');
    while visited.iter().flatten().any(|x| !x) {
        let (i, j) = find_min(&lengths, &visited);
        visited[i][j] = true;
        if lengths[i][j] == u32::MAX {
            break
        }
        let height = char_to_height(grid[i][j]);
        dbg!(i, j, height, lengths[i][j]);
        if i > 0
            && char_to_height(grid[i - 1][j]) + 1 >= height
            && lengths[i - 1][j] > lengths[i][j] + 1
        {
            lengths[i - 1][j] = lengths[i][j] + 1;
        }
        if i < grid.len() - 1
            && char_to_height(grid[i + 1][j]) + 1 >= height
            && lengths[i + 1][j] > lengths[i][j] + 1
        {
            lengths[i + 1][j] = lengths[i][j] + 1;
        }
        if j > 0
            && char_to_height(grid[i][j - 1]) + 1 >= height
            && lengths[i][j - 1] > lengths[i][j] + 1
        {
            lengths[i][j - 1] = lengths[i][j] + 1;
        }
        if j < grid[0].len() - 1
            && char_to_height(grid[i][j + 1]) + 1 >= height
            && lengths[i][j + 1] > lengths[i][j] + 1
        {
            lengths[i][j + 1] = lengths[i][j] + 1;
        }
    }
    lengths.iter().enumerate().fold(u32::MAX, |acc, (i, x)| {
        x.iter().enumerate().fold(acc, |acc, (j, _)| {
            if (grid[i][j] == 'a' || grid[i][j] == 'S') && lengths[i][j] < acc {
                return lengths[i][j];
            }
            return acc;
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 31, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 29, "Failed example 2");
    }
}
