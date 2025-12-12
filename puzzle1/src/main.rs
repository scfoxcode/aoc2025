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

enum Limit {
    Min(i32),
    Max(i32),
}

struct Action {
    pub inc: i32,
    pub clicks: i32,
    pub limit: Limit,
    pub reset: i32,
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .fold((50, 0), |acc, line| {
            let action: Action = match line.split_at(1) {
                ("L", val) => { 
                    Action {
                        inc: -1, 
                        clicks: val.parse().expect("Invalid number for L"),
                        limit: Limit::Min(0),
                        reset: 99,
                    }
                },
                ("R", val) => { 
                    Action {
                        inc: 1, 
                        clicks: val.parse().expect("Invalid number for R"),
                        limit: Limit::Max(99),
                        reset: 0,
                    }
                }
                _ => panic!("Invalid input when splitting line"),
            };

            let mut sum = acc.0;
            let mut zero_intercepts = acc.1;

            for click in 0..action.clicks {
                sum += action.inc;
                if sum == 0 {
                    zero_intercepts += 1;
                }
                match action.limit {
                    Limit::Max(val) => {
                        if sum > val {
                            zero_intercepts += 1;
                            sum = action.reset;
                        }
                    }
                    Limit::Min(val) => {
                        if sum < val {
                            sum = action.reset;
                        }
                    }
                };
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
}
