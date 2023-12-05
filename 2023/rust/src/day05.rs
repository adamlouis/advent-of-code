use crate::aoc;
use crate::Day;
use std::collections::HashMap;

pub struct Day05 {}

impl Day for Day05 {
    fn num(&self) -> u8 {
        5
    }
    fn input1(&self) -> String {
        return aoc::read("input/05_input.txt");
    }
    fn input2(&self) -> String {
        return aoc::read("input/05_input.txt");
    }
    fn part1(&self, input: &String) -> String {
        format!("{}", part1(input.lines()))
    }
    fn part2(&self, input: &String) -> String {
        format!("{}", part2(input.lines()))
    }
}
fn part1(mut lines: std::str::Lines) -> u64 {
    let seeds = parse_seeds(lines.next().unwrap().to_string());
    let mappings = parse_mappings(lines.map(|l| l.to_string()).collect());
    let locs: Vec<u64> = seeds.iter().map(|s| get_location(*s, &mappings)).collect();

    *locs.iter().min().unwrap()
}

fn part2(mut lines: std::str::Lines) -> u64 {
    // this is too slow
    // rethink - work backwards, or cache each. loook at the data. do later

    let seed_ranges = parse_seeds(lines.next().unwrap().to_string());
    let mut seeds: Vec<u64> = Vec::new();

    for i in 0..seed_ranges.len() / 2 {
        let start = seed_ranges[i * 2];
        let end = seed_ranges[i * 2] + seed_ranges[i * 2 + 1];
        for j in start..end {
            seeds.push(j as u64);
        }
    }

    println!("SEEDS:{:?}", seeds.len());

    let mappings = parse_mappings(lines.map(|l| l.to_string()).collect());
    let mut cache: HashMap<u64, u64> = HashMap::new();
    let locs: Vec<u64> = seeds
        .iter()
        .map(|s| {
            if cache.contains_key(s) {
                return *cache.get(s).unwrap();
            }
            let v = get_location(*s, &mappings);

            cache.insert(*s, v);

            return v;
        })
        .collect();

    *locs.iter().min().unwrap()
}
const KEYS: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

fn get_location(seed: u64, lookup: &HashMap<String, Vec<Mapping>>) -> u64 {
    let mut ret = seed;
    for k in KEYS {
        ret = get_mapping(ret, lookup.get(k).unwrap());
    }
    ret
}

fn get_mapping(val: u64, mappings: &Vec<Mapping>) -> u64 {
    for mapping in mappings {
        if val >= mapping.src && val < mapping.src + mapping.len {
            return mapping.dst + (val - mapping.src);
        }
    }
    val
}

fn parse_seeds(line: String) -> Vec<u64> {
    // todo - make a iterator to stream nums from line
    return line[7..]
        .split(" ")
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
}

fn parse_mappings(lines: Vec<String>) -> HashMap<String, Vec<Mapping>> {
    let mut ret = HashMap::new();

    let mut key = String::from("");
    let mut mappings: Vec<Mapping> = Vec::new();

    for line in lines {
        if line == "" {
            if key != "" {
                let v = mappings;
                let k = key.clone();
                ret.insert(k, v);

                mappings = Vec::new();
                key = String::from("");
            }
        } else if line.ends_with(" map:") {
            key = line.split(" ").nth(0).unwrap().to_string();
        } else {
            let parts: Vec<u64> = line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect();

            mappings.push(Mapping {
                dst: parts[0],
                src: parts[1],
                len: parts[2],
            })
        }
    }

    if key != "" {
        ret.insert(key, mappings);
    }

    ret
}

#[derive(Debug)]
struct Mapping {
    dst: u64,
    src: u64,
    len: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_example_1() {
        let d = Day05 {};
        assert_eq!(d.part1(&EXAMPLE.to_string()), "35");
    }

    #[test]
    fn test_example_2() {
        let d = Day05 {};
        assert_eq!(d.part2(&EXAMPLE.to_string()), "46");
    }
}
