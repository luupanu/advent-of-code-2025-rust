use anyhow::{Context, Result};
use std::fs;

/// Read input file for a given day as a single string
pub fn read_input(day: u8) -> Result<String> {
    let filename = format!("input/day{:0>2}.txt", day);
    let content = fs::read_to_string(&filename)
        .with_context(|| format!("Could not read file: {}", filename))?;
    Ok(content.trim().to_string())
}

/// Read input file for a given day as lines
pub fn read_input_lines(day: u8) -> Result<Vec<String>> {
    let filename = format!("input/day{:0>2}.txt", day);
    let content = fs::read_to_string(&filename)
        .with_context(|| format!("Could not read file: {}", filename))?;
    Ok(content
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect())
}
