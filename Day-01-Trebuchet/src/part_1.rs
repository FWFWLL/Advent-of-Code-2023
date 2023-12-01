use rayon::prelude::*;

pub fn process(input: &str) -> String {
    let output: u32 = input.par_lines()
        .map(|line| {
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
    let result = process("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");

    assert_eq!(result, "142");
}
