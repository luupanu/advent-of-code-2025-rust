use advent_of_code_2025::{read_input_lines};
use anyhow::{Result};

fn main() -> Result<()> {
    let lines = read_input_lines(3)?;

    println!("Part 1: {}", solve_part1(&lines)?);
    println!("Part 2: {}", solve_part2(&lines)?);

    Ok(())
}

fn solve_part1(lines: &Vec<String>) -> Result<String> {
    let mut sum: u64 = 0;

    for line in lines {
        sum += find_maximal_of_length_n(line, 2);
    }

    Ok(sum.to_string())
}

fn solve_part2(lines: &Vec<String>) -> Result<String> {
    let mut sum: u64 = 0;

    for line in lines {
        sum += find_maximal_of_length_n(line, 12);
    }

    Ok(sum.to_string())
}

fn find_maximal_of_length_n(input: &str, n: usize) -> u64 {
    let mut found_len: usize = 0;
    let mut ith_num_idx: usize = 0;
    let mut chars: Vec<char> = vec![];

    for i in 0..n {
        let start = if i == 0 { 0 } else { ith_num_idx + 1 };
        let end = input.len() - n + found_len;

        let (ith_num_as_char, next_idx) = find_max_number_from_substring(&input[start..=end]);

        chars.push(ith_num_as_char);
        ith_num_idx = start + next_idx;
        found_len += 1;
    }

    chars.iter().collect::<String>().parse::<u64>().unwrap()
}

fn find_max_number_from_substring(input: &str) -> (char, usize) {
    let mut max_number_as_char: char = '0';
    let mut max_number_idx: usize = 0;

    for (idx, x) in input.chars().enumerate() {
        if x == '9' {
            // If we find 9, we are done here.
            // Can't get any better.
            return (x, idx);
        } else if x > max_number_as_char {
            max_number_as_char = x;
            max_number_idx = idx;
        }
    }

    (max_number_as_char, max_number_idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximal_of_length_n_pair_1() {
        let input = "987654321111111";

        assert_eq!(find_maximal_of_length_n(input, 2), 98);
    }

    #[test]
    fn test_find_maximal_of_length_n_pair_2() {
        let input = "811111111111119";

        assert_eq!(find_maximal_of_length_n(input, 2), 89);
    }

    #[test]
    fn test_find_maximal_of_length_n_pair_3() {
        let input = "234234234234278";

        assert_eq!(find_maximal_of_length_n(input, 2), 78);
    }

    #[test]
    fn test_find_maximal_of_length_n_pair_4() {
        let input = "818181911112111";

        assert_eq!(find_maximal_of_length_n(input, 2), 92);
    }

    #[test]
    fn test_solve_part1_counts_correctly() {
        let lines = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];

        assert_eq!(solve_part1(&lines).unwrap(), "357");
    }

    #[test]
    fn test_solve_part2_counts_correctly() {
        let lines = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];

        assert_eq!(solve_part2(&lines).unwrap(), "3121910778619");
    }
}
