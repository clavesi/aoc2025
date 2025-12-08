use aoc2025::*;
use std::io;

fn main() {
    let input = read_file(3, true);
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
    // FOr each line, we need to grab the two numbers that make the biggest number when combined
    // i.e. 987654321111111 -> (98)7654321111111 and  811111111111119->(8)1111111111111(9)
    // then sum these numbers
    // note: it is in order, so like how 811111111111119 was 89, not 98

    let mut sum = 0;

    let lines = read_lines(input);
    for line in lines {
        let mut biggest = 0;
        for (i, c) in line.chars().enumerate() {
            for (_, d) in line[i + 1..].chars().enumerate() {
                let mut num = String::from("");
                num.push(c);
                num.push(d);
                let num: u64 = num.parse().unwrap();
                if num > biggest {
                    biggest = num;
                }
            }
        }
        sum += biggest;
    }

    // Can't believe this worked first try

    return sum;
}

fn part2(input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../inputs/day03.test");

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 357);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), -1);
    }
}
