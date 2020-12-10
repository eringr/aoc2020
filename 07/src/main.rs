
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn collect_input() -> HashMap::<String, HashMap<String, usize>> {
    let mut inputs = Vec::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        inputs.push(line.expect(""));
    }

    let matcher = Regex::new(
        "^([a-z ]+) bags contain\
        (?: ([0-9]+) ([a-z ]+) bags?[.,])?\
        (?: ([0-9]+) ([a-z ]+) bags?[.,])?\
        (?: ([0-9]+) ([a-z ]+) bags?[.,])?\
        (?: ([0-9]+) ([a-z ]+) bags?[.,])?").unwrap();

    let mut m = HashMap::<String, HashMap<String, usize>>::new();
    for input in inputs {
        let caps = matcher.captures(input.as_str()).unwrap();
        let caps_it = &mut caps.iter().flatten().map(|x| x.as_str());
        let container = caps_it.skip(1).next().unwrap();
        for contained_desc in caps_it.collect::<Vec<&str>>().chunks(2) {
            if let [n, desc] = &contained_desc[..] {
                m.entry(container.to_string()).or_insert(HashMap::new())
                    .insert(desc.to_string(), n.parse::<usize>().unwrap());
            }
        }
    }
    m
}

fn solve_pt1(inputs: &HashMap::<String, HashMap<String, usize>>) -> usize {
    let mut possible_containers = HashSet::<String>::new();
    let mut to_search = vec!["shiny gold".to_string()];
    return loop {
        if let Some(checker) = to_search.pop() {
            for (container, contents) in inputs {
                if contents.contains_key(&checker) {
                    if !possible_containers.contains(container) {
                        to_search.push(container.to_string());
                        possible_containers.insert(container.to_string());
                    }
                }
            }
        } else {
            break possible_containers.len();
        }
    }
}

fn solve_pt2(inputs: &HashMap::<String, HashMap<String, usize>>) -> usize {
    fn get_subcount(
        m: &HashMap::<String, HashMap<String, usize>>,
        desc: &String
    ) -> usize {
        let mut subcount = 1;
        if let Some(subs) = m.get(desc) {
            for (subdesc, count) in subs {
                // println!("{} holds {} {}", desc, count, subdesc);
                subcount += count * get_subcount(m, &subdesc);
                // println!("{} {} subcount {}", desc, subdesc, count);
            }
        }
        subcount
    };
    return get_subcount(inputs, &"shiny gold".to_string()) - 1;
}

fn main() {
    let inputs = collect_input();
    println!("Answer part 1: {:?}", solve_pt1(&inputs));
    println!("Answer part 2: {:?}", solve_pt2(&inputs));
}
