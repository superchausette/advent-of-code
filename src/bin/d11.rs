use std::fs;
use std::process;

type CountFn = fn(input: &Vec<Vec<Tile>>, row_id: usize, col_id: usize) -> usize;

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl Tile {
    fn new(v: char) -> Option<Self> {
        match v {
            '.' => Some(Tile::Floor),
            'L' => Some(Tile::Empty),
            '#' => Some(Tile::Occupied),
            _ => None,
        }
    }

    fn is_occupied(self: &Self) -> bool {
        *self == Tile::Occupied
    }

    fn switch(self: &mut Self) {
        match self {
            Tile::Floor => {}
            Tile::Empty => *self = Tile::Occupied,
            Tile::Occupied => *self = Tile::Empty,
        }
    }
}

fn parse_file(filename: &str) -> Vec<Vec<Tile>> {
    println!("Parsing file {}", filename);
    fs::read_to_string(filename)
        .expect("Could not load file")
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars().filter_map(Tile::new).collect()
}

fn process(input: &Vec<Vec<Tile>>, count_fn: CountFn, max_occupied: usize) -> Vec<(usize, usize)> {
    let mut to_modify = Vec::new();

    for (row_id, row) in input.iter().enumerate() {
        for (col_id, tile) in row.iter().enumerate() {
            match tile {
                Tile::Floor => {}
                Tile::Empty => {
                    if count_fn(input, row_id, col_id) == 0 {
                        to_modify.push((row_id, col_id));
                    }
                }
                Tile::Occupied => {
                    if count_fn(input, row_id, col_id) >= max_occupied {
                        to_modify.push((row_id, col_id));
                    }
                }
            }
        }
    }

    to_modify
}

fn count_adjacent_occupied(input: &Vec<Vec<Tile>>, row_id: usize, col_id: usize) -> usize {
    let mut ret = 0;
    let incr = |tile: &Tile| -> usize { tile.is_occupied() as usize };

    let is_first_row = row_id == 0;
    let is_last_row = row_id == (input.len() - 1);
    let is_first_col = col_id == 0;
    let is_last_col = col_id == (input[row_id].len() - 1);

    // Check ROW - 1
    if !is_first_row {
        if !is_first_col {
            ret += incr(&input[row_id - 1][col_id - 1]);
        }
        ret += incr(&input[row_id - 1][col_id]);
        if !is_last_col {
            ret += incr(&input[row_id - 1][col_id + 1]);
        }
    }

    // Check ROW
    if !is_first_col {
        ret += incr(&input[row_id][col_id - 1]);
    }
    if !is_last_col {
        ret += incr(&input[row_id][col_id + 1]);
    }

    // Check ROW +1
    if !is_last_row {
        if !is_first_col {
            ret += incr(&input[row_id + 1][col_id - 1]);
        }
        ret += incr(&input[row_id + 1][col_id]);
        if !is_last_col {
            ret += incr(&input[row_id + 1][col_id + 1]);
        }
    }

    ret
}

fn count_adjacent_visible(input: &Vec<Vec<Tile>>, row_id: usize, col_id: usize) -> usize {
    let mut ret = 0;
    ret += is_occupied_in_direction(&input, row_id, col_id, -1, -1);
    ret += is_occupied_in_direction(&input, row_id, col_id, -1, 0);
    ret += is_occupied_in_direction(&input, row_id, col_id, -1, 1);
    ret += is_occupied_in_direction(&input, row_id, col_id, 0, -1);
    ret += is_occupied_in_direction(&input, row_id, col_id, 0, 1);
    ret += is_occupied_in_direction(&input, row_id, col_id, 1, -1);
    ret += is_occupied_in_direction(&input, row_id, col_id, 1, 0);
    ret += is_occupied_in_direction(&input, row_id, col_id, 1, 1);
    ret
}

fn is_occupied_in_direction(
    input: &Vec<Vec<Tile>>,
    row_id: usize,
    col_id: usize,
    delta_x: i32,
    delta_y: i32,
) -> usize {
    let (mut cur_row_id, mut cur_col_id) = (row_id, col_id);
    loop {
        if cur_row_id as i32 + delta_y < 0 {
            return 0;
        }
        cur_row_id = (cur_row_id as i32 + delta_y) as usize;
        if cur_col_id as i32 + delta_x < 0 {
            return 0;
        }
        cur_col_id = (cur_col_id as i32 + delta_x) as usize;
        match input.get(cur_row_id) {
            None => {
                return 0;
            }
            _ => match input[cur_row_id].get(cur_col_id) {
                None => {
                    return 0;
                }
                Some(Tile::Floor) => {}
                Some(Tile::Empty) => {
                    return 0;
                    }
                Some(Tile::Occupied) => {
                    return 1;
                }
            },
        }
    }
}

fn modify(input: &Vec<Vec<Tile>>, to_modify: &Vec<(usize, usize)>) -> Vec<Vec<Tile>> {
    let mut ret = input.clone();
    for tile in to_modify {
        ret[tile.0][tile.1].switch();
    }
    ret.to_vec()
}

fn part1(input: &Vec<Vec<Tile>>) -> usize {
    let mut modified: Vec<Vec<Tile>> = input.clone();
    let mut to_modify = process(&modified, count_adjacent_occupied, 4);
    while !to_modify.is_empty() {
        modified = modify(&modified, &to_modify);
        to_modify = process(&modified, count_adjacent_occupied, 4);
    }
    modified
        .iter()
        .map(|vt| vt.iter().filter(|t| t.is_occupied()).count())
        .sum()
}

fn part2(input: &Vec<Vec<Tile>>) -> usize {
    let mut modified: Vec<Vec<Tile>> = input.clone();
    let mut to_modify = process(&modified, count_adjacent_visible, 5);
    while !to_modify.is_empty() {
        modified = modify(&modified, &to_modify);
        to_modify = process(&modified, count_adjacent_visible, 5);
    }
    modified
        .iter()
        .map(|vt| vt.iter().filter(|t| t.is_occupied()).count())
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let input = parse_file(filename);
    assert!(input.iter().all(|v| v.len() == input[0].len()));
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
