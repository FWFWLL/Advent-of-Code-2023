use std::collections::BTreeMap;

#[derive(Debug)]
struct Number {
    positions: Vec<(usize, usize)>,
    value: u32,
}

#[derive(Debug)]
enum Symbol {
    Empty,
    Symbol,
    Digit(u32),
}

pub fn process(input: &str) -> String {
    let input = input.lines()
        .map(|line| {
            let mut line = line.chars()
                .collect::<String>();

            line.push_str(".\n");

            line
        })
        .collect::<String>();

    let schematic: BTreeMap<(usize, usize), Symbol> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, character)| {
                    let symbol = match character {
                        c if c.is_ascii_digit() => Symbol::Digit(c.to_digit(10).unwrap()),
                        '.' => Symbol::Empty,
                        _ => Symbol::Symbol,
                    };

                    ((y, x), symbol)
                })
        }).collect();

    let mut digits: Vec<Vec<((usize, usize), u32)>> = vec![];
    schematic.iter()
        .for_each(|(&(x, y), symbol)| {
            if let Symbol::Digit(num) = symbol {
                match digits.iter_mut().last() {
                    Some(list) => {
                        list.push(((x, y), *num));
                    },
                    None => digits.push(vec![((x, y), *num)]),
                };
            } else {
                digits.push(vec![]);
            }
        });

    let numbers: Vec<Number> = digits.iter()
        .filter(|list| !list.is_empty())
        .map(|digit_list| {
            let positions = digit_list.iter()
                .map(|(pos, _)| *pos)
                .collect();

            let value = digit_list.iter()
                .fold(0, |acc, (_, digit)| acc * 10 + digit);

            Number {
                positions,
                value,
            }
        })
        .collect();

    let output: u32 = numbers.iter()
        .map(|number| {
            let coords_to_check: Vec<(usize, usize)> = number.positions
                .iter()
                .flat_map(|&(x, y)| {
                    let positions = [
                        (1, 0),
                        (1, -1),
                        (0, -1),
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                    ];

                    positions.iter().map(move |(i, j)| (x as i32 + i, y as i32 + j)).collect::<Vec<_>>()
                })
                .filter(|&(x, y)| x >= 0 || y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .collect();

            let found = coords_to_check.iter().find(|coord| {
                matches!(schematic.get(coord), Some(Symbol::Symbol))
            });

            if found.is_some() {
                number.value
            } else {
                0
            }
        })
        .sum();

    output.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_1.txt");
    let result = process(input);

    assert_eq!(result, "4361");
}
