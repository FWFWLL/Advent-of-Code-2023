const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn process(input: &str) -> String {
    let output: u32 = input.lines()
        .filter_map(|line| {
            let (id, subsets) = line.split_once(": ").unwrap();

            let id: u32 = id.split_whitespace()
                .last().unwrap()
                .parse().unwrap();

            let mut max_cube_counts = (0, 0, 0);
            subsets.split("; ")
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

            if max_cube_counts.0 > MAX_RED || max_cube_counts.1 > MAX_GREEN || max_cube_counts.2 > MAX_BLUE {
                None
            } else {
                Some(id)
            }
        })
        .sum();

    output.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_1.txt");
    let result = process(input);

    assert_eq!(result, "8");
}
