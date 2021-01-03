use std::fs;
use std::process;

#[derive(Clone, Debug)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Fwd(i32),
}

impl Instruction {
    fn new(s: &str) -> Option<Self> {
        let ins = s.as_bytes()[0] as char;
        let val = s[1..].parse::<i32>().unwrap();
        match ins {
            'N' => Some(Instruction::North(val)),
            'S' => Some(Instruction::South(val)),
            'E' => Some(Instruction::East(val)),
            'W' => Some(Instruction::West(val)),
            'L' => Some(Instruction::Left(val)),
            'R' => Some(Instruction::Right(val)),
            'F' => Some(Instruction::Fwd(val)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Ship {
    position: (i32, i32),
    angle: i32,
}

#[derive(Debug)]
struct WaypointShip {
    ship: Ship,
    waypoint: (i32, i32),
}

impl Ship {
    fn apply(self: &mut Self, instr: &Instruction) {
        match instr {
            Instruction::North(unit) => self.position.1 += unit,
            Instruction::South(unit) => self.position.1 -= unit,
            Instruction::East(unit) => self.position.0 += unit,
            Instruction::West(unit) => self.position.0 -= unit,
            Instruction::Left(angle) => self.angle = (self.angle - angle + 360) % 360,
            Instruction::Right(angle) => self.angle = (self.angle + angle) % 360,
            Instruction::Fwd(unit) => match self.angle {
                0 => {
                    self.apply(&Instruction::North(*unit));
                }
                90 => {
                    self.apply(&Instruction::East(*unit));
                }
                180 => {
                    self.apply(&Instruction::South(*unit));
                }
                270 => {
                    self.apply(&Instruction::West(*unit));
                }
                _ => {
                    panic!("Unexpected ship angle: {}", self.angle)
                }
            },
        }
    }

    fn distance(self: &Self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: (0, 0),
            angle: 90,
        }
    }
}

impl Default for WaypointShip {
    fn default() -> Self {
        Self {
            ship: Ship::default(),
            waypoint: (10, 1),
        }
    }
}

impl WaypointShip {
    fn apply(self: &mut Self, instr: &Instruction) {
        match instr {
            Instruction::North(unit) => self.waypoint.1 += unit,
            Instruction::South(unit) => self.waypoint.1 -= unit,
            Instruction::East(unit) => self.waypoint.0 += unit,
            Instruction::West(unit) => self.waypoint.0 -= unit,
            Instruction::Left(angle) => {
                self.waypoint = match angle {
                    0 => self.waypoint,
                    90 => (-self.waypoint.1, self.waypoint.0),
                    180 => (-self.waypoint.0, -self.waypoint.1),
                    270 => (self.waypoint.1, -self.waypoint.0),
                    _ => panic!(format!("Invalid left angle {}", angle)),
                }
            }
            Instruction::Right(angle) => {
                self.waypoint = match angle {
                    0 => self.waypoint,
                    90 => (self.waypoint.1, -self.waypoint.0),
                    180 => (-self.waypoint.0, -self.waypoint.1),
                    270 => (-self.waypoint.1, self.waypoint.0),
                    _ => panic!(format!("Invalid right angle {}", angle)),
                }
            }
            Instruction::Fwd(unit) => {
                self.ship.position.0 += self.waypoint.0 * unit;
                self.ship.position.1 += self.waypoint.1 * unit;
            }
        }
    }

    fn distance(self: &Self) -> i32 {
        self.ship.distance()
    }
}

fn parse_file(filename: &str) -> Vec<Instruction> {
    println!("Parsing file {}", filename);
    fs::read_to_string(filename)
        .expect("Could not load file")
        .lines()
        .filter_map(Instruction::new)
        .collect()
}

fn part1(input: &Vec<Instruction>) -> i32 {
    let mut ship = Ship::default();
    input.iter().for_each(|instr| ship.apply(instr));
    ship.distance()
}

fn part2(input: &Vec<Instruction>) -> i32 {
    let mut ship = WaypointShip::default();
    input.iter().for_each(|instr| ship.apply(instr));
    ship.distance()
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
