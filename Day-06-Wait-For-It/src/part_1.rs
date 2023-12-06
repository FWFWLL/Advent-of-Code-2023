#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

pub fn process(input: &str) -> String {
    let (time_str, dist_str) = input.split_once('\n').unwrap();

    let times: Vec<u32> = time_str.strip_prefix("Time:").unwrap()
        .split_whitespace()
        .filter_map(|time| time.parse().ok())
        .collect();

    let distances: Vec<u32> = dist_str.strip_prefix("Distance:").unwrap()
        .split_whitespace()
        .filter_map(|distance| distance.parse().ok())
        .collect();

    let races: Vec<Race> = times.into_iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            time,
            distance,
        })
        .collect();

    let output: usize = races.iter()
        .map(|race| {
            (1..race.time - 1).filter(|i| {
                race.distance < i * (race.time - i)
            })
            .count()
        })
        .product::<usize>();

    output.to_string()
}

#[test]
fn test() {
    let input = include_str!("../input/test_1.txt");
    let result = process(input);

    assert_eq!(result, "288");
}
