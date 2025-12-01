use advent_of_code_2025::read_input_lines;
use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let lines = read_input_lines(1)?;

    println!("Part 1: {}", solve_part1(&lines)?);
    println!("Part 2: {}", solve_part2(&lines)?);

    Ok(())
}

const DIAL_START: usize = 50;
const DIAL_LENGTH: usize = 100;
const DIAL_TARGET: usize = 0;

fn solve_part1(lines: &Vec<String>) -> Result<String> {
    let mut curr_dial = DIAL_START;
    let mut target_dial_count: usize = 0;

    for line in lines {
        rotate_part1(line, &mut curr_dial)?;
        if curr_dial == DIAL_TARGET {
            target_dial_count += 1;
        }
    }

    Ok(target_dial_count.to_string())
}

fn solve_part2(lines: &Vec<String>) -> Result<String> {
    let mut curr_dial = DIAL_START;
    let mut target_dial_count: usize = 0;

    for line in lines {
        rotate_part2(line, &mut curr_dial, &mut target_dial_count)?;
    }

    Ok(target_dial_count.to_string())
}


fn rotate_part1(line: &str, curr_dial: &mut usize) -> Result<()> {
    let direction = line.chars().nth(0)
        .with_context(|| format!("Should have direction in line: {}", line))?;
    let rotation = line
        .get(1..)
        .with_context(|| format!("Should have rotation in line: {}", line))?
        .parse::<usize>()
        .with_context(|| format!("Should be able to parse number in line: {}", line))?;

    match direction {
        'R' => {
            *curr_dial = (*curr_dial + rotation) % DIAL_LENGTH;
        }
        'L' => {
            // Add DIAL_LENGTH before subtracting and do mod DIAL_LENGTH to ensure the result is positive
            *curr_dial = (*curr_dial + DIAL_LENGTH - rotation % DIAL_LENGTH) % DIAL_LENGTH;
        }
        _ => {
            bail!("Unknown direction '{}' in line: {}", direction, line);
        }
    }

    Ok(())
}

/// Note: the target_dial_count calculation here would NOT work when `DIAL_TARGET != 0`
fn rotate_part2(line: &str, curr_dial: &mut usize, target_dial_count: &mut usize) -> Result<()> {
    let direction = line.chars().nth(0)
        .with_context(|| format!("Should have direction in line: {}", line))?;
    let rotation = line
        .get(1..)
        .with_context(|| format!("Should have rotation in line: {}", line))?
        .parse::<usize>()
        .with_context(|| format!("Should be able to parse number in line: {}", line))?;
    let start_dial = *curr_dial;

    match direction {
        'R' => {
            *curr_dial = (*curr_dial + rotation) % DIAL_LENGTH;

            // Count how many times we pass through 0
            let total_crosses = (start_dial + rotation) / DIAL_LENGTH;
            *target_dial_count += total_crosses;
        }
        'L' => {
            // Add DIAL_LENGTH before subtracting and do mod DIAL_LENGTH to ensure the result is positive
            *curr_dial = (*curr_dial + DIAL_LENGTH - (rotation % DIAL_LENGTH)) % DIAL_LENGTH;

            // Check if we cross or end at 0 in the remaining rotation (after removing "full" rotations)
            let remainder_rotation = rotation % DIAL_LENGTH;
            let remainder_rotation_crosses = if remainder_rotation >= start_dial && start_dial > 0 { 1 } else { 0 };

            let full_rotation_crosses = cmd.amount / DIAL_LENGTH;
            *target_dial_count += remainder_rotation_crosses + full_rotation_crosses;
        }
        _ => {
            bail!("Unknown direction '{}' in line: {}", direction, line);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_part1_right() {
        let mut curr_dial: usize = 20;
        rotate_part1("R20", &mut curr_dial).unwrap();
        assert_eq!(curr_dial, 40);
    }

    #[test]
    fn test_rotate_part1_left() {
        let mut curr_dial: usize = 20;
        rotate_part1("L10", &mut curr_dial).unwrap();
        assert_eq!(curr_dial, 10);
    }

    #[test]
    fn test_rotate_part1_right_wrap() {
        let mut curr_dial: usize = 20;
        rotate_part1("R80", &mut curr_dial).unwrap();
        assert_eq!(curr_dial, 0);
    }

    #[test]
    fn test_rotate_part1_left_wrap() {
        let mut curr_dial: usize = 5;
        rotate_part1("L10", &mut curr_dial).unwrap();
        assert_eq!(curr_dial, 95);
    }

    #[test]
    fn test_solve_part1_counts_correctly() {
        let lines = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        assert_eq!(solve_part1(&lines).unwrap(), "3");
    }

    #[test]
    fn test_rotate_part2_right() {
        let mut curr_dial: usize = 20;
        let mut target_dial_count: usize = 0;
        rotate_part2("R20", &mut curr_dial, &mut target_dial_count).unwrap();
        assert_eq!(curr_dial, 40);
    }

    #[test]
    fn test_rotate_part2_left() {
        let mut curr_dial: usize = 20;
        let mut target_dial_count: usize = 0;
        rotate_part2("L10", &mut curr_dial, &mut target_dial_count).unwrap();
        assert_eq!(curr_dial, 10);
    }

    #[test]
    fn test_rotate_part2_right_wrap() {
        let mut curr_dial: usize = 75;
        let mut target_dial_count: usize = 0;
        rotate_part2("R125", &mut curr_dial, &mut target_dial_count).unwrap();
        assert_eq!(curr_dial, 0);
        assert_eq!(target_dial_count, 2);
    }

    #[test]
    fn test_rotate_part2_left_wrap() {
        let mut curr_dial: usize = 75;
        let mut target_dial_count: usize = 0;
        rotate_part2("L125", &mut curr_dial, &mut target_dial_count).unwrap();
        assert_eq!(curr_dial, 50);
        assert_eq!(target_dial_count, 1);
    }

    #[test]
    fn test_rotate_part2_right_ends_at_zero() {
        let mut curr_dial: usize = 52;
        let mut target_dial_count: usize = 0;
        rotate_part2("R48", &mut curr_dial, &mut target_dial_count).unwrap();
        assert_eq!(curr_dial, 0);
        assert_eq!(target_dial_count, 1);
    }

    #[test]
    fn test_rotate_part2_left_ends_at_zero() {
        let mut curr_dial: usize = 55;
        let mut target_dial_count: usize = 0;
        rotate_part2("L55", &mut curr_dial, &mut target_dial_count).unwrap();
        assert_eq!(curr_dial, 0);
        assert_eq!(target_dial_count, 1);
    }

    #[test]
    fn test_rotate_part2_right_starts_at_zero() {
        let mut curr_dial: usize = 0;
        let mut target_dial_count: usize = 0;
        rotate_part2("R14", &mut curr_dial, &mut target_dial_count).unwrap();
        assert_eq!(curr_dial, 14);
        assert_eq!(target_dial_count, 0);
    }

    #[test]
    fn test_rotate_part2_left_starts_at_zero() {
        let mut curr_dial: usize = 0;
        let mut target_dial_count: usize = 0;
        rotate_part2("L5", &mut curr_dial, &mut target_dial_count).unwrap();
        assert_eq!(curr_dial, 95);
        assert_eq!(target_dial_count, 0);
    }

    #[test]
    fn test_solve_part2_counts_correctly() {
        let lines = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        assert_eq!(solve_part2(&lines).unwrap(), "6");
    }
}
