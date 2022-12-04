fn parse_line(line: &str) -> (u32, u32, u32, u32) {
    let splitted_line = line.split(',').collect::<Vec<_>>();
    let (left, right) = (splitted_line[0], splitted_line[1]);
    let splitted_left = left
        .split('-')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let splitted_right = right
        .split('-')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let (left_min, left_max) = (splitted_left[0], splitted_left[1]);
    let (right_min, right_max) = (splitted_right[0], splitted_right[1]);

    (left_min, left_max, right_min, right_max)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, line| {
        let (left_min, left_max, right_min, right_max) = parse_line(line);

        if (left_min >= right_min
            && left_min <= right_max
            && left_max <= right_max
            && left_max >= right_min)
            || (right_min >= left_min
                && right_min <= left_max
                && right_max <= left_max
                && right_max >= left_min)
        {
            acc + 1
        } else {
            acc
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, line| {
        let (left_min, left_max, right_min, right_max) = parse_line(line);

        if ((left_min >= right_min && left_min <= right_max)
            || (left_max <= right_max && left_max >= right_min))
            || ((right_min >= left_min && right_min <= left_max)
                || (right_max <= left_max && right_max >= left_min))
        {
            acc + 1
        } else {
            acc
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
