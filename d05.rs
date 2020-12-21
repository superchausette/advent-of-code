use std::fs;
use std::process;

#[derive(Debug)]
struct Interval {
    min: u16,
    max: u16,
}

impl Interval {

    fn to_upper_half(&mut self) {
        self.min += self.len()/2;
    }

    fn to_lower_half(&mut self) {
        self.max -= self.len()/2
    }

    fn len(&self) -> u16  {
        self.max - self.min + 1
    }

    fn get_single_value(&self) -> Option<u16> {
        return if self.len() == 0 { None } else { Some(self.min) }
    }
}


#[derive(Debug, PartialEq)]
struct Seat {
    row: u16,
    column: u16,
}

impl Seat {
    fn new(input: &str) -> Option<Self> {
        if input.is_empty() {
            return None;
        }
        let mut row_interval =  Interval{min: 0, max: 127};
        let mut column_interval =  Interval{min: 0, max: 7};
        for letter in input.chars() {
            match letter {
                'F' => {row_interval.to_lower_half();},
                'B' => {row_interval.to_upper_half();},
                'L' => {column_interval.to_lower_half();},
                'R' => {column_interval.to_upper_half();},
                _ => {},
            }
        }
        assert!(row_interval.len() == 1);
        assert!(column_interval.len() == 1);
        if let (Some(row), Some(col)) = (row_interval.get_single_value(), column_interval.get_single_value()) {
            return Some(Seat{row: row, column: col});
        }
        None
    }

    fn compute_set_id(&self) -> u16 {
        self.row * 8 + self.column
    }
}

fn parse_file(filename: &str) -> Vec<Seat> {
    println!("Parsing file {}", filename);
    fs::read_to_string(filename)
        .expect("Could not load file")
        .split("\n")
        .filter_map(|s| Seat::new(s.trim()))
        .collect()
}

fn find_missing_seat(seats: &Vec<u16>) -> Option<u16> {
    let mut sorted_seats = seats.clone();
    sorted_seats.sort();
    for (index, value) in sorted_seats.iter().enumerate() {
        if let Some(next_value) = sorted_seats.get(index + 1) {
            if *next_value != (value + 1) {
                return Some(value + 1);
            }
        }
    }
    None
}

mod tests {
    #[test]
    fn test_seat_new() {
        use super::Seat;
        assert_eq!(Seat::new(""), None);
        assert_eq!(Seat::new("BFFFBBFRRR"), Some(Seat{row: 70, column: 7}));
        assert_eq!(Seat::new("FFFBBBFRRR"), Some(Seat{row: 14, column: 7}));
        assert_eq!(Seat::new("BBFFBBFRLL"), Some(Seat{row: 102, column: 4}));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <intput file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let seats = parse_file(filename);
    let seats_number : Vec<u16> = seats.iter().map(|s| s.compute_set_id()).collect();
    let highest_seat_number = seats_number.iter().max().unwrap();
    println!("Number of seats: {}", seats.len());
    println!("Highes seat number: {}", highest_seat_number);
    println!("My seat: {}", find_missing_seat(&seats_number).unwrap());
}
