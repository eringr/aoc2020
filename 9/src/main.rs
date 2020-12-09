
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::LinkedList;
use itertools::Itertools;

fn collect_inputs() -> Vec::<i64> {
    let mut ret = Vec::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        ret.push(line.unwrap().parse::<i64>().unwrap());
    }
    ret
}


fn solve_pt1(inputs: &Vec::<i64>) -> i64 {
    let mut past = LinkedList::new();
    let it = &mut inputs.iter();
    for n in it.take(25) {
        past.push_back(*n);
    }
    loop {
        if let Some(n) = it.next() {
            let mut found: bool = false;
            for combo in past.iter().combinations(2) {
                if *n == combo.iter().map(|&x| *x).sum::<i64>() {
                    found = true;
                }
            }
            if found {
                past.pop_front();
                past.push_back(*n);
            } else {
                return *n;
            }
        } else {
            panic!("Out of numbers");
        }
    }
}

fn solve_pt2(inputs: &Vec::<i64>, target: i64) -> i64 {
    for i in 0..inputs.len() {
        let mut sum = 0;
        let mut it = inputs.iter().skip(i).peekable();
        let mut smallest = *it.peek().unwrap();
        let mut largest = smallest;
        for j in it {
            sum += j;
            if j > largest {
                largest = j;
            } else if j < smallest {
                smallest = j;
            }
            if sum == target {
                return smallest + largest;
            } else if sum > target {
                continue;
            }
        }
    }
    panic!("Out of numbers");
}

fn main() {
    let inputs = collect_inputs();
    let target = solve_pt1(&inputs);
    println!("Answer part 1: {:?}", target);
    println!("Answer part 2: {:?}", solve_pt2(&inputs, target));
}
