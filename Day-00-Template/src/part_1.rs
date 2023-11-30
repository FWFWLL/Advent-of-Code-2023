pub fn process(input: &str) -> String {
    input.to_string()
}

#[test]
fn test() {
    let result = process("");

    assert_eq!(result, "");
}
