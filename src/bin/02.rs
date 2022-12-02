pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, line| {
        let plays = line.split(' ').collect::<Vec<&str>>();
        let their_play = match plays[0] {
            "A" => "rock",
            "B" => "paper",
            "C" => "scissors",
            _ => unreachable!("Invalid play"),
        };
        let my_play = match plays[1] {
            "X" => "rock",
            "Y" => "paper",
            "Z" => "scissors",
            _ => unreachable!("Invalid play"),
        };
        let points_for_move = match my_play {
            "rock" => 1,
            "paper" => 2,
            "scissors" => 3,
            _ => unreachable!("Invalid play"),
        };
        let points_for_win = 6;
        let points_for_draw = 3;

        match (their_play, my_play) {
            ("rock", "paper") | ("paper", "scissors") | ("scissors", "rock") => {
                acc + points_for_win + points_for_move
            }
            ("scissors", "scissors") | ("rock", "rock") | ("paper", "paper") => {
                acc + points_for_draw + points_for_move
            }
            _ => acc + points_for_move,
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, line| {
        let plays = line.split(' ').collect::<Vec<&str>>();
        let their_play = match plays[0] {
            "A" => "rock",
            "B" => "paper",
            "C" => "scissors",
            _ => unreachable!("Invalid play"),
        };
        let needed_outcome = match plays[1] {
            "X" => "lose",
            "Y" => "draw",
            "Z" => "win",
            _ => unreachable!("Invalid play"),
        };
        let points_for_win = 6;
        let points_for_draw = 3;

        fn get_points_for_move(my_play: &str) -> u32 {
            match my_play {
                "rock" => 1,
                "paper" => 2,
                "scissors" => 3,
                _ => unreachable!("Invalid play"),
            }
        }

        fn get_winning_move(play: &str) -> &str {
            match play {
                "rock" => "paper",
                "paper" => "scissors",
                "scissors" => "rock",
                _ => unreachable!("Invalid play"),
            }
        }

        fn get_losing_move(play: &str) -> &str {
            match play {
                "rock" => "scissors",
                "paper" => "rock",
                "scissors" => "paper",
                _ => unreachable!("Invalid play"),
            }
        }

        if needed_outcome == "win" {
            acc + points_for_win + get_points_for_move(get_winning_move(their_play))
        } else if needed_outcome == "draw" {
            acc + points_for_draw + get_points_for_move(their_play)
        } else {
            acc + get_points_for_move(get_losing_move(their_play))
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
