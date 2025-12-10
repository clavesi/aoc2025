use aoc2025::*;
use std::{collections::HashSet, io};

fn main() {
    let input = read_file(7, true);
    println!("Which part do you want to run?");
    let mut part = String::new();
    io::stdin()
        .read_line(&mut part)
        .expect("Failed to read input for part");
    match part.trim() {
        "1" => {
            let result = part1(&input);
            println!("Part 1 result: {}", result);
        }
        "2" => {
            let result = part2(&input);
            println!("Part 2 result: {}", result);
        }
        _ => println!("Invalid part!"),
    }
}

fn part1(input: &str) -> usize {
    // Start at S and go down to below cell with beam
    // If beam hits ^, it splits to left and right and those beams go down
    // Count each time it hits ^

    let mut split_count = 0;

    let lines = read_lines(input);
    let first_line = &lines[0];
    let s_index = first_line.find("S").unwrap();
    // changed: vec->HashSet so we don't need to keep track of indices by modifying the chars to for when two splits overlap a beam
    let mut tachyon_indices = HashSet::new(); // keep track of which indices have a beam
    tachyon_indices.insert(s_index);
    for line in &lines[1..] {
        let chars: Vec<char> = line.chars().collect();
        // a split changes which indices to keep track of, so we replace at end of line
        let mut new_indices = HashSet::new();
        for &index in &tachyon_indices {
            if chars[index] == '.' {
                new_indices.insert(index);
            } else if chars[index] == '^' {
                // Doesn't seem like ^ is ever on ends, so no out of bounds thankfully
                new_indices.insert(index - 1);
                new_indices.insert(index + 1);
                split_count += 1;
            }
        }
        tachyon_indices = new_indices;
    }

    return split_count;
}

fn part2(input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../inputs/day07.test");

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), -1);
    }
}
