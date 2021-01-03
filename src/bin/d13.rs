use std::fs;
use std::process;

fn parse_file1(filename: &str) -> (usize, Vec<usize>) {
    println!("Parsing file {}", filename);
    let lines: Vec<String> = fs::read_to_string(filename)
        .expect("Could not load file")
        .lines()
        .filter(|s| !s.is_empty())
        .map(str::trim)
        .map(String::from)
        .collect();
    (
        lines[0].parse::<usize>().unwrap(),
        lines[1]
            .split(',')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect(),
    )
}

fn parse_line2(line: &String) -> Vec<(usize, usize)> {
    line.split(',')
        .enumerate()
        .filter_map(|(i, s)| s.parse::<usize>().ok().map(|v| (i, v)))
        .collect()
}

fn parse_file2(filename: &str) -> Vec<(usize, usize)> {
    println!("Parsing file {}", filename);
    let lines: Vec<String> = fs::read_to_string(filename)
        .expect("Could not load file")
        .lines()
        .filter(|s| !s.is_empty())
        .map(str::trim)
        .map(String::from)
        .collect();
    parse_line2(&lines[1])
}

fn part1(input: &(usize, Vec<usize>)) -> usize {
    let (ts, buses) = input;
    let mut next_ts = *ts;
    loop {
        if let Some(id) = buses.iter().find(|&&bus| next_ts % bus == 0) {
            return (next_ts - ts) * id;
        }
        next_ts += 1;
    }
}

fn part2(input: &Vec<(usize, usize)>) -> usize {
    let mut ts = 1;
    let mut next_ts_incr = 1;
    'first_loop: loop {
        for (idx, bus) in input {
            if (ts + idx) % bus != 0 {
                ts += next_ts_incr;
                next_ts_incr = 1;
                continue 'first_loop
            } else {
                next_ts_incr *= bus;
            }
        }
        return ts
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let input1 = parse_file1(filename);
    let input2 = parse_file2(filename);
    println!("Part1: {}", part1(&input1));
    println!("Part2: {}", part2(&input2));
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse_line2(&"17,x,13,19".to_string())), 3417);
    assert_eq!(part2(&parse_line2(&"67,7,59,61".to_string())), 754018);
    assert_eq!(part2(&parse_line2(&"67,x,7,59,61".to_string())), 779210);
    assert_eq!(part2(&parse_line2(&"67,7,x,59,61".to_string())), 1261476);
    assert_eq!(part2(&parse_line2(&"1789,37,47,1889".to_string())), 1202161486);
}
