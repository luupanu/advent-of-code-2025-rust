use std::cmp::Ordering;

use advent_of_code_2025::{read_input_lines};
use anyhow::{Result};

fn main() -> Result<()> {
    let lines = read_input_lines(5)?;
    let (ranges, ingredients): (Vec<String>, Vec<String>) = lines
        .into_iter()
        .partition(|line| line.contains('-'));

    println!("Part 1: {}", solve_part1(&ranges, &ingredients)?);
    println!("Part 2: {}", solve_part2(&ranges)?);

    Ok(())
}

fn solve_part1(ranges: &Vec<String>, ingredients: &Vec<String>) -> Result<String> {
    let parsed_and_sorted_ranges = parse_and_sort_ranges(&ranges)?;
    let merged_ranges = merge_overlapping_ranges(&parsed_and_sorted_ranges);
    let parsed_ingredients: Vec<u64> = ingredients.iter().map(|i| i.parse::<u64>().unwrap()).collect();

    let mut sum: u64 = 0;

    for ingr in parsed_ingredients {
        let res = merged_ranges.binary_search_by(|range| {
            if ingr < range.start {
                Ordering::Greater
            } else if ingr > range.end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        if res.is_ok() {
            sum += 1;
        }
    }

    Ok(sum.to_string())
}

fn solve_part2(ranges: &Vec<String>) -> Result<String> {
    let parsed_and_sorted_ranges = parse_and_sort_ranges(&ranges)?;
    let merged_ranges = merge_overlapping_ranges(&parsed_and_sorted_ranges);

    let mut sum: u64 = 0;

    for range in merged_ranges {
        sum += range.end - range.start + 1;
    }

    Ok(sum.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_and_sort_ranges(ranges_as_str: &Vec<String>) -> Result<Vec<Range>> {
    let mut ranges = ranges_as_str.iter().map(|x | {
        let (start_str, end_str) = x.split_once('-').unwrap();
        let start = start_str.parse::<u64>()?;
        let end = end_str.parse::<u64>()?;

        Ok(Range { start, end })
    }).collect::<Result<Vec<Range>>>()?;

    ranges.sort();

    Ok(ranges)
}

fn merge_overlapping_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    let mut merged_ranges = vec![ranges[0]];

    for range in &ranges[1..] {
        let last = merged_ranges.last_mut().unwrap();

        if range.start <= last.end {
            last.end = last.end.max(range.end);
        } else {
            merged_ranges.push(*range);
        }
    }

    merged_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_sort_ranges_sorts_by_start_and_then_by_end() {
        let input = vec![
            "3-5",
            "10-14",
            "16-20",
            "12-18",
            "10-15",
        ].into_iter().map(String::from).collect();;

        let sorted_ranges = parse_and_sort_ranges(&input).unwrap();
        assert_eq!(sorted_ranges, vec![
            Range{ start: 3, end: 5 },
            Range{ start: 10, end: 14 },
            Range{ start: 10, end: 15 },
            Range{ start: 12, end: 18 },
            Range{ start: 16, end: 20 },
        ]);
    }

    #[test]
    fn test_merge_overlapping_ranges_merges_correctly() {
        let ranges = vec![
            Range{ start: 3, end: 5 },
            Range{ start: 10, end: 14 },
            Range{ start: 12, end: 18 },
            Range{ start: 16, end: 20 },
        ];
        assert_eq!(merge_overlapping_ranges(&ranges), vec![
            Range{ start: 3, end: 5 },
            Range{ start: 10, end: 20 },
        ]);
    }

    #[test]
    fn test_solve_part1_counts_correctly() {
        let ranges = vec![
            "3-5",
            "10-14",
            "16-20",
            "12-18",
        ].into_iter().map(String::from).collect();
        let ingredients = vec!["1", "5", "8", "11", "17", "32"].into_iter().map(String::from).collect();
        assert_eq!(solve_part1(&ranges, &ingredients).unwrap(), "3");
    }

    #[test]
    fn test_solve_part2_counts_correctly() {
        let ranges = vec![
            "3-5",
            "10-14",
            "16-20",
            "12-18",
        ].into_iter().map(String::from).collect();
        assert_eq!(solve_part2(&ranges).unwrap(), "14");
    }
}