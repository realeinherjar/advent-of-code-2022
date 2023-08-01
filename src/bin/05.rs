use regex::Regex;

#[derive(Debug, PartialEq)]
struct Supplies {
    len: usize,
    stacks: Vec<Stack>,
}

#[derive(Debug, PartialEq)]
struct Stack {
    crates: Vec<char>,
    index: usize,
}

#[derive(Debug, PartialEq)]
struct Move {
    from: usize,
    to: usize,
    quantity: usize,
}

fn parse_crate_line(input: &str) -> Vec<Option<char>> {
    let re = Regex::new(r"\[(\w{1})\]").unwrap();
    input
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .map(|c| {
            re.captures(c.as_str())
                .map(|caps| caps[1].chars().next().unwrap())
        })
        .collect()
}

fn crates_len(input: &str) -> usize {
    let crates_line: Vec<&str> = input.lines().last().unwrap().split_whitespace().collect();
    crates_line.len()
}

fn parse_crates(input: &str) -> Supplies {
    let len: usize = crates_len(input);
    let mut stacks = Vec::<Stack>::with_capacity(len);
    for i in 0..len {
        stacks.push(Stack {
            index: i,
            crates: Vec::<char>::new(),
        });
    }
    let re = Regex::new(r"\d").unwrap();
    for line in input.lines() {
        if re.is_match(line) {
            break;
        }
        let crates = parse_crate_line(line);
        for (i, c) in crates.iter().enumerate() {
            if let Some(c) = c {
                stacks[i].crates.insert(0, *c);
            }
        }
    }

    Supplies { len, stacks }
}

fn parse_moves(input: &str) -> Vec<Move> {
    let re = Regex::new(r"move (\d{1,2}) from (\d{1}) to (\d{1})").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Move {
                quantity: caps[1].parse::<usize>().unwrap(),
                from: caps[2].parse::<usize>().unwrap() - 1,
                to: caps[3].parse::<usize>().unwrap() - 1,
            }
        })
        .collect()
}

impl Supplies {
    fn move_stack(&mut self, m: Move) {
        for _ in 1..=m.quantity {
            let from_char: char = self.stacks[m.from].crates.pop().unwrap();
            self.stacks[m.to].crates.push(from_char);
        }
    }
    fn move_stack_part2(&mut self, m: Move) {
        let mut v: Vec<char> = Vec::with_capacity(m.quantity);
        for _ in 1..=m.quantity {
            v.insert(0, self.stacks[m.from].crates.pop().unwrap());
        }
        self.stacks[m.to].crates.extend(v);
    }
}

fn get_top_crates(supplies: &Supplies) -> String {
    supplies
        .stacks
        .iter()
        .map(|stack| stack.crates.last().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let parsed_str = input.split("\n\n").collect::<Vec<&str>>();
    let mut crates = parse_crates(parsed_str[0]);
    let moves = parse_moves(parsed_str[1]);
    for m in moves {
        crates.move_stack(m);
    }
    let answer = get_top_crates(&crates);
    Some(answer)
}

pub fn part_two(input: &str) -> Option<String> {
    let parsed_str = input.split("\n\n").collect::<Vec<&str>>();
    let mut crates = parse_crates(parsed_str[0]);
    let moves = parse_moves(parsed_str[1]);
    for m in moves {
        crates.move_stack_part2(m);
    }
    let answer = get_top_crates(&crates);
    Some(answer)
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
    fn test_parse_crate_line() {
        let input_full = "[Z] [M] [P]";
        let input_half = "[N] [C]    ";
        let input_empty = "    [D]    ";
        assert_eq!(
            vec![Some('Z'), Some('M'), Some('P')],
            parse_crate_line(input_full)
        );
        assert_eq!(
            vec![Some('N'), Some('C'), None],
            parse_crate_line(input_half)
        );
        assert_eq!(vec![None, Some('D'), None], parse_crate_line(input_empty));
    }

    #[test]
    fn test_crates_len() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3";
        assert_eq!(3, crates_len(input));
    }

    #[test]
    fn test_parse_crates() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3";
        let supplies = Supplies {
            len: 3,
            stacks: vec![
                Stack {
                    index: 0,
                    crates: vec!['Z', 'N'],
                },
                Stack {
                    index: 1,
                    crates: vec!['M', 'C', 'D'],
                },
                Stack {
                    index: 2,
                    crates: vec!['P'],
                },
            ],
        };
        assert_eq!(supplies, parse_crates(input));
    }

    #[test]
    fn test_parse_moves() {
        let input =
            "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";
        assert_eq!(
            vec![
                Move {
                    quantity: 1,
                    from: 1,
                    to: 0
                },
                Move {
                    quantity: 3,
                    from: 0,
                    to: 2
                },
                Move {
                    quantity: 2,
                    from: 1,
                    to: 0
                },
                Move {
                    quantity: 1,
                    from: 0,
                    to: 1
                },
            ],
            parse_moves(input)
        );
    }

    #[test]
    fn test_supplies_move() {
        let mut supplies = Supplies {
            len: 3,
            stacks: vec![
                Stack {
                    index: 0,
                    crates: vec!['A', 'B', 'C'],
                },
                Stack {
                    index: 1,
                    crates: vec!['D', 'E'],
                },
                Stack {
                    index: 2,
                    crates: vec!['F'],
                },
            ],
        };
        supplies.move_stack(Move {
            quantity: 3,
            from: 0,
            to: 2,
        });
        assert_eq!(
            supplies,
            Supplies {
                len: 3,
                stacks: vec![
                    Stack {
                        index: 0,
                        crates: vec![],
                    },
                    Stack {
                        index: 1,
                        crates: vec!['D', 'E'],
                    },
                    Stack {
                        index: 2,
                        crates: vec!['F', 'C', 'B', 'A'],
                    },
                ],
            }
        )
    }

    #[test]
    fn test_supplies_move_part2() {
        let mut supplies = Supplies {
            len: 3,
            stacks: vec![
                Stack {
                    index: 0,
                    crates: vec!['A', 'B', 'C'],
                },
                Stack {
                    index: 1,
                    crates: vec!['D', 'E'],
                },
                Stack {
                    index: 2,
                    crates: vec!['F'],
                },
            ],
        };
        supplies.move_stack_part2(Move {
            quantity: 3,
            from: 0,
            to: 2,
        });
        assert_eq!(
            supplies,
            Supplies {
                len: 3,
                stacks: vec![
                    Stack {
                        index: 0,
                        crates: vec![],
                    },
                    Stack {
                        index: 1,
                        crates: vec!['D', 'E'],
                    },
                    Stack {
                        index: 2,
                        crates: vec!['F', 'A', 'B', 'C'],
                    },
                ],
            }
        )
    }

    #[test]
    fn test_get_top_crates() {
        let supplies = Supplies {
            len: 3,
            stacks: vec![
                Stack {
                    index: 0,
                    crates: vec!['A', 'B', 'C'],
                },
                Stack {
                    index: 1,
                    crates: vec!['D', 'E'],
                },
                Stack {
                    index: 2,
                    crates: vec!['F'],
                },
            ],
        };
        assert_eq!(get_top_crates(&supplies), "CEF");
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
