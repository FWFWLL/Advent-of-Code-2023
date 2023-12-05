#[derive(Debug)]
struct Map {
    dst: usize,
    src: usize,
    range: usize,
}

pub fn process(input: &str) -> String {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds: Vec<usize> = seeds.strip_prefix("seeds: ").unwrap()
        .split_whitespace()
        .filter_map(|seed| seed.parse().ok())
        .collect();

    let maps: Vec<Vec<Map>> = maps.split("\n\n")
        .map(|block| {
            block.lines()
                .skip(1)
                .map(|line| {
                    let numbers: Vec<usize> = line.split_whitespace()
                        .filter_map(|num| num.parse().ok())
                        .collect();

                    Map {
                        dst: numbers[0],
                        src: numbers[1],
                        range: numbers[2],
                    }
                })
                .collect()
        })
        .collect();

    let output = seeds.into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |curr, maps| {
                if let Some(rule) = maps.iter().find(|map| curr >= map.src && curr <= map.src + map.range) {
                    curr + rule.dst - rule.src
                } else {
                    curr
                }
            })
        })
        .min().unwrap();

    output.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_1.txt");
    let result = process(input);

    assert_eq!(result, "35");
}
