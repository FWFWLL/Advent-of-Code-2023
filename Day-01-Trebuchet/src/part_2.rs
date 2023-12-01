use rayon::prelude::*;

const REPLACEMENTS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

pub fn process(input: &str) -> String {
    let output: u32 = input.par_lines()
        .map(|line| {
            let mut line = line.to_owned();
            REPLACEMENTS.iter().for_each(|(from, to)| line = line.replace(from, to));

            let digits: Vec<u32> = line.par_chars()
                .filter_map(|char| char.to_digit(10))
                .collect();

            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            first * 10 + last
        })
        .sum();

    output.to_string()
}

#[test]
fn test() {
    let result = process("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");

    assert_eq!(result, "281");
}
