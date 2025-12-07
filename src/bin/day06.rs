use advent_of_code_2025::read_input_lines;
use anyhow::{Result, bail};

fn main() -> Result<()> {
    let mut lines = read_input_lines(6)?;

    println!("Part 1: {}", solve_part1(&lines)?);
    println!("Part 2: {}", solve_part2(&mut lines)?);

    Ok(())
}

fn solve_part1(lines: &Vec<String>) -> Result<String> {
    let (mut numbers, mut operators) = parse_numbers_operators_part_1(lines)?;
    let mut sum: u64 = 0;

    'outer: loop {
        let mut curr_numbers: Vec<u64> = vec![];

        for num_vec in numbers.iter_mut() {
            if let Some(num) = num_vec.pop() {
                curr_numbers.push(num);
            } else {
                break 'outer;
            }
        }
        if let Some(op) = operators.pop() {
            match op {
                Op::Product => sum += curr_numbers.iter().product::<u64>(),
                Op::Sum => sum += curr_numbers.iter().sum::<u64>(),
            }
        }
    }

    Ok(sum.to_string())
}

fn solve_part2(lines: &mut Vec<String>) -> Result<String> {
    let len = lines.len();
    let (num_lines, op_slice) = lines.split_at_mut(len - 1);
    let mut sum: u64 = 0;
    let mut curr_numbers: Vec<u64> = vec![];

    'outer: loop {
        let mut num_as_str = String::new();

        for num_line in num_lines.iter_mut() {
            if let Some(char) = num_line.pop() {
                if let Some(num) = char.to_digit(10) {
                    num_as_str.push_str(&num.to_string());
                }
            } else {
                break 'outer;
            }
        }

        if !num_as_str.is_empty() {
            curr_numbers.push(num_as_str.parse::<u64>()?);
        }

        if let Some(char) = op_slice[0].pop() {
            match char {
                // my poor enum :(
                '*' => {
                    sum += curr_numbers.iter().product::<u64>();
                    curr_numbers = vec![];
                }
                '+' => {
                    sum += curr_numbers.iter().sum::<u64>();
                    curr_numbers = vec![];
                }
                _ => { /* do nothing */ }
            }
        }
    }

    Ok(sum.to_string())
}

fn parse_numbers_operators_part_1(lines: &Vec<String>) -> Result<(Vec<Vec<u64>>, Vec<Op>)> {
    let numbers: Vec<Vec<u64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|str| str.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let operators = lines[lines.len() - 1]
        .split_whitespace()
        .map(|str| match str {
            "*" => Ok(Op::Product),
            "+" => Ok(Op::Sum),
            _ => bail!("Bad operator {}", str),
        })
        .collect::<Result<Vec<Op>>>()?;

    Ok((numbers, operators))
}

#[derive(Debug)]
enum Op {
    Product,
    Sum,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_counts_correctly() {
        let lines = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(solve_part1(&lines).unwrap(), "4277556");
    }

    #[test]
    fn test_solve_part2_counts_correctly() {
        let mut lines = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(solve_part2(&mut lines).unwrap(), "3263827");
    }
}
