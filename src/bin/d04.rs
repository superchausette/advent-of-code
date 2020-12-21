use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::process;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

lazy_static! {
    static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
}

impl Passport {
    fn new(input: &str) -> Self {
        let mut hm: HashMap<String, String> = HashMap::new();
        input
            .split(&['\n', ' '][..])
            .map(|s| s.split(':').collect::<Vec<_>>())
            .filter(|s| s.len() == 2)
            .for_each(|kv| {
                hm.insert(kv[0].to_string(), kv[1].to_string());
            });
        Self {
            byr: hm.get("byr").map(|s| s.clone()),
            iyr: hm.get("iyr").map(|s| s.clone()),
            eyr: hm.get("eyr").map(|s| s.clone()),
            hgt: hm.get("hgt").map(|s| s.clone()),
            hcl: hm.get("hcl").map(|s| s.clone()),
            ecl: hm.get("ecl").map(|s| s.clone()),
            pid: hm.get("pid").map(|s| s.clone()),
            cid: hm.get("cid").map(|s| s.clone()),
        }
    }
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_strict(&self) -> bool {
        if !self.is_valid() {
            return false;
        }
        let byr = self.byr.as_ref().unwrap();
        if byr.len() != 4 || !between(&byr.parse::<u32>().ok(), &Some(1920), &Some(2002)) {
            return false;
        }
        let iyr = self.iyr.as_ref().unwrap();
        if iyr.len() != 4 || !between(&iyr.parse::<u32>().ok(), &Some(2010), &Some(2020)) {
            return false;
        }
        let eyr = self.eyr.as_ref().unwrap();
        if eyr.len() != 4 || !between(&eyr.parse::<u32>().ok(), &Some(2020), &Some(2030)) {
            return false;
        }
        let hgt = self.hgt.as_ref().unwrap();
        if hgt.ends_with("cm") && hgt.len() == 5 {
            if !between(&hgt[0..3].parse::<u32>().unwrap(), &150, &193) {
                return false;
            }
        } else if hgt.ends_with("in") && hgt.len() == 4 {
            if !between(&hgt[0..2].parse::<u32>().unwrap(), &59, &76) {
                return false;
            }
        } else {
            return false;
        }
        if !HCL_REGEX.is_match(self.hcl.as_ref().unwrap().as_str()) {
            return false;
        }
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .contains(&self.ecl.as_ref().unwrap().as_str())
        {
            return false;
        }
        if !PID_REGEX.is_match(self.pid.as_ref().unwrap().as_str()) {
            return false;
        }
        true
    }
}

fn between<T: PartialOrd>(value: &T, min: &T, max: &T) -> bool {
    value >= min && value <= max
}

fn parse_file(filename: &str) -> Vec<Passport> {
    println!("Parsing file {}", filename);
    fs::read_to_string(filename)
        .expect("Could not load file")
        .split("\n\n")
        .map(|x| Passport::new(x))
        .collect()
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
        "Valid nb passport: {}",
        input.iter().filter(|x| x.is_valid()).count()
    );
    println!(
        "Strict Valid nb passport: {}",
        input.iter().filter(|x| x.is_valid_strict()).count()
    );
}
