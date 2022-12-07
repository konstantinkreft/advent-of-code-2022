type Stack = Vec<Vec<char>>;
type Move = (usize, usize, usize);
type Input = (Stack, Vec<Move>);

fn parse(input: &str) -> Input {
    let (stack_str, move_str) = input.split_once("\n\n").unwrap();

    let mut stack_iter = stack_str.lines().rev();
    let mut input_stacks = stack_iter
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let max_input_stack = input_stacks.pop().unwrap().unwrap();
    let mut stacks: Stack = vec![vec![]; max_input_stack];

    stack_iter.for_each(|l| {
        l.chars().skip(1).enumerate().for_each(|(i, c)| {
            if i % 4 == 0 && c != ' ' {
                stacks[i / 4].push(c);
            }
        });
    });

    let moves = move_str
        .lines()
        .filter_map(|l| {
            let s: Vec<&str> = l.split_ascii_whitespace().collect();
            Some((
                s[1].parse::<usize>().ok()?,
                s[3].parse::<usize>().ok()?,
                s[5].parse::<usize>().ok()?,
            ))
        })
        .collect::<Vec<_>>();

    (stacks, moves)
}

fn get_result(stacks: &Stack) -> String {
    stacks
        .iter()
        .filter_map(|m| m.last())
        .collect::<Vec<_>>()
        .into_iter()
        .collect()
}

fn move_crates(stacks: &mut Stack, moves: &[Move], move_in_packs: bool) -> Stack {
    moves.iter().for_each(|(amount, from, to)| {
        if move_in_packs {
            let from_stack = &mut stacks[*from - 1];
            let crates = from_stack.split_off(from_stack.len() - amount);

            stacks[*to - 1].extend(crates.iter());
        } else {
            for _ in 0..*amount {
                let c = stacks[*from - 1].pop().unwrap();
                stacks[*to - 1].push(c);
            }
        }
    });

    stacks.clone()
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse(input);

    Some(get_result(&move_crates(&mut stacks, &moves, false)))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse(input);

    Some(get_result(&move_crates(&mut stacks, &moves, true)))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
