use aoc2025::*;
use std::io;

fn main() {
    let input = read_file(6, true);
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

fn part1(input: &str) -> u64 {
    // Get lines
    // For each line, split based on whitespace
    // Last line split indicates math operation
    // For each index of each split, do the math operation on those numbers
    let mut total = 0;

    let lines = read_lines(input);
    let num_lines = lines.len();
    let mut operations = vec![];
    for line in lines {
        operations.push(
            line.split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        )
    }

    // Note, this was incredibly annoying.
    // I kept getting col vs row mixed up
    for col in 0..operations[0].len() {
        let mut op_total: u64 = operations[0][col].parse().unwrap();
        for row in 1..(num_lines - 1) {
            let op = &operations[num_lines - 1][col];
            if op == "*" {
                op_total = op_total * operations[row][col].parse::<u64>().unwrap();
            }
            if op == "+" {
                op_total = op_total + operations[row][col].parse::<u64>().unwrap();
            }
        }
        total += op_total;
    }

    return total;
}

fn part2(input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../inputs/day06.test");

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 4277556);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), -1);
    }
}
