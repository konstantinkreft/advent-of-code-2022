fn get_alphabet() -> Vec<char> {
    let mut alphabet = (10..36)
        .map(|i| char::from_digit(i, 36).unwrap())
        .collect::<Vec<_>>();
    let mut uppercase = alphabet
        .iter()
        .map(|c| c.to_uppercase().next().unwrap())
        .collect::<Vec<_>>();
    alphabet.append(&mut uppercase);

    alphabet
}

fn get_priority(input_char: &char, alphabet: &[char]) -> u32 {
    alphabet.iter().position(|c| c == input_char).unwrap() as u32 + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let alphabet = get_alphabet();

    Some(input.lines().fold(0, |acc, line| {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let mut common_char = None;
        for c in first_half.chars() {
            if second_half.contains(c) {
                common_char = Some(c);
                break;
            }
        }
        let common_char = common_char.unwrap();
        let priority = get_priority(&common_char, &alphabet.clone());
        acc + priority
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_vec = input.lines().collect::<Vec<_>>();
    let input_chunks = input_vec.chunks(3);
    let alphabet = get_alphabet();

    Some(input_chunks.fold(0, |acc, chunk| {
        let mut common_char: Option<char> = None;
        for c in chunk[0].chars() {
            if chunk[1].contains(c) && chunk[2].contains(c) {
                common_char = Some(c);
                break;
            }
        }
        let common_char = common_char.unwrap();
        let priority = get_priority(&common_char, &alphabet.clone());
        acc + priority
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
