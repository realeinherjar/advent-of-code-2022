fn find_marker(input: &str, distinct: usize) -> usize {
    let mut a = 0;
    let mut b = 0;
    let mut occupied = [false; 256];

    let bytes = input.as_bytes();
    while b < bytes.len() && b - a < distinct {
        let c = bytes[b] as usize;
        if occupied[c] {
            occupied[bytes[a] as usize] = false;
            a += 1;
            continue;
        }
        occupied[c] = true;
        b += 1;
    }
    b
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_marker(input, 4) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_marker(input, 14) as u32)
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
