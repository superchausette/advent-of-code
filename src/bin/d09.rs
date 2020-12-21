use std::collections::HashSet;
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

fn generate_all_sums(preamble: &[u64]) -> HashSet<u64> {
    assert_eq!(preamble.len(), 25);
    let mut ret = HashSet::new();
    for (idx1, value1) in preamble.iter().enumerate() {
        for (idx2, value2) in preamble.iter().enumerate() {
            if idx1 != idx2 {
                ret.insert(value1 + value2);
            }
        }
    }
    ret
}

fn part1(input: &Vec<u64>) -> Option<u64> {
    for (idx, value) in input[25..].iter().enumerate() {
        if !generate_all_sums(&input[idx..idx + 25]).contains(value) {
            return Some(*value);
        }
    }
    None
}

fn part2(input: &Vec<u64>, part1: u64) -> Option<u64> {
    for set_size in 2..input.len() - 2 {
        for idx in 0..input.len() - set_size {
            let slice = &input[idx..idx + set_size];
            if slice.iter().sum::<u64>() == part1 {
                return Some(slice.iter().min().unwrap() + *slice.iter().max().unwrap());
            }
        }
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let input = parse_file(filename);
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input, part1(&input).unwrap()).unwrap());
}
