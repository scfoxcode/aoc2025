use std::fs;

fn part1(input: &str) -> i32 {
    input
        .lines()
        .fold((50, 0), |acc, line| {
            match line.split_at(1) {
                ("L", val) => {
                    let clicks: i32 = val.parse().expect("Invalid number for L");
                    let mut sum = acc.0 - clicks;
                    while sum < 0 {
                        sum += 100;
                    }
                    let new_count = if sum == 0 { acc.1 + 1 } else { acc.1 };
                    (sum, new_count)
                }
                ("R", val) => {
                    let clicks: i32 = val.parse().expect("Invalid number for R");
                    let mut sum = acc.0 + clicks;
                    while sum > 99 {
                        sum -= 100;
                    }
                    let new_count = if sum == 0 { acc.1 + 1 } else { acc.1 };
                    (sum, new_count)
                }
                _ => panic!("Illegal Input")
            }
        }).1
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .fold((50, 0), |acc, line| {
            let parts = line.split_at(1);
            let inc = match parts.0 {
                "L" => -1,
                "R" => 1,
                _ => panic!("Invalid direction. Must be L or R"),
            };
            let clicks = parts.1.parse().expect("Invalid number = {parts.1}");

            let mut sum = acc.0;
            let mut zero_intercepts = acc.1;

            for click in 0..clicks {
                sum += inc;
                if sum == 0 {
                    zero_intercepts += 1;
                }
                if sum > 99 {
                    zero_intercepts += 1;
                    sum = 0;
                }
                if sum < 0 {
                    sum = 99;
                }
            }
            (sum, zero_intercepts)
        }).1
}

fn main() {
    let puzzle_input = fs::read_to_string("part1.input").expect("Failed to open part1 input");
    let part_1_answer = part1(puzzle_input.as_str());
    println!("Part 1 = {part_1_answer}"); // 1100 for my input :D
    let part_2_answer = part2(puzzle_input.as_str());
    println!("Part 2 = {part_2_answer}"); // 6358 for my input :D
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_input: &str = 
        "L68\n\
        L30\n\
        R48\n\
        L5\n\
        R60\n\
        L55\n\
        L1\n\
        L99\n\
        R14\n\
        L82";

    #[test]
    fn test_part_1() {
        let result = part1(test_input);
        assert_eq!(result , 3);
    }

    #[test]
    fn test_part_2() {
        let result = part2(test_input);
        assert_eq!(result , 6);
    }

    #[test]
    fn full_part_1() {
        let puzzle_input = fs::read_to_string("part1.input").expect("Failed to open part1 input");
        let result = part1(puzzle_input.as_str());
        assert_eq!(result, 1100);
    }
}
    #[test]
    fn full_part_2() {
        let puzzle_input = fs::read_to_string("part1.input").expect("Failed to open part1 input");
        let result = part2(puzzle_input.as_str());
        assert_eq!(result, 6358);
    }
