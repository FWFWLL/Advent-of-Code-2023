pub fn process(input: &str) -> String {
    let output: u32 = input.lines()
        .map(|line| {
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
    let result = process("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");

    assert_eq!(result, "142");
}
