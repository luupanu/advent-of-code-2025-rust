use std::collections::{HashMap, HashSet};

use advent_of_code_2025::read_input_lines;
use anyhow::{Result, anyhow, bail};

fn main() -> Result<()> {
    let lines = read_input_lines(7)?;
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    println!("Part 1: {}", solve_part1(&grid)?);
    println!("Part 2: {}", solve_part2(&grid)?);

    Ok(())
}

fn solve_part1(grid: &Vec<Vec<char>>) -> Result<String> {
    let mut num_splits: usize = 0;

    let mut beam_pos: HashSet<usize> = HashSet::new();
    let start_x = grid[0]
        .iter()
        .position(|char| *char == 'S')
        .ok_or(anyhow!("Start not found"))?;
    beam_pos.insert(start_x);

    for y in 1..grid.len() {
        let mut new_pos = Vec::new();

        for x in beam_pos.drain() {
            match grid[y][x] {
                '^' => {
                    if let Some(left_x) = x.checked_sub(1) {
                        new_pos.push(left_x);
                    }
                    if x + 1 < grid[y].len() {
                        new_pos.push(x + 1);
                    }
                    num_splits += 1;
                }
                '.' => {
                    new_pos.push(x);
                }
                _ => bail!("Unknown character {}", grid[y][x]),
            }
        }
        beam_pos.extend(new_pos);
    }

    Ok(num_splits.to_string())
}

fn solve_part2(grid: &Vec<Vec<char>>) -> Result<String> {
    // key: x-pos, value: number of distinct paths that reached it
    let mut paths: HashMap<usize, usize> = HashMap::new();
    let start_x = grid[0]
        .iter()
        .position(|char| *char == 'S')
        .ok_or(anyhow!("Start not found"))?;
    paths.insert(start_x, 1);

    for y in 1..grid.len() {
        let mut new_paths: HashMap<usize, usize> = HashMap::new();

        for (&x, &count) in &paths {
            match grid[y][x] {
                '^' => {
                    if let Some(left_x) = x.checked_sub(1) {
                        *new_paths.entry(left_x).or_insert(0) += count;
                    }
                    if x + 1 < grid[y].len() {
                        *new_paths.entry(x + 1).or_insert(0) += count;
                    }
                }
                '.' => {
                    *new_paths.entry(x).or_insert(0) += count;
                }
                _ => bail!("Unknown character {}", grid[y][x]),
            }
        }

        paths = new_paths;
    }

    Ok(paths.values().sum::<usize>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_counts_correctly() {
        let lines = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];
        let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
        assert_eq!(solve_part1(&grid).unwrap(), "21");
    }

    #[test]
    fn test_solve_part2_counts_correctly() {
        let lines = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];
        let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
        assert_eq!(solve_part2(&grid).unwrap(), "40");
    }
}
