use aoc2025::*;
use std::io;

fn main() {
    let input = read_file(2, true);
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
    // sum = 0
    // split by comma to get each range
    // for each range
    //     split by hyphen to get both sides
    //     for each ID in the range of both sides
    //         split in two by middle (if odd, then don't consider)
    //         if both sides are equal, sum += side
    // return sum
    let mut sum: u64 = 0;
    let line = read_lines(input);
    let ranges: Vec<&str> = line[0].split(",").collect();

    for range in ranges {
        let ids: Vec<&str> = range.split('-').collect();
        let left_id: u64 = ids[0].parse().expect("Can't convert string ID to integer");
        let right_id: u64 = ids[1].parse().expect("Can't convert string ID to integer");
        for id in left_id..(right_id + 1) {
            let id_str = id.to_string();
            let id_len = id_str.len();
            if id_len % 2 != 0 {
                // No need to check if odd length
                continue;
            }
            let id_mid = id_len / 2;
            let left_id = &id_str[..id_mid];
            let right_id = &id_str[id_mid..];
            // println!("{:?} {:?}", left_id, right_id);
            if left_id == right_id {
                println!("Found invalid id - {}", id);
                sum += id;
            }
        }
    }

    return sum;
}

fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../inputs/day02.test");

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 1227775554);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), -1);
    }
}
