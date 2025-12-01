use std::fs;

/// Read input file for a given day as a single string
pub fn read_input(day: u8) -> String {
    let filename = format!("input/day{:0>2}.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Could not read file: {}", filename))
        .trim()
        .to_string()
}

/// Read input file for a given day as lines
pub fn read_input_lines(day: u8) -> Vec<String> {
    let filename = format!("input/day{:0>2}.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Could not read file: {}", filename))
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
