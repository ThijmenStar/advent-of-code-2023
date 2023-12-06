use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Mapping = (usize, usize, usize);

#[derive(Debug, PartialEq, Clone)]
pub struct Almanac {
    seeds: Vec<usize>,

    seeds_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl Almanac {
    fn sort_maps(&mut self) {
        self.seeds_to_soil.sort_by_key(|&(_, src, _)| src);
        self.soil_to_fertilizer.sort_by_key(|&(_, src, _)| src);
        self.fertilizer_to_water.sort_by_key(|&(_, src, _)| src);
        self.water_to_light.sort_by_key(|&(_, src, _)| src);
        self.light_to_temperature.sort_by_key(|&(_, src, _)| src);
        self.temperature_to_humidity.sort_by_key(|&(_, src, _)| src);
        self.humidity_to_location.sort_by_key(|&(_, src, _)| src);
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Almanac {
    let re_seeds = Regex::new(r"seeds:\s(?<seeds>(?:\d+\s*)*)").unwrap();
    let re_mappings = Regex::new(r"(?:\w|-)* map:\n(?<mappings>(?:\d+ \d+ \d+\s?)*)").unwrap();

    let mut entries = input.split("\n\n");

    let seeds: Vec<usize> = re_seeds.captures(entries.next().unwrap()).unwrap()["seeds"]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mappings: Vec<Vec<Mapping>> = entries
        .map(|s| {
            let m: &str = &re_mappings.captures(s).unwrap()["mappings"];
            m.lines()
                .map(|l| {
                    let mut split = l.splitn(3, " ");
                    (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let mut mappings_iter = mappings.into_iter();

    return Almanac {
        seeds,
        seeds_to_soil: mappings_iter.next().unwrap(),
        soil_to_fertilizer: mappings_iter.next().unwrap(),
        fertilizer_to_water: mappings_iter.next().unwrap(),
        water_to_light: mappings_iter.next().unwrap(),
        light_to_temperature: mappings_iter.next().unwrap(),
        temperature_to_humidity: mappings_iter.next().unwrap(),
        humidity_to_location: mappings_iter.next().unwrap(),
    };
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Almanac) -> usize {
    let maps = &[
        &input.seeds_to_soil,
        &input.soil_to_fertilizer,
        &input.fertilizer_to_water,
        &input.water_to_light,
        &input.light_to_temperature,
        &input.temperature_to_humidity,
        &input.humidity_to_location,
    ];

    let mut locations: Vec<usize> = vec![];

    for seed in &input.seeds {
        locations.push(follow_maps(maps, *seed));
    }

    return *locations.iter().min().unwrap();
}

#[aoc(day5, part2)]
pub fn solve_part2(input_ref: &Almanac) -> usize {
    let mut input = input_ref.clone();
    input.sort_maps();

    let maps = &[
        &input.seeds_to_soil,
        &input.soil_to_fertilizer,
        &input.fertilizer_to_water,
        &input.water_to_light,
        &input.light_to_temperature,
        &input.temperature_to_humidity,
        &input.humidity_to_location,
    ];

    return input
        .seeds
        .chunks_exact(2)
        .map(|i| min_locations(maps, 0, i[0], i[1]))
        .min()
        .unwrap();
}

fn follow_maps(maps: &[&Vec<Mapping>], src: usize) -> usize {
    let mut dst = src;
    for m in maps {
        dst = map_value(m, dst);
    }

    return dst;
}

fn min_locations(maps: &[&Vec<Mapping>], map_index: usize, start: usize, length: usize) -> usize {
    if map_index >= maps.len() {
        // base case
        return start;
    }

    let first_range = maps[map_index]
        .iter()
        .filter(|&&(_, src, len)| start < src + len && start+length > src)
        .last();

    match first_range {
        Some(&(dst, src, len)) => {
            if src <= start {
                // first range contains start
                
                if start + length <= src + len {
                    // whole range contained
                    min_locations(maps, map_index + 1, dst + (start - src), length)
                } else {
                    // part of the range contained
                    min_locations(
                        maps,
                        map_index + 1,
                        dst + (start - src),
                        (src + len) - start,
                    )
                    .min(min_locations(
                        maps,
                        map_index,
                        src + len,
                        (start + length) - (src + len),
                    ))
                }
            } else {
                // start not contained in range
                min_locations(maps, map_index + 1, start, src - start).min(min_locations(
                    maps,
                    map_index,
                    src,
                    length - (src - start),
                ))
            }
        }
        None => min_locations(maps, map_index + 1, start, length), // no ranges after start
    }
}

fn map_value(mapping: &Vec<Mapping>, key: usize) -> usize {
    for &(dst, src, len) in mapping {
        if src <= key && key < src + len {
            return dst + (key - src);
        }
    }

    return key;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";

    #[test]
    fn parse_example() {
        let expect: Almanac = Almanac {
            seeds: vec![79, 14, 55, 13],
            seeds_to_soil: vec![(50, 98, 2), (52, 50, 48)],
            soil_to_fertilizer: vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
            fertilizer_to_water: vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
            water_to_light: vec![(88, 18, 7), (18, 25, 70)],
            light_to_temperature: vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)],
            temperature_to_humidity: vec![(0, 69, 1), (1, 0, 69)],
            humidity_to_location: vec![(60, 56, 37), (56, 93, 4)],
        };

        assert_eq!(parse_input(EXAMPLE_INPUT), expect);
    }

    #[test]
    fn solve_part1_example() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 35);
    }

    #[test]
    fn solve_part2_example() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 46);
    }

    #[test]
    fn test_min_locations() {
        
        // no mappings
        assert_eq!(min_locations(&[&vec![]], 0, 5, 5), 5);
        
        // range contained in mapping
        assert_eq!(min_locations(&[&vec![(10, 0, 10)]], 0, 5, 5), 15);
        
        // mapping just before range
        assert_eq!(min_locations(&[&vec![(10, 0, 5)]], 0, 5, 5), 5);
        
        // mapping just after range
        assert_eq!(min_locations(&[&vec![(0, 10, 5)]], 0, 5, 5), 5);
        
        // multiple mappings in range
        assert_eq!(min_locations(&[&vec![(20, 12, 5), (0, 17, 3)]], 0, 10, 20), 0);
        
        // multiple maps
//        assert_eq!(min_locations(&[&vec![(10, 10, 10)], ], map_index, start, length))
    }
}
