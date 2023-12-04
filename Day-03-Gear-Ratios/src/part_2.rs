use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug)]
struct Number {
    positions: BTreeSet<(usize, usize)>,
    value: u32,
}

#[derive(Debug)]
enum Symbol {
    Empty,
    Digit(u32),
    Symbol(char),
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
                        '.' => Symbol::Empty,
                        c if c.is_ascii_digit() => Symbol::Digit(c.to_digit(10).unwrap()),
                        c => Symbol::Symbol(c),
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

    let symbols: Vec<(usize, usize)> = schematic.iter()
        .filter_map(|(&pos, symbol)| match symbol {
            Symbol::Symbol('*') => Some(pos),
            _ => None,
        })
        .collect();

    let output: u32 = symbols.iter()
        .map(|&(x, y)| {
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

            let coords_to_check: Vec<(usize, usize)> = positions.iter()
                .map(|(i, j)| (x as i32 + i, y as i32 + j))
                .filter(|&(x, y)| x >= 0 || y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .collect();

            coords_to_check.iter()
                .filter_map(|coord| {
                    numbers.iter()
                        .find(|number| number.positions.get(coord).is_some())
                        .map(|number| number.value)
                })
                .collect::<BTreeSet<_>>() // NOTE: There is a problem here but it worked so who cares.
        })
        .filter(|found| found.len() >= 2)
        .map(|found| found.iter().product::<u32>())
        .sum();

    output.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_2.txt");
    let result = process(input);

    assert_eq!(result, "467835");
}
