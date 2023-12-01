pub fn process(input: &str) -> String {
    let replacement_table = [
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

    let output: u32 = input.lines()
        .map(|line| {
            let mut line = line.to_owned();

            replacement_table.iter()
                .for_each(|(from, to)| {
                    line = line.replace(from, to);
                });

            let digits: Vec<char> = line.chars()
                .filter(|char| char.is_digit(10))
                .collect();

            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        })
        .filter_map(|value| value.parse::<u32>().ok())
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
