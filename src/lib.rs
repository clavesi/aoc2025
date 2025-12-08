use std::fs;

pub fn read_file(day: u8, is_real_input: bool) -> String {
    let path: String = if is_real_input {
        format!("./inputs/day{:02}.real", day)
    } else {
        format!("./inputs/day{:02}.test", day)
    };
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {path}"))
}

pub fn read_lines(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn read_lines_as_chars_vec(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect()
}
