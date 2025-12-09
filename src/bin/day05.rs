use aoc2025::*;
use std::{collections::HashSet, io};

fn main() {
    let input = read_file(5, true);
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

fn _part1_first_attempt(input: &str) -> u64 {
    /*
    * Input structure:
    * List of ranges
    * Blank line
    * Ingredient ids

    *  If ingredient in ranges, FRESH, else false
    * Total num of fresh ingredients
    */

    // Read ranges
    // Add each number in ranges to a list
    // Read ids
    // If id in range list, fresh++

    let mut total = 0;
    let lines = read_lines_without_empty_filter(input);
    let parts: Vec<_> = lines.split(|line| line.is_empty()).collect();
    let ranges = parts[0];
    let ids = parts[1];

    let mut fresh_set: HashSet<u64> = HashSet::new();
    for range in ranges {
        let r: Vec<&str> = range.split(|ch| ch == '-').collect();
        let first: u64 = r[0].parse().unwrap();
        let last: u64 = r[1].parse().unwrap();
        for i in first..last + 1 {
            fresh_set.insert(i);
            println!("{i}");
        }
    }

    println!("{:?}", fresh_set);

    for id in ids {
        let id: u64 = id.parse().unwrap();
        if fresh_set.contains(&id) {
            total += 1;
        }
    }

    return total;

    // Note: this won't exactly work
    // The ranges are way too big so this method will take super long
    // Like each range is maybe a trillion options
}

fn part1(input: &str) -> i32 {
    // Instead, we could make a list of the ranges
    // Then check whether each number falls in a range
    //      i.e. num >= first && num <= max
    // If it does, total++ and go to next num

    let mut total = 0;
    let lines = read_lines_without_empty_filter(input);
    let parts: Vec<_> = lines.split(|line| line.is_empty()).collect();
    let ranges = parts[0];
    let ids = parts[1];

    let mut r: Vec<Vec<u64>> = vec![];
    for range in ranges {
        // This could probably happen at ranges definition
        r.push(range.split('-').map(|s| s.parse().unwrap()).collect());
    }

    for id in ids {
        let id: u64 = id.parse().unwrap();
        for range in &r {
            if id >= range[0] && id <= range[1] {
                total += 1;
                break;
            }
        }
    }

    return total;
}

fn part2(input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../inputs/day05.test");

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), -1);
    }
}
