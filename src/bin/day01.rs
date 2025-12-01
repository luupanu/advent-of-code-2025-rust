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
        let cmd = parse_rotation_cmd(line)?;
        rotate_part1(cmd, &mut curr_dial, &mut target_dial_count);
    }

    Ok(target_dial_count.to_string())
}

fn solve_part2(lines: &Vec<String>) -> Result<String> {
    let mut curr_dial = DIAL_START;
    let mut target_dial_count: usize = 0;

    for line in lines {
        let cmd = parse_rotation_cmd(line)?;
        rotate_part2(cmd, &mut curr_dial, &mut target_dial_count);
    }

    Ok(target_dial_count.to_string())
}

enum Direction {
    Right,
    Left,
}

struct RotationCmd {
    direction: Direction,
    amount: usize,
}

fn parse_rotation_cmd(line: &str) -> Result<RotationCmd> {
    let direction_char = line.chars().nth(0)
        .with_context(|| format!("Should have direction in line: {}", line))?;
    let direction = match direction_char {
        'R' => Direction::Right,
        'L' => Direction::Left,
        _ => {
            bail!("Unknown direction '{}' in line: {}", direction_char, line);
        }
    };
    let amount = line
        .get(1..)
        .with_context(|| format!("Should have rotation amount in line: {}", line))?
        .parse::<usize>()
        .with_context(|| format!("Should be able to parse number in line: {}", line))?;

    Ok(RotationCmd{ direction, amount })
}

fn rotate_part1(cmd: RotationCmd, curr_dial: &mut usize, target_dial_count: &mut usize) {
    match cmd.direction {
        Direction::Right => {
            *curr_dial = (*curr_dial + cmd.amount) % DIAL_LENGTH;
        }
        Direction::Left => {
            // Add DIAL_LENGTH before subtracting and do mod DIAL_LENGTH to ensure the result is positive
            *curr_dial = (*curr_dial + DIAL_LENGTH - cmd.amount % DIAL_LENGTH) % DIAL_LENGTH;
        }
    }
    if *curr_dial == DIAL_TARGET {
        *target_dial_count += 1;
    }
}

/// Note: the target_dial_count calculation here would NOT work when `DIAL_TARGET != 0`
fn rotate_part2(cmd: RotationCmd, curr_dial: &mut usize, target_dial_count: &mut usize) {
    let start_dial = *curr_dial;

    match cmd.direction {
        Direction::Right => {
            *curr_dial = (*curr_dial + cmd.amount) % DIAL_LENGTH;

            // Count how many times we pass through 0
            let total_crosses = (start_dial + cmd.amount) / DIAL_LENGTH;
            *target_dial_count += total_crosses;
        }
        Direction::Left => {
            // Add DIAL_LENGTH before subtracting and do mod DIAL_LENGTH to ensure the result is positive
            *curr_dial = (*curr_dial + DIAL_LENGTH - (cmd.amount % DIAL_LENGTH)) % DIAL_LENGTH;

            // Check if we cross or end at 0 in the remaining rotation (after removing "full" rotations)
            // This is asymmetric with Direction::Right – meaning that it can't be calculated with `(start_dial - cmd.amount) / DIAL_LENGTH`
            // since division does not count the first zero crossing from positive numbers to negative numbers.
            let remainder_rotation = cmd.amount % DIAL_LENGTH;
            let remainder_rotation_crosses = if remainder_rotation >= start_dial && start_dial > 0 { 1 } else { 0 };

            let full_rotation_crosses = cmd.amount / DIAL_LENGTH;
            *target_dial_count += remainder_rotation_crosses + full_rotation_crosses;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_part1_right() {
        let mut curr_dial: usize = 20;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Right, amount: 20 };

        rotate_part1(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 40);
        assert_eq!(target_dial_count, 0);
    }

    #[test]
    fn test_rotate_part1_left() {
        let mut curr_dial: usize = 20;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Left, amount: 10 };

        rotate_part1(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 10);
        assert_eq!(target_dial_count, 0);
    }

    #[test]
    fn test_rotate_part1_right_wrap() {
        let mut curr_dial: usize = 20;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Right, amount: 80 };

        rotate_part1(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 0);
        assert_eq!(target_dial_count, 1);
    }

    #[test]
    fn test_rotate_part1_left_wrap() {
        let mut curr_dial: usize = 5;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Left, amount: 10 };

        rotate_part1(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 95);
        assert_eq!(target_dial_count, 0);
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
        let cmd = RotationCmd { direction: Direction::Right, amount: 20 };

        rotate_part2(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 40);
    }

    #[test]
    fn test_rotate_part2_left() {
        let mut curr_dial: usize = 20;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Left, amount: 10 };

        rotate_part2(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 10);
    }

    #[test]
    fn test_rotate_part2_right_wrap() {
        let mut curr_dial: usize = 75;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Right, amount: 125 };

        rotate_part2(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 0);
        assert_eq!(target_dial_count, 2);
    }

    #[test]
    fn test_rotate_part2_left_wrap() {
        let mut curr_dial: usize = 75;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Left, amount: 125 };

        rotate_part2(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 50);
        assert_eq!(target_dial_count, 1);
    }

    #[test]
    fn test_rotate_part2_right_ends_at_zero() {
        let mut curr_dial: usize = 52;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Right, amount: 48 };

        rotate_part2(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 0);
        assert_eq!(target_dial_count, 1);
    }

    #[test]
    fn test_rotate_part2_left_ends_at_zero() {
        let mut curr_dial: usize = 55;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Left, amount: 55 };

        rotate_part2(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 0);
        assert_eq!(target_dial_count, 1);
    }

    #[test]
    fn test_rotate_part2_right_starts_at_zero() {
        let mut curr_dial: usize = 0;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Right, amount: 14 };

        rotate_part2(cmd, &mut curr_dial, &mut target_dial_count);

        assert_eq!(curr_dial, 14);
        assert_eq!(target_dial_count, 0);
    }

    #[test]
    fn test_rotate_part2_left_starts_at_zero() {
        let mut curr_dial: usize = 0;
        let mut target_dial_count: usize = 0;
        let cmd = RotationCmd { direction: Direction::Left, amount: 5 };

        rotate_part2(cmd, &mut curr_dial, &mut target_dial_count);

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
