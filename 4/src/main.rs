
use std::fs;
use regex::Regex;

fn collect_input() -> Vec<String> {
    let mut ret = Vec::new();
    let f = fs::read_to_string("input").expect("Unable to open input file");
    for line in f.split("\r\n\r\n") {
        ret.push(line.replace("\r\n", " ").to_string());
    }
    ret
}

fn solve_pt1(inputs: &Vec<String>) -> usize {
    let mut n_valid = 0;
    'outer: for entry in inputs {
        let search_strs = ["byr:","iyr:","eyr:","hgt:","hcl:","ecl:","pid:"];
        for s in &search_strs {
            if !entry.contains(s) {
                continue 'outer;
            }
        }
        n_valid += 1;
    }
    n_valid
}

fn solve_pt2(inputs: &Vec<String>) -> usize {
    fn check_year(s: &str, min: i64, max: i64) -> bool {
        if let Ok(year) = s.parse::<i64>() {
            return year >= min && year <= max;
        }
        false
    }
    fn check_hgt(s: &str) -> bool {
        if let Some(m) = Regex::new("([0-9]+)cm").unwrap().captures(s) {
            return match m.get(1).unwrap().as_str().parse::<i64>().expect("") {
                150 ..= 193 => true,
                _ => false,
            }
        }
        if let Some(m) = Regex::new("([0-9]+)in").unwrap().captures(s) {
            return match m.get(1).unwrap().as_str().parse::<i64>().expect("") {
                59 ..= 76 => true,
                _ => false,
            }
        }
        false
    }
    fn check_ecl(s: &str) -> bool {
        let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        return colors.iter().any(|x| s == *x);
    }

    let search_strs = vec!(
        (Regex::new(r"hgt:([^\s]+)\b").unwrap(), check_hgt as fn(&str)->bool),
        (Regex::new(r"byr:([0-9]{4})\b").unwrap(), |x| check_year(x, 1920, 2002)),
        (Regex::new(r"iyr:([0-9]{4})\b").unwrap(), |x| check_year(x, 2010, 2020)),
        (Regex::new(r"eyr:([0-9]{4})\b").unwrap(), |x| check_year(x, 2020, 2030)),
        (Regex::new(r"hcl:(#[0-9a-f]{6})\b").unwrap(), |_| true),
        (Regex::new(r"ecl:([a-z]{3})\b").unwrap(), check_ecl),
        (Regex::new(r"pid:([0-9]{9})\b").unwrap(), |_| true),
    );

    let mut n_valid = 0;
    'outer: for entry in inputs {
        for (s, t) in &search_strs {
            if let Some(m) = s.captures(entry) {
                if ! t(m.get(1).unwrap().as_str()) {
                    continue 'outer;
                }
            } else {
                continue 'outer;
            }
        }
        n_valid += 1;
    }
    n_valid
}

fn main() {
    let inputs = collect_input();
    println!("Answer part 1: {:?}", solve_pt1(&inputs));
    println!("Answer part 2: {:?}", solve_pt2(&inputs));
}
