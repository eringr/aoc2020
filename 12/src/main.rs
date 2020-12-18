
use std::fs::File;
use std::io::{BufReader, BufRead};
use nalgebra;

fn collect_inputs() -> Vec::<String> {
    let mut ret = Vec::new();
    let f = File::open("input").expect("Unable to open input file");
    for line in BufReader::new(f).lines() {
        ret.push(line.expect("").to_string());
    }
    ret
}

fn solve_pt1(inputs: &Vec::<String>) -> i64 {
    let mut pos = nalgebra::Matrix2x1::new(0i64, 0i64);
    let mut heading = nalgebra::Matrix2x1::new(1i64, 0i64);
    for line in inputs {
        let val: i64 = line.as_str()[1..].parse().unwrap();
        match line.as_bytes()[0] as char {
            'N' => pos += val * nalgebra::Matrix2x1::new( 0i64,  1i64),
            'E' => pos += val * nalgebra::Matrix2x1::new( 1i64,  0i64),
            'S' => pos += val * nalgebra::Matrix2x1::new( 0i64, -1i64),
            'W' => pos += val * nalgebra::Matrix2x1::new(-1i64,  0i64),
            'F' => pos += val * heading,
            'R' => match val {
                90 => heading = nalgebra::Matrix2::new(0, 1, -1, 0) * heading,
                180 => heading = nalgebra::Matrix2::new(-1, 0, 0, -1) * heading,
                270 => heading = nalgebra::Matrix2::new(0, -1, 1, 0) * heading,
                _ => panic!(""),
            }
            'L' => match val {
                90 => heading = nalgebra::Matrix2::new(0, -1, 1, 0) * heading,
                180 => heading = nalgebra::Matrix2::new(-1, 0, 0, -1) * heading,
                270 => heading = nalgebra::Matrix2::new(0, 1, -1, 0) * heading,
                _ => panic!(""),
            }
            _ => panic!(""),
        }
    }
    return pos[(0,0)].abs() + pos[(1,0)].abs();
}

fn solve_pt2(inputs: &Vec::<String>) -> i64 {
    let mut pos = nalgebra::Matrix2x1::new(0i64, 0i64);
    let mut wp = nalgebra::Matrix2x1::new(10i64, 1i64);
    for line in inputs {
        let val: i64 = line.as_str()[1..].parse().unwrap();
        match line.as_bytes()[0] as char {
            'N' => wp += val * nalgebra::Matrix2x1::new( 0i64,  1i64),
            'E' => wp += val * nalgebra::Matrix2x1::new( 1i64,  0i64),
            'S' => wp += val * nalgebra::Matrix2x1::new( 0i64, -1i64),
            'W' => wp += val * nalgebra::Matrix2x1::new(-1i64,  0i64),
            'F' => pos += val * wp,
            'R' => match val {
                90 => wp = nalgebra::Matrix2::new(0, 1, -1, 0) * wp,
                180 => wp = nalgebra::Matrix2::new(-1, 0, 0, -1) * wp,
                270 => wp = nalgebra::Matrix2::new(0, -1, 1, 0) * wp,
                _ => panic!(""),
            }
            'L' => match val {
                90 => wp = nalgebra::Matrix2::new(0, -1, 1, 0) * wp,
                180 => wp = nalgebra::Matrix2::new(-1, 0, 0, -1) * wp,
                270 => wp = nalgebra::Matrix2::new(0, 1, -1, 0) * wp,
                _ => panic!(""),
            }
            _ => panic!(""),
        }
    }
    return pos[(0,0)].abs() + pos[(1,0)].abs();
}

fn main() {
    let inputs = collect_inputs();
    println!("Answer part 1: {:?}", solve_pt1(&inputs));
    println!("Answer part 2: {:?}", solve_pt2(&inputs));
}
