use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::process;


#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
struct ProcessorState {
    addr : usize,
    acc : i32,
}

impl Instruction {
    fn new(input: &str) -> Option<Self> {
        let splitted : Vec<_> = input.split(" ").map(str::trim).collect();
        if splitted.len() != 2 {
            return None;
        }
        let arg = splitted[1].parse().unwrap();
        match splitted[0] {
            "nop" => Some(Instruction::Nop(arg)),
            "acc" => Some(Instruction::Acc(arg)),
            "jmp" => Some(Instruction::Jmp(arg)),
            _ => None
        }
    }
}

fn parse_file(filename: &str) -> Vec<Instruction> {
    println!("Parsing file {}", filename);
    fs::read_to_string(filename)
        .expect("Could not load file")
        .lines()
        .filter_map(|s| Instruction::new(s))
        .collect()
}

fn add(addr : usize, offset: i32) -> usize {
    if offset >= 0 {
        return addr + usize::try_from(offset).unwrap();
    } else  {
        return addr - usize::try_from(-offset).unwrap()
    }
}

fn swap_jmp_nop(instruction : &mut Instruction) {
    match instruction {
        Instruction::Nop(offset) => {*instruction = Instruction::Jmp(*offset)},
        Instruction::Jmp(offset) => {*instruction = Instruction::Nop(*offset)},
        Instruction::Acc(_) => {},
    }
}


fn generate_modified_listing(listing : &Vec<Instruction>, idx : usize) -> Vec<Instruction> {
    let mut ret =  listing.clone();
    swap_jmp_nop(ret.get_mut(idx).unwrap());
    ret
}

fn process(instr: &Instruction, state: &ProcessorState) -> ProcessorState {
    let (next_addr, next_acc) = match instr {
        Instruction::Nop(_) => {(state.addr + 1, state.acc)},
        Instruction::Acc(acc) => {(state.addr + 1, state.acc +acc )},
        Instruction::Jmp(offset) => {(add(state.addr, *offset), state.acc)},
    };
    ProcessorState{addr: next_addr, acc: next_acc }
}

fn run(listing : &Vec<Instruction>) -> Option<i32> {
    let mut visited : HashSet<usize> = HashSet::new();

    let mut state = ProcessorState{addr: 0, acc:0 };
    while let Some(instr) = listing.get(state.addr) {
        if visited.contains(&state.addr) {
            return None;
        }
        visited.insert(state.addr);
        state = process(instr, &state);
    }
    Some(state.acc)
}


fn part1(listing : &Vec<Instruction>) -> i32 {
    let mut visited : HashSet<usize> = HashSet::new();

    let mut state = ProcessorState{addr: 0, acc:0 };
    while !visited.contains(&state.addr) {
        visited.insert(state.addr);
        let next_state = process(&listing[state.addr], &state);
        state = next_state
    }
    state.acc
}

fn part2(listing : &Vec<Instruction>) -> i32 {
    let mut modified_listing = listing.clone();
    for (idx, _) in modified_listing.iter_mut().enumerate() {
        let modified_listing = generate_modified_listing(listing, idx);
        if let Some(acc) = run(&modified_listing) {
            return acc;
        }
    }
    0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let listing = parse_file(filename);
    println!("Part1: {}", part1(&listing));
    println!("Part2: {}", part2(&listing));
}
