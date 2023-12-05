#[derive(Debug, Clone, Copy)]
struct Range {
    from: isize,
    to: isize,
}

#[derive(Debug)]
struct Map {
    dst: isize,
    src: isize,
    range: isize,
}

pub fn process(input: &str) -> String{
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds: Vec<isize> = seeds.strip_prefix("seeds: ").unwrap()
        .split_whitespace()
        .filter_map(|seed| seed.parse().ok())
        .collect();

    let seeds: Vec<Range> = seeds.chunks(2)
        .map(|chunk| {
            let mut chunk = chunk.into_iter();
            let from = chunk.next().copied().unwrap();
            let range = chunk.next().copied().unwrap();

            Range {
                from,
                to: from + range,
            }
        })
        .collect();

    let maps: Vec<Vec<Map>> = maps.split("\n\n")
        .map(|block| {
            let mut maps: Vec<Map> = block.lines()
                .skip(1)
                .map(|line| {
                    let numbers: Vec<isize> = line.split_whitespace()
                        .filter_map(|num| num.parse().ok())
                        .collect();

                    Map {
                        dst: numbers[0],
                        src: numbers[1],
                        range: numbers[2],
                    }
                })
                .collect();

            maps.sort_by(|a, b| a.src.cmp(&b.src));

            maps
        })
        .collect();

    let mut curr_ranges = seeds;
    maps.iter().for_each(|map| {
        let mut new_ranges: Vec<Range> = Vec::new();

        curr_ranges.iter_mut().for_each(|range| {
            let mut curr = range.clone();

            map.iter().for_each(|rule| {
                let offset = rule.dst - rule.src;

                let rule_applies = curr.from <= curr.to && curr.from <= rule.src + rule.range && curr.to >= rule.src;

                if rule_applies {
                    if curr.from < rule.src {
                        new_ranges.push(Range {
                            from: curr.from,
                            to: rule.src - 1,
                        });

                        curr.from = rule.src;

                        if curr.to < rule.src + rule.range {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: curr.to + offset,
                            });

                            curr.from = curr.to + 1;
                        } else {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: rule.src + rule.range - 1 + offset,
                            });

                            curr.from = rule.src + rule.range;
                        }
                    } else if curr.to < rule.src + rule.range {
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: curr.to + offset,
                        });

                        curr.from = curr.to + 1;
                    } else {
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: rule.src + rule.range - 1 + offset,
                        });

                        curr.from = rule.src + rule.range;
                    }
                }
            });

            if curr.from <= curr.to {
                new_ranges.push(Range {
                    from: curr.from,
                    to: curr.to,
                });
            }
        });

        curr_ranges = new_ranges;
    });

    let output = curr_ranges.iter()
        .map(|range| range.from)
        .min().unwrap();

    output.to_string()
} 

#[test]
fn test() {
    let input = include_str!("../input/test_2.txt");
    let result = process(input);

    assert_eq!(result, "46");
}
