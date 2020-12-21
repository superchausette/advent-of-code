use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::process;

fn parse_file(filename: &str) -> Vec<u64> {
    println!("Parsing file {}", filename);
    fs::read_to_string(filename)
        .expect("Could not load file")
        .lines()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn part1(input: &Vec<u64>) -> usize {
    let mut adapters = input.clone();
    adapters.sort();
    let mut differences: Vec<i64> = Vec::new();
    let mut current = 0;
    for adapter in adapters {
        let delta = i64::try_from(adapter).unwrap() - i64::try_from(current).unwrap();
        differences.push(delta);
        current += delta;
    }
    return differences.iter().filter(|v| **v == 1).count()
        * (differences.iter().filter(|v| **v == 3).count() + 1);
}

fn explore_paths_brute_force(
    adapters: &[u64],
    joltage: u64,
    cache: &mut HashMap<u64, u64>,
    max_joltage: u64,
) -> u64 {
    let mut ret = 0;
    if joltage == max_joltage {
        ret = 1
    } else {
        for (idx, adapter) in adapters.iter().enumerate() {
            if *adapter >= joltage + 1 && *adapter <= joltage + 3 {
                if !cache.contains_key(adapter) {
                    let found = explore_paths_brute_force(
                        &adapters[idx + 1..],
                        *adapter,
                        cache,
                        max_joltage,
                    );
                    cache.insert(*adapter, found);
                }
                ret += cache[adapter];
            } else {
                break;
            }
        }
    }
    ret
}

fn part2(input: &Vec<u64>) -> u64 {
    let mut adapters = input.clone();
    adapters.sort();
    let max_joltage = adapters.iter().max().unwrap();
    let mut cache: HashMap<u64, u64> = HashMap::new();
    explore_paths_brute_force(&adapters[..], 0, &mut cache, *max_joltage)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let input = parse_file(filename);
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
