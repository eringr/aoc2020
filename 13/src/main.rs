
use std::fs::File;
use std::io::{BufReader, BufRead};

fn collect_inputs() -> Vec::<String> {
    let mut ret = Vec::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        ret.push(line.expect("").to_string());
    }
    ret
}

fn solve_pt1(inputs: &Vec::<String>) -> usize {
    let departure = inputs[0].parse::<usize>().unwrap();
    let busses: Vec<usize> = inputs[1].split(",")
        .filter(|x| *x != "x")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    println!("{:?}", busses);
    let mut smallest = *busses.iter().max().unwrap();
    let mut ret = 0;
    for bus in busses {
        let wait_time = bus - (departure % bus);
        if wait_time < smallest {
            smallest = wait_time;
            ret = wait_time * bus;
        }
    }
    ret
}

fn solve_pt2(inputs: &Vec::<String>) -> usize {
    let busses: Vec<(usize, usize)> = inputs[1].split(",")
        .enumerate()
        .filter(|&(_, x)| x != "x")
        .map(|(i, x)| (i, x.parse::<usize>().unwrap()))
        .collect();

    let (_, mut lcm) = &busses[0];
    let mut time = 0;
    for (off, id) in busses.iter().skip(1) {
        while (time + off) % id != 0 {
            time += lcm;
        }
        lcm *= id;
    }
    time
}

fn main() {
    let inputs = collect_inputs();
    println!("Answer part 1: {:?}", solve_pt1(&inputs));
    println!("Answer part 2: {:?}", solve_pt2(&inputs));
}
