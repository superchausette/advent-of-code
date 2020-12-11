use std::env;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::process;

static EXPECTED_VALUE: i32 = 2020;

fn parse_file(filename: &str) -> Vec<i32> {
    println!("Parsing file {}", filename);
    let mut ret : Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                match ip.parse::<i32>() {
                    Ok(number) => ret.push(number),
                    Err(..) =>  {}
                };
            }
        }
    }
    ret
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(sorted_numbers: &Vec<i32>) {
    for (first_idx, first_value) in sorted_numbers.iter().enumerate() {
        for (second_idx, second_value) in sorted_numbers.iter().enumerate() {
            // prevent self match
            if first_idx == second_idx {
                continue;
            }
            // Do not compute multiple time the same operation
            if first_idx > second_idx {
                continue
            }
            if first_value + second_value > EXPECTED_VALUE {
                break
            }
            if first_value + second_value < EXPECTED_VALUE {
                continue
            }
            println!("[PART1]: Value found: {} and {}, the result is: {}", first_value, second_value, first_value*second_value)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <intput file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let mut numbers = parse_file(filename);
    numbers.sort();

    let sorted_numbers = numbers;
    println!("{} Numbers found", sorted_numbers.len());

    part1(&sorted_numbers)
}