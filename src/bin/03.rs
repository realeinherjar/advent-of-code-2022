use std::char;
use std::collections::HashSet;
use std::u32;
// use std::collections::HashMap;

/*
fn priority() -> HashMap<char, u8> {
    (b'a'..=b'z')
        .map(char::from)
        .chain((b'A'..=b'Z').map(char::from))
        .zip((1..=26).chain(27..=52))
        .collect::<HashMap<char, u8>>()
}
*/
const fn priority(ch: &char) -> u8 {
    match ch {
        'a'..='z' => *ch as u8 - b'a' + 1,
        'A'..='Z' => *ch as u8 - b'A' + 27,
        _ => panic!("Invalid character"),
    }
}

fn split_string(input: &str) -> (&str, &str) {
    assert!(input.len() % 2 == 0);
    let mid = input.len() / 2;
    input.split_at(mid)
}

fn common_chars(a: &str, b: &str) -> Vec<char> {
    let set_b: HashSet<_> = b.chars().collect();
    let mut common_chars: Vec<char> = a.chars().filter(|c1| set_b.contains(c1)).collect();
    common_chars.dedup();
    common_chars
}

fn common_chars_3(a: &str, b: &str, c: &str) -> Vec<char> {
    let set_b: HashSet<_> = b.chars().collect();
    let set_c: HashSet<_> = c.chars().collect();
    let mut common_chars: Vec<char> = a
        .chars()
        .filter(|c1| set_b.contains(c1))
        .filter(|c1| set_c.contains(c1))
        .collect();
    common_chars.dedup();
    common_chars
}

fn priorities_sum(v: Vec<char>) -> u32 {
    v.iter().map(priority).sum::<u8>() as u32
}

fn priorities_sum_part2(v: Vec<&str>) -> u32 {
    assert!(v.len() == 3);
    let common_chars: Vec<char> = common_chars_3(v[0], v[1], v[2]);
    priorities_sum(common_chars)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(split_string)
            .map(|(a, b)| common_chars(a, b))
            .map(priorities_sum)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let v: Vec<&str> = input.lines().collect();
    v.chunks(3)
        .map(|v| priorities_sum_part2(v.to_vec()))
        .sum::<u32>()
        .into()
    // None
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
    fn test_priority() {
        assert_eq!(priority(&'a'), 1);
        assert_eq!(priority(&'z'), 26);
        assert_eq!(priority(&'A'), 27);
        assert_eq!(priority(&'Z'), 52);
    }

    #[test]
    fn test_split_string() {
        assert_eq!(
            split_string("vJrwpWtwJgWrhcsFMMfFFhFp"),
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
        );
        assert_eq!(
            split_string("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")
        );
    }

    #[test]
    fn test_common_chars() {
        assert_eq!(common_chars("vJrwpWtwJgWr", "hcsFMMfFFhFp"), vec!['p']);
        assert_eq!(
            common_chars("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            vec!['L']
        );
    }

    #[test]
    fn test_common_chars_3() {
        assert_eq!(
            common_chars_3(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            vec!['r']
        );
        assert_eq!(
            common_chars_3(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            vec!['Z']
        );
    }

    #[test]
    fn test_priorities_sum() {
        assert_eq!(priorities_sum(vec!['p']), 16);
        assert_eq!(priorities_sum(vec!['P']), 42);
        assert_eq!(priorities_sum(vec!['a', 'A']), 28);
        assert_eq!(priorities_sum(vec!['A', 'a']), 28);
    }
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
