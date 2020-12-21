use std::env;
use std::fs;
use std::process;

static EXPECTED_VALUE: i32 = 2020;

fn parse_file(filename: &str) -> Vec<i32> {
    println!("Parsing file {}", filename);
    let file_content = fs::read_to_string(filename).expect("Could not load file");
    file_content
        .split('\n')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn part1(sorted_numbers: &Vec<i32>) {
    'first: for (first_idx, first_value) in sorted_numbers.iter().enumerate() {
        'second: for (second_idx, second_value) in sorted_numbers.iter().enumerate() {
            // prevent self match
            if first_idx == second_idx {
                continue;
            }
            // Do not compute multiple time the same operation
            if first_idx > second_idx {
                continue;
            }
            if first_value + second_value > EXPECTED_VALUE {
                break 'second;
            }
            if first_value + second_value < EXPECTED_VALUE {
                continue;
            }
            println!(
                "[PART1]: Value found: {} and {}, the result is: {}",
                first_value,
                second_value,
                first_value * second_value
            );
            break 'first;
        }
    }
}

fn part2(sorted_numbers: &Vec<i32>) {
    'first: for (first_idx, first_value) in sorted_numbers.iter().enumerate() {
        'second: for (second_idx, second_value) in sorted_numbers.iter().enumerate() {
            'third: for (third_idx, third_value) in sorted_numbers.iter().enumerate() {
                // prevent self match
                if first_idx == second_idx || first_idx == third_idx || second_idx == third_idx {
                    continue;
                }
                // Do not compute multiple time the same operation
                if first_idx > second_idx {
                    continue;
                }
                if first_value + second_value > EXPECTED_VALUE {
                    break 'second;
                }
                if first_value + second_value + third_value > EXPECTED_VALUE {
                    break 'third;
                }
                if first_value + second_value + third_value < EXPECTED_VALUE {
                    continue;
                }
                println!(
                    "[PART2]: Value found: {}, {} and {}, the result is: {}",
                    first_value,
                    second_value,
                    third_value,
                    first_value * second_value * third_value
                );
                break 'first;
            }
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

    part1(&sorted_numbers);
    part2(&sorted_numbers);
}
