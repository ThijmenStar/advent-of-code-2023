use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Mapping = (i64, i64, i64);

#[derive(Debug, PartialEq, Clone)]
pub struct Almanac {
    seeds: Vec<i64>,

    seeds_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Almanac {
    let re_seeds = Regex::new(r"seeds:\s(?<seeds>(?:\d+\s*)*)").unwrap();
    let re_mappings = Regex::new(r"(?:\w|-)* map:\n(?<mappings>(?:\d+ \d+ \d+\s?)*)").unwrap();

    let mut entries = input.split("\n\n");

    let seeds: Vec<i64> = re_seeds.captures(entries.next().unwrap()).unwrap()["seeds"]
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
pub fn solve_part1(input: &Almanac) -> i64 {
    let maps = &[
        &input.seeds_to_soil,
        &input.soil_to_fertilizer,
        &input.fertilizer_to_water,
        &input.water_to_light,
        &input.light_to_temperature,
        &input.temperature_to_humidity,
        &input.humidity_to_location,
    ];

    let mut locations: Vec<i64> = vec![];

    for seed in &input.seeds {
        locations.push(follow_maps(maps, *seed));
    }

    return *locations.iter().min().unwrap();
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Almanac) -> i64 {
    let maps = &[
        &input.seeds_to_soil,
        &input.soil_to_fertilizer,
        &input.fertilizer_to_water,
        &input.water_to_light,
        &input.light_to_temperature,
        &input.temperature_to_humidity,
        &input.humidity_to_location,
    ];

    let mut min_location = i64::MAX;


    let total_seeds: i64 = input.seeds.chunks_exact(2).map(|s| s[1]).sum();
    let mut processed_seeds = 0;


    println!("Total seeds: {}", total_seeds);
    for seed in input.seeds.chunks_exact(2) {
        for i in seed[0]..seed[0]+seed[1] {
            min_location = min_location.min(follow_maps(maps, i));
            if processed_seeds % 100_000_000 == 0 {
                println!("Progress: {:.0}%", (processed_seeds as f64 / total_seeds as f64)*100.0);
            }

            processed_seeds += 1;
        }
    }

    return min_location;

}

fn follow_maps(maps: &[&Vec<Mapping>], src: i64) -> i64 {
    let mut dst = src;
    for m in maps {
        dst = map_value(m, dst);
    }

    return dst;
}

fn map_value(mapping: &Vec<Mapping>, key: i64) -> i64 {
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
    fn solve_part1_with_part2() {
        const INPUT: &str = include_str!("../input/2023/day5.txt");
        let mut input = parse_input(INPUT);

        input.seeds = input.seeds.iter().map(|&s| [s, 1]).flatten().collect();

        assert_eq!(solve_part2(&input), 178159714)


    }


    #[test]
    fn print_locations() {
        const INPUT: &str = include_str!("../input/2023/day5.txt");
        let input = parse_input(INPUT);

        let mut counter = 0;

        for i in input.seeds.chunks_exact(2) {
            counter += i[1];
        }

        println!("{}", counter);




    }
}
