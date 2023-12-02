pub fn process(input: &str) -> String {
    input.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_2.txt");
    let result = process(input);

    assert_eq!(result, "");
}
