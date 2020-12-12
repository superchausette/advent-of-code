#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs;
use std::process;


#[derive(Debug)]
struct PasswordRule {
    min: usize,
    max: usize,
    letter: char,
}

fn parse_file(filename: &str) -> Vec<(PasswordRule, String)>{
    println!("Parsing file {}", filename);
    let file_content = fs::read_to_string(filename).expect("Could not load file");
    file_content
        .split('\n')
        .filter_map(|s | parse_line(s))
        .collect()
}

fn parse_line(line: &str) -> Option<(PasswordRule, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\d+)-(\\d+)\\s+(\\w):\\s+(\\w+)\\s*$").unwrap();
    }
    for cap in RE.captures_iter(line) {
        let min = cap[1].parse::<usize>().unwrap();
        let max = cap[2].parse::<usize>().unwrap();
        let letter = cap[3].chars().nth(0).unwrap();
        let password_rule = PasswordRule{min, max, letter};
        return Some((password_rule, cap[4].to_string()))
    }
    None
}

fn xor(a: bool, b: bool) -> bool {
    ! ( (a && b) || (! a && ! b))
}

fn check_part_1(rule:  &PasswordRule, pass : &String) -> bool {
    let occurrence = pass.matches(rule.letter).count();
    occurrence >= rule.min && occurrence <= rule.max
}

fn check_part_2(rule:  &PasswordRule, pass : &String) -> bool {
    let first_letter = pass.chars().nth(rule.min -1) == Some(rule.letter);
    let second_letter = pass.chars().nth(rule.max -1) == Some(rule.letter);
    xor(first_letter, second_letter)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <intput file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let input = parse_file(filename);
    let mut nb_valid_pass_first :u32 = 0;
    let mut nb_valid_pass_second :u32 = 0;
    for data in input {
        if check_part_1(&data.0, &data.1){
            nb_valid_pass_first += 1;
        }
        if check_part_2(&data.0, &data.1) {
            nb_valid_pass_second += 1;
        }
    }
    println!("Number of valid password: {} for first policy", nb_valid_pass_first);
    println!("Number of valid password: {} for second policy", nb_valid_pass_second);

}