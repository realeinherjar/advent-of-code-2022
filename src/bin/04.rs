use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::RangeInclusive;

fn parse_range(input: &str) -> std::ops::RangeInclusive<u32> {
    let ranges: Vec<u32> = input
        .split('-')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    assert!(ranges.len() == 2);
    (ranges[0])..=(ranges[1])
}
fn overlap_ranges(l: RangeInclusive<u32>, r: RangeInclusive<u32>) -> bool {
    (l.contains(r.start()) && l.contains(r.end())) || (r.contains(l.start()) && r.contains(l.end()))
}

fn overlap_ranges_bool(l: RangeInclusive<u32>, r: RangeInclusive<u32>) -> bool {
    let l_set: HashSet<u32> = HashSet::from_iter(l);
    let r_set: HashSet<u32> = HashSet::from_iter(r);
    l_set.intersection(&r_set).count() != 0
}

#[allow(dead_code)]
fn overlap_ranges_n(l: RangeInclusive<u32>, r: RangeInclusive<u32>) -> u32 {
    let l_set: HashSet<u32> = HashSet::from_iter(l);
    let r_set: HashSet<u32> = HashSet::from_iter(r);
    u32::try_from(l_set.intersection(&r_set).count()).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.split(','))
            .map(|l| {
                let ranges: Vec<&str> = l.collect();
                assert!(ranges.len() == 2);
                (parse_range(ranges[0]), parse_range(ranges[1]))
            })
            .map(|(l, r)| overlap_ranges(l, r))
            .map(u32::from)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.split(','))
            .map(|l| {
                let ranges: Vec<&str> = l.collect();
                assert!(ranges.len() == 2);
                (parse_range(ranges[0]), parse_range(ranges[1]))
            })
            // .filter(|(l, r)| overlap_ranges(l.clone(), r.clone()))
            .map(|(l, r)| overlap_ranges_bool(l, r))
            // .map(|x| u32::try_from(x).unwrap())
            .map(u32::from)
            .sum(),
    )
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
    fn test_parse_range() {
        assert_eq!(parse_range("1-4"), 1..=4);
        assert_eq!(parse_range("1-1"), 1..=1);
    }

    #[test]
    fn test_overlap_ranges() {
        assert_eq!(overlap_ranges(1..=4, 2..=5), false);
        assert_eq!(overlap_ranges(1..=4, 2..=4), true);
        assert_eq!(overlap_ranges(2..=5, 1..=4), false);
        assert_eq!(overlap_ranges(2..=4, 1..=4), true);
    }

    #[test]
    fn test_overlap_ranges_bool() {
        assert_eq!(overlap_ranges_bool(1..=4, 2..=5), true);
        assert_eq!(overlap_ranges_bool(1..=1, 1..=1), true);
        assert_eq!(overlap_ranges_bool(1..=4, 5..=7), false);
    }

    #[test]
    fn test_overlap_ranges_n() {
        assert_eq!(overlap_ranges_n(1..=4, 2..=5), 3);
        assert_eq!(overlap_ranges_n(1..=1, 1..=1), 1);
        assert_eq!(overlap_ranges_n(1..=4, 5..=7), 0);
    }

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
