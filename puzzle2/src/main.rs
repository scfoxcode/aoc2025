use std::fs;

fn part1(input: &str) -> u64 {
    input.split(',') // Intentionally condensed, not good code, too fancy
        .flat_map(|str_range| {
            str_range.split_once('-')
                .map(|(left, right)| {
                    match (left.parse::<u64>() , right.parse::<u64>()) {
                        (Ok(start), Ok(end)) => (start..=end).map(|val| (val, val.to_string())),
                        _ => panic!("Failed to parse left or right {left} {right}"),
                    }
                })
        })
        .flatten() // all id tuples
        .filter_map(|(id, id_string)| {
            match id_string.split_at(id_string.len() / 2) {
                (a, b) if a == b && a.len() == b.len() => Some(id),
                _ => None,
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input.split(',')
        .flat_map(|str_range| {
            str_range.split_once('-')
                .map(|(left, right)| {
                    match (left.parse::<u64>() , right.parse::<u64>()) {
                        (Ok(start), Ok(end)) => (start..=end).map(|val| (val, val.to_string())),
                        _ => panic!("Failed to parse left or right {left} {right}"),
                    }
                })
        })
        .flatten()        
        .filter_map(|(id, id_string)| {
            let digits = id_string.len();

            if digits <= 1 {
                return None;
            }

            let divisors = valid_divisors(digits);

            for divisor in divisors {
                let part_length = id_string.len() / divisor; 
                let mut parts: Vec<String> = Vec::new();
                let mut temp_string = String::new();

                for char in id_string.chars() {
                    temp_string.push(char);
                    if temp_string.len() == part_length {
                        parts.push(temp_string);
                        temp_string = String::new();
                    }
                }

                if all_equal(parts.as_slice()) {
                    return Some(id);
                }
            }
            return None;

        })
        .sum()
}

fn all_equal(input: &[String]) -> bool {
    for i in 0..(input.len() - 1) {
        if input[i] != input[i+1] {
            return false;
        }
    }
    true
}

fn valid_divisors(digits: usize) -> Vec<usize> {
    let mut divisors: Vec<usize> = vec![digits];
    for potential in 2..digits {
        if digits % potential == 0 {
            divisors.push(potential);
        }
    }
    divisors
}

fn main() {
    let puzzle_input = {
        let input = fs::read_to_string("part1.input").expect("Failed to open part1 input");
        input.trim().to_string()
    };

    let part_1_answer = part1(puzzle_input.as_str());
    println!("Part 1 = {part_1_answer}");
    let part_2_answer = part2(puzzle_input.as_str());
    println!("Part 2 = {part_2_answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result , 1_227_775_554);
    }

    #[test]
    fn test_part_2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result , 4_174_379_265);
    }

    #[test]
    fn full_part_1() {
        let puzzle_input = {
            let input = fs::read_to_string("part1.input").expect("Failed to open part1 input");
            input.trim().to_string()
        };
        let result = part1(puzzle_input.as_str());
        assert_eq!(result, 29_940_924_880);
    }
}
    #[test]
    fn full_part_2() {
        let puzzle_input = {
            let input = fs::read_to_string("part1.input").expect("Failed to open part1 input");
            input.trim().to_string()
        };
        let result = part2(puzzle_input.as_str());
        assert_eq!(result, 48_631_958_998);
    }
