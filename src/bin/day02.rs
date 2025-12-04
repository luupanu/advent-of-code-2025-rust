use advent_of_code_2025::read_input;
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = read_input(2)?;

    println!("Part 1: {}", solve_part1(&input)?);
    println!("Part 2: {}", solve_part2(&input)?);

    Ok(())
}

fn solve_part1(input: &str) -> Result<String> {
    let mut sum: u64 = 0;

    for range in input.split(',') {
        let (start, end) = parse_range(range)?;

        for x in start..=end {
            if is_repeated_twice(&x.to_string()) {
                sum += x;
            }
        }
    }

    Ok(sum.to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    let mut sum: u64 = 0;

    for range in input.split(',') {
        let (start, end) = parse_range(range)?;

        for x in start..=end {
            if has_repeated_sequence(&x.to_string()) {
                sum += x;
            }
        }
    }

    Ok(sum.to_string())
}

fn parse_range(range: &str) -> Result<(u64, u64)> {
    let (start_str, end_str) = range.split_once('-')
        .ok_or_else(|| anyhow!("should have split once"))?;
    Ok((start_str.parse()?, end_str.parse()?))
}

fn is_repeated_twice(input: &str) -> bool {
    // Skip checking inputs with odd length.
    // They cannot be made of only some sequence of digits repeated twice.
    if input.len() % 2 == 1 {
        return false
    }
    let len_half = input.len() / 2;
    return &input[0..len_half] == &input[len_half..];
}

fn has_repeated_sequence(input: &str) -> bool {
    let concatted = format!("{}{}", input, input);
    let search_slice = &concatted[1..&concatted.len()-1];

    search_slice.find(input).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_repeated_twice_finds_seq_1() {
        let input = "55";

        assert_eq!(is_repeated_twice(input), true);
    }

    #[test]
    fn test_is_repeated_twice_finds_seq_2() {
        let input = "6464";

        assert_eq!(is_repeated_twice(input), true);
    }

    #[test]
    fn test_is_repeated_twice_finds_seq_3() {
        let input = "123123";

        assert_eq!(is_repeated_twice(input), true);
    }

    #[test]
    fn test_is_repeated_twice_no_repetition_returns_false() {
        let input = "12341235";

        assert_eq!(is_repeated_twice(input), false);
    }

    #[test]
    fn test_has_repeated_sequence_finds_seq_1() {
        let input = "12341234";

        assert_eq!(has_repeated_sequence(input), true);
    }

    #[test]
    fn test_has_repeated_sequence_finds_seq_2() {
        let input = "123123123";

        assert_eq!(has_repeated_sequence(input), true);
    }

    #[test]
    fn test_has_repeated_sequence_finds_seq_3() {
        let input = "1212121212";

        assert_eq!(has_repeated_sequence(input), true);
    }

    #[test]
    fn test_has_repeated_sequence_finds_seq_4() {
        let input = "1111111";

        assert_eq!(has_repeated_sequence(input), true);
    }

    #[test]
    fn test_has_repeated_sequence_no_repetition_returns_false() {
        let input = "12341235";

        assert_eq!(has_repeated_sequence(input), false);
    }

    #[test]
    fn test_solve_part1_counts_correctly() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(solve_part1(input).unwrap(), "1227775554");
    }

    #[test]
    fn test_solve_part2_counts_correctly() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(solve_part2(input).unwrap(), "4174379265");
    }
}
