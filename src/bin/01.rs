pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sorted_calories: Vec<u32> = input
        .split("\n\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect::<Vec<u32>>();

    sorted_calories.sort_by(|a, b| b.cmp(a));
    let sum_of_calories = sorted_calories[0..3].iter().sum::<u32>();

    return Some(sum_of_calories);
}

fn main() {
    let input = advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, &input);
    advent_of_code::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
