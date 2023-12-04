pub fn process(input: &str) -> String {
    let output: u32 = input.lines()
        .map(|line| {
            let mut max_cube_counts = (0, 0, 0);

            line.split_once(": ").unwrap()
                .0
                .split("; ")
                .for_each(|subset| {
                    subset.split(", ")
                        .filter_map(|cubes| cubes.split_once(' '))
                        .for_each(|(amount, cube_type)| {
                            let amount: u32 = amount.parse().unwrap();

                            match cube_type {
                                "red" => max_cube_counts.0 = max_cube_counts.0.max(amount),
                                "green" => max_cube_counts.1 = max_cube_counts.1.max(amount),
                                "blue" => max_cube_counts.2 = max_cube_counts.2.max(amount),
                                _ => (),
                            }
                        });
                });

            max_cube_counts.0 * max_cube_counts.1 * max_cube_counts.2
        })
        .sum();

    output.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_2.txt");
    let result = process(input);

    assert_eq!(result, "2286");
}
