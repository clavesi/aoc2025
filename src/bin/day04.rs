use aoc2025::*;
use std::io;

fn main() {
    let input = read_file(4, true);
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
    let mut total = 0;

    let grid = read_lines_as_chars_vec(input);
    let height = grid.len();
    let width = grid[0].len();

    // Go over each cell
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '@' {
                let mut neighboring: u8 = 0;
                // Go over each neighbor
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let ny = (y as i32 + dy) as usize;
                        let nx = (x as i32 + dx) as usize;
                        // Neighbor is also
                        if ny < height && nx < width && grid[ny][nx] == '@' {
                            neighboring += 1;
                        }
                    }
                }

                if neighboring < 4 {
                    total += 1;
                }
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

    const TEST_INPUT: &str = include_str!("../../inputs/day04.test");

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), -1);
    }
}
