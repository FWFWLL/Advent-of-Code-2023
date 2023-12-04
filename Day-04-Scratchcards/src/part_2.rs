use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn process(input: &str) -> String {
    let mut card_counts: BTreeMap<usize, usize> = BTreeMap::new();
    let max_card_id = input.lines().count() + 1;

    input.lines()
        .filter_map(|line| line.split_once(": "))
        .for_each(|(card, numbers)| {
            let card_id = card.split_whitespace().last().unwrap().parse::<usize>().unwrap();
            match card_counts.get(&card_id) {
                Some(count) => card_counts.insert(card_id, count + 1),
                None => card_counts.insert(card_id, 1),
            };

            let (winning, draws) = numbers.split_once(" | ").unwrap();

            let winning = winning.split_whitespace()
                .filter_map(|number| number.parse().ok())
                .collect::<BTreeSet<u32>>();

            draws.split_whitespace()
                .filter_map(|number| number.parse().ok())
                .filter(|number| winning.contains(number))
                .enumerate()
                .for_each(|(i, _)| {
                    let next_card_id = 1 + card_id + i;
                    match card_counts.get(&next_card_id) {
                        Some(count) if next_card_id < max_card_id => card_counts.insert(next_card_id, count + card_counts.get(&card_id).unwrap()),
                        None if next_card_id < max_card_id => card_counts.insert(next_card_id, *card_counts.get(&card_id).unwrap()),
                        _ => None,
                    };
                });
        });

    let output: usize = card_counts.values().sum();

    output.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_2.txt");
    let result = process(input);

    assert_eq!(result, "30");
}
