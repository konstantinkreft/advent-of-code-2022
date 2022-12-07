
use std::collections::HashSet;

fn find_distinct_substring(input: &str, length: usize) -> Option<u32> {
    let mut packet_marker: Option<u32> = None;
    for (i, _c) in input.chars().enumerate() {
        if i >= length {
            let mut known_chars = HashSet::new();
            for j in 0..length {
                known_chars.insert(input.chars().nth(i - j).unwrap());
            }

            if known_chars.len() == length {
                packet_marker = Some(i as u32 + 1);
                break;
            }
        }
    };

    packet_marker
}

pub fn part_one(input: &str) -> Option<u32> {
    find_distinct_substring(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_distinct_substring(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
