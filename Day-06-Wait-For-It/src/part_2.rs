pub fn process(input: &str) -> String {
    let (time_str, dist_str) = input.split_once('\n').unwrap();

    let time: String = time_str.strip_prefix("Time:").unwrap()
        .split_whitespace()
        .collect();

    let time: usize = time.parse().unwrap();

    let distance: String = dist_str.strip_prefix("Distance:").unwrap()
        .split_whitespace()
        .collect();

    let distance: usize = distance.parse().unwrap();

    let output = (1..time - 1).filter(|i| distance < i * (time - i)).count();

    output.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_2.txt");
    let result = process(input);

    assert_eq!(result, "71503");
}

