use advent_of_code_2025::{read_input_lines};
use anyhow::{Result};

fn main() -> Result<()> {
    let lines = read_input_lines(4)?;
    let grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();

    println!("Part 1: {}", solve_part1(&grid)?);
    println!("Part 2: {}", solve_part2(&mut grid.clone())?);

    Ok(())
}

const NEIGHBOURING_OFFSETS: [(isize, isize);8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn solve_part1(grid: &Vec<Vec<char>>) -> Result<String> {
    let mut sum: usize = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '.' {
                continue;
            }
            if find_neighbours(x, y, grid) < 4 {
                sum += 1;
            }
        }
    }

    Ok(sum.to_string())
}

fn solve_part2(grid: &mut Vec<Vec<char>>) -> Result<String> {
    let mut sum: usize = 0;
    let mut removed: bool = true;

    while removed {
        removed = false;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '.' {
                    continue;
                }
                if find_neighbours(x, y, grid) < 4 {
                    sum += 1;
                    removed = true;
                    grid[y][x] = '.';
                }
            }
        }
    }

    Ok(sum.to_string())
}

fn find_neighbours(x: usize, y: usize, grid: &Vec<Vec<char>>) -> usize {
    let mut num_of_neighbours: usize = 0;

    for (dx, dy) in NEIGHBOURING_OFFSETS {
        if let (Some(nx), Some(ny)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
            if ny < grid.len() && nx < grid[ny].len() && grid[ny][nx] == '@' {
                num_of_neighbours += 1;
            }
        }
    }

    num_of_neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_counts_correctly() {
        let lines = vec![
            "..@@.@@@@.".to_string(),
            "@@@.@.@.@@".to_string(),
            "@@@@@.@.@@".to_string(),
            "@.@@@@..@.".to_string(),
            "@@.@@@@.@@".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "@.@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "@.@.@@@.@.".to_string(),
        ];
        let grid: Vec<Vec<char>> = lines.iter()
            .map(|line| line.chars().collect())
            .collect();

        assert_eq!(solve_part1(&grid).unwrap(), "13");
    }

    #[test]
    fn test_solve_part2_counts_correctly() {
        let lines = vec![
            "..@@.@@@@.".to_string(),
            "@@@.@.@.@@".to_string(),
            "@@@@@.@.@@".to_string(),
            "@.@@@@..@.".to_string(),
            "@@.@@@@.@@".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "@.@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "@.@.@@@.@.".to_string(),
        ];
        let mut grid: Vec<Vec<char>> = lines.iter()
            .map(|line| line.chars().collect())
            .collect();

        assert_eq!(solve_part2(&mut grid).unwrap(), "43");
    }
}
