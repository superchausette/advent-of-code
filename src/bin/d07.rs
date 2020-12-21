use std::collections::{HashMap, HashSet};
use std::fs;
use std::process;

#[derive(Debug, PartialEq)]
struct BagRule {
    name: String,
    contained: HashMap<String, u32>,
}

fn parse_file(filename: &str) -> Vec<BagRule> {
    println!("Parsing file {}", filename);
    fs::read_to_string(filename)
        .expect("Could not load file")
        .lines()
        .filter_map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Option<BagRule> {
    let sanitized_line = line.replace("bags", "bag");
    let splited = sanitized_line.split("contain").map(str::trim).collect::<Vec<&str>>();
    if splited.len() != 2 {
        return None;
    }
    let name = splited[0].to_string();
    let contained_bags: Vec<(String, u32)> = splited[1]
        .split(",")
        .map(str::trim)
        .map(|s| s.replace(".", ""))
        .map(|s| s.splitn(2, " ").map(str::to_string).collect::<Vec<String>>())
        .filter_map(|vs| {
            if vs.len() != 2 { return None; }
            if vs[0] == "no" { return None; }
            Some((vs[1].clone(), vs[0].parse().unwrap()))

        })
        .collect();
    Some(BagRule {
        name: name,
        contained: contained_bags.into_iter()
        .collect(),
    })
}



fn find_all_containing_bags(map: &HashMap<String, HashMap<String, u32>>, name :&str) -> HashSet<String> {
    let sanitized_name = name.replace("bags", "bag");
    let mut ret = HashSet::new();
    for (bag, contained) in map {
        if contained.contains_key(&sanitized_name) {
            ret.insert(bag.clone());
            ret.extend(find_all_containing_bags(map, bag));
        }
    }
    ret
}

fn find_nb_of_bags_contained(map: &HashMap<String, HashMap<String, u32>>, name :&str) -> u64 {
    map[&name.replace("bags", "bag")].iter().map(| (bag, contained) | u64::from(*contained) * (1 + find_nb_of_bags_contained(&map, &bag))).sum()
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <intput file>", args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let input = parse_file(filename);

    let map : HashMap<String, HashMap<String, u32>> = input.into_iter().map(| br | (br.name, br.contained)).collect();
    let target = "shiny gold bags";
    let part1 = find_all_containing_bags(&map, target).len();
    println!("Part1: Found {} bags", part1);
    let part2 = find_nb_of_bags_contained(&map, target);
    println!("Part2: Number of bag required: {}", part2);
}

mod tests {
    #[test]
    fn test_parse_line_empty() {
        use super::parse_line;
        assert_eq!(parse_line(""), None);
    }
    #[test]
    fn test_parse_line_0_bag() {
        use super::{parse_line, BagRule};
        use std::collections::HashMap;
        assert_eq!(
            parse_line("faded blue bags contain no other bags."),
            Some(BagRule {
                name: "faded blue bag".to_string(),
                contained: HashMap::new(),
            })
        );
    }
    #[test]
    fn test_parse_line_1_bag() {
        use super::{parse_line, BagRule};
        assert_eq!(
            parse_line("bright white bags contain 1 shiny gold bag."),
            Some(BagRule {
                name: "bright white bag".to_string(),
                contained: vec![("shiny gold bag".to_string(), 1)]
                    .into_iter()
                    .collect(),
            })
        );
    }
    #[test]
    fn test_parse_line_2_bag() {
        use super::{parse_line, BagRule};
        assert_eq!(
            parse_line("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Some(BagRule {
                name: "light red bag".to_string(),
                contained: vec![
                    ("bright white bag".to_string(), 1),
                    ("muted yellow bag".to_string(), 2)
                ]
                .into_iter()
                .collect(),
            })
        );
    }
}
