use std::collections::BTreeSet;

pub fn process(input: &str) -> String {
    let output: u32 = input.lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(_, numbers)| numbers.split_once(" | "))
        .map(|(winning, draws)| {
            let winning = winning.split_whitespace()
                .filter_map(|number| number.parse().ok())
                .collect::<BTreeSet<u32>>();

            let points = draws.split_whitespace()
                .filter_map(|number| number.parse().ok())
                .filter(|number| winning.contains(number))
                .fold(1, |acc, _| acc * 2);

            points / 2
        })
        .sum();

    output.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_1.txt");
    let result = process(input);

    assert_eq!(result, "13");
}
