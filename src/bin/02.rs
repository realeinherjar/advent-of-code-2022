use std::char;

#[derive(Debug, PartialEq)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug, PartialEq)]
enum RockPaperScissorsResult {
    Win,
    Lose,
    Draw,
}

fn match_rps(c: char) -> RockPaperScissors {
    match c {
        'A' => RockPaperScissors::Rock,
        'B' => RockPaperScissors::Paper,
        'C' => RockPaperScissors::Scissors,
        'X' => RockPaperScissors::Rock,
        'Y' => RockPaperScissors::Paper,
        'Z' => RockPaperScissors::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn match_result(c: char) -> RockPaperScissorsResult {
    match c {
        'Z' => RockPaperScissorsResult::Win,
        'X' => RockPaperScissorsResult::Lose,
        'Y' => RockPaperScissorsResult::Draw,
        _ => panic!("Invalid input"),
    }
}

fn match_chars(v: Vec<char>) -> Vec<RockPaperScissors> {
    assert!(v.len() == 2);
    v.iter().map(|c| match_rps(*c)).collect::<Vec<_>>()
}

fn match_chars_part2(v: Vec<char>) -> (RockPaperScissors, RockPaperScissorsResult) {
    assert!(v.len() == 2);
    let first = match_rps(*v.first().unwrap());
    let second = match_result(*v.last().unwrap());
    (first, second)
}

fn parse_moves(moves: Vec<RockPaperScissors>) -> u32 {
    assert!(moves.len() == 2);
    match (moves.first().unwrap(), moves.last().unwrap()) {
        (RockPaperScissors::Rock, RockPaperScissors::Paper) => 8,
        (RockPaperScissors::Rock, RockPaperScissors::Scissors) => 3,
        (RockPaperScissors::Paper, RockPaperScissors::Rock) => 1,
        (RockPaperScissors::Paper, RockPaperScissors::Scissors) => 9,
        (RockPaperScissors::Scissors, RockPaperScissors::Rock) => 7,
        (RockPaperScissors::Scissors, RockPaperScissors::Paper) => 2,
        (RockPaperScissors::Rock, RockPaperScissors::Rock) => 4,
        (RockPaperScissors::Paper, RockPaperScissors::Paper) => 5,
        (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => 6,
    }
}

fn parse_moves_part2(moves: (RockPaperScissors, RockPaperScissorsResult)) -> u32 {
    match (moves.0, moves.1) {
        (RockPaperScissors::Scissors, RockPaperScissorsResult::Win) => 7,
        (RockPaperScissors::Rock, RockPaperScissorsResult::Win) => 8,
        (RockPaperScissors::Paper, RockPaperScissorsResult::Win) => 9,
        (RockPaperScissors::Rock, RockPaperScissorsResult::Draw) => 4,
        (RockPaperScissors::Paper, RockPaperScissorsResult::Draw) => 5,
        (RockPaperScissors::Scissors, RockPaperScissorsResult::Draw) => 6,
        (RockPaperScissors::Rock, RockPaperScissorsResult::Lose) => 3,
        (RockPaperScissors::Scissors, RockPaperScissorsResult::Lose) => 2,
        (RockPaperScissors::Paper, RockPaperScissorsResult::Lose) => 1,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.replace(' ', ""))
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(match_chars)
        .map(parse_moves)
        .reduce(|a, b| a + b)
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.replace(' ', ""))
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(match_chars_part2)
        .map(parse_moves_part2)
        .reduce(|a, b| a + b)
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
    fn test_match_rps() {
        assert_eq!(match_rps('A'), RockPaperScissors::Rock);
        assert_eq!(match_rps('B'), RockPaperScissors::Paper);
        assert_eq!(match_rps('C'), RockPaperScissors::Scissors);
        assert_eq!(match_rps('X'), RockPaperScissors::Rock);
        assert_eq!(match_rps('Y'), RockPaperScissors::Paper);
        assert_eq!(match_rps('Z'), RockPaperScissors::Scissors);
    }

    #[test]
    fn test_match_result() {
        assert_eq!(match_result('X'), RockPaperScissorsResult::Lose);
        assert_eq!(match_result('Y'), RockPaperScissorsResult::Draw);
        assert_eq!(match_result('Z'), RockPaperScissorsResult::Win);
    }

    #[test]
    fn test_match_chars() {
        assert_eq!(
            match_chars(vec!['A', 'C']),
            vec![RockPaperScissors::Rock, RockPaperScissors::Scissors]
        );
    }

    #[test]
    fn test_match_chars_part2() {
        assert_eq!(
            match_chars_part2(vec!['A', 'X']),
            (RockPaperScissors::Rock, RockPaperScissorsResult::Lose)
        );
    }

    #[test]
    fn test_parse_moves() {
        assert_eq!(
            parse_moves(vec![RockPaperScissors::Rock, RockPaperScissors::Paper]),
            8
        );
        assert_eq!(
            parse_moves(vec![RockPaperScissors::Rock, RockPaperScissors::Scissors]),
            3
        );
        assert_eq!(
            parse_moves(vec![RockPaperScissors::Paper, RockPaperScissors::Rock]),
            1
        );
        assert_eq!(
            parse_moves(vec![RockPaperScissors::Paper, RockPaperScissors::Scissors]),
            9
        );
        assert_eq!(
            parse_moves(vec![RockPaperScissors::Scissors, RockPaperScissors::Rock]),
            7
        );
        assert_eq!(
            parse_moves(vec![RockPaperScissors::Scissors, RockPaperScissors::Paper]),
            2
        );
        assert_eq!(
            parse_moves(vec![RockPaperScissors::Rock, RockPaperScissors::Rock]),
            4
        );
        assert_eq!(
            parse_moves(vec![RockPaperScissors::Paper, RockPaperScissors::Paper]),
            5
        );
        assert_eq!(
            parse_moves(vec![
                RockPaperScissors::Scissors,
                RockPaperScissors::Scissors
            ]),
            6
        );
    }

    #[test]
    fn test_parse_moves_part2() {
        assert_eq!(
            parse_moves_part2((RockPaperScissors::Scissors, RockPaperScissorsResult::Win)),
            7
        );
        assert_eq!(
            parse_moves_part2((RockPaperScissors::Rock, RockPaperScissorsResult::Win)),
            8
        );
        assert_eq!(
            parse_moves_part2((RockPaperScissors::Paper, RockPaperScissorsResult::Win)),
            9
        );
        assert_eq!(
            parse_moves_part2((RockPaperScissors::Rock, RockPaperScissorsResult::Draw)),
            4
        );
        assert_eq!(
            parse_moves_part2((RockPaperScissors::Paper, RockPaperScissorsResult::Draw)),
            5
        );
        assert_eq!(
            parse_moves_part2((RockPaperScissors::Scissors, RockPaperScissorsResult::Draw)),
            6
        );
        assert_eq!(
            parse_moves_part2((RockPaperScissors::Rock, RockPaperScissorsResult::Lose)),
            3
        );
        assert_eq!(
            parse_moves_part2((RockPaperScissors::Scissors, RockPaperScissorsResult::Lose)),
            2
        );
        assert_eq!(
            parse_moves_part2((RockPaperScissors::Paper, RockPaperScissorsResult::Lose)),
            1
        );
    }

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
