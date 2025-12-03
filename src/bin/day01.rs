use aoc2025::*;
use std::io;

fn main() {
    let input = read_file(1, true);
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

fn part1(input: &str) -> i32 {
    // Input is like L# and R#
    // Read first char, if L, then # is neg
    //                  if R, then # is pos
    // Keep a running sum and a tally. Every time the sum % 100 = 0, add to tally.
    // Return tally.

    let mut sum = 50; // dial starts at 50
    let mut tally = 0;
    let lines = read_lines(input);

    for line in lines {
        // loops 0..99
        let pos_or_neg = &line[..1];
        // convert num to integer
        let mut num: i32 = line[1..].parse().expect("Can't convert string to integer");
        // if pos_or_neg == "L" multiply by -1
        if pos_or_neg == "L" {
            num *= -1;
        }
        // add to sum
        sum += num;
        // keep sum between -100 and 100 since % is remainder in Rust, not modulo
        sum = sum % 100;
        println!("{}", sum);
        if sum == 0 {
            tally += 1;
        }
    }

    return tally;
}

fn part2(input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../inputs/day01.test");

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), -2);
    }
}
