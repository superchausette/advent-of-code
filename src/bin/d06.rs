use std::collections::HashSet;
use std::fs;
use std::process;

fn parse_file(filename: &str) -> Vec<Vec<String>> {
    println!("Parsing file {}", filename);
    fs::read_to_string(filename)
        .expect("Could not load file")
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .filter_map(| s| {if s.is_empty() {return None; } Some(s.to_string())})
                .collect::<Vec<String>>()
        })
        .collect()
}

fn intersection(vhs: &Vec<HashSet<char>>) -> Option<HashSet<char>> {
    if vhs.is_empty() {
        return None;
    }
    Some(vhs.iter().fold(vhs[0].clone(), |acc, hs| {
        hs.intersection(&acc).copied().collect()
    }))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <intput file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let input = parse_file(filename);

    let part1: usize = input
        .iter()
        .map(|vs| vs.join("").chars().collect::<HashSet<char>>().len())
        .sum();
    println!("Part 1: {}", part1);
    let part2: usize = input
        .iter()
        .map(|vs| {
            vs.iter()
                .map(|s| s.chars().collect::<HashSet<char>>())
                .collect::<Vec<HashSet<char>>>()
        })
        .filter_map(|vhs| intersection(&vhs))
        .map(|hm| hm.len())
        .sum();
    println!("Part 2: {}", part2);
}
