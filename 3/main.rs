use std::fs;
use std::process;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TileType {
    Open,
    Tree,
}

impl TileType {
    fn new(v: char) -> Option<TileType> {
        match v {
            '.' => Some(TileType::Open),
            '#' => Some(TileType::Tree),
            _ => None,
        }
    }
}

fn parse_file(filename: &str) -> Vec<Vec<TileType>> {
    println!("Parsing file {}", filename);
    let file_content = fs::read_to_string(filename).expect("Could not load file");
    file_content
        .split('\n')
        .filter_map(|s| parse_line(s))
        .collect()
}

fn parse_line(line: &str) -> Option<Vec<TileType>> {
    if line.is_empty() {
        return None;
    }
    return Some(line.chars().filter_map(|c| TileType::new(c)).collect());
}

fn get_tile_type(line: &Vec<TileType>, idx: usize) -> TileType {
    assert!(!line.is_empty());
    line[idx % line.len()]
}

fn tree_encountered(slope: &Vec<Vec<TileType>>, down_shift: usize, right_shift: usize) -> usize {
    slope
        .iter()
        .enumerate()
        .filter_map(|x| match x.0 % down_shift {
            0 => Some(x.1),
            _ => None,
        })
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
        .filter(|x| {
            get_tile_type(x.1, x.0 * right_shift) == TileType::Tree
        })
        .count()
}

fn print_tree_encountered(slope: &Vec<Vec<TileType>>, down_shift: usize, right_shift: usize) {
    let encountered = tree_encountered(slope, down_shift, right_shift);
    println!(
        "For down: {} and right: {} => {} / {}",
        down_shift,
        right_shift,
        encountered,
        slope.len()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <intput file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let input = parse_file(filename);
    println!(
        "Part 1: Number of tree encountered {} out of {}",
        tree_encountered(&input, 1, 3),
        input.len()
    );
    print_tree_encountered(&input, 1, 1);
    print_tree_encountered(&input, 1, 3);
    print_tree_encountered(&input, 1, 5);
    print_tree_encountered(&input, 1, 7);
    print_tree_encountered(&input, 2, 1);
    println!(
        "Part 2: {}",
        (tree_encountered(&input, 1, 1)
            * tree_encountered(&input, 1, 3)
            * tree_encountered(&input, 1, 5)
            * tree_encountered(&input, 1, 7)
            * tree_encountered(&input, 2, 1))
    );
}
