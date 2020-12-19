
use std::fs;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::cmp;

type RangeVec = Vec::<((usize, usize), (usize, usize))>;

fn collect_inputs() -> Vec::<String> {
    let f = fs::File::open("input").expect("Unable to open input file");
    return BufReader::new(f).lines().map(|x| x.unwrap().to_string()).collect();
}

fn solve_ranges(columns: &Vec::<Vec::<usize>>, ranges: &RangeVec) -> usize {
    let num_columns = columns.len();
    let mut range_possibilities = vec![Vec::<usize>::new(); num_columns];

    for (i, ((min1, max1), (min2, max2))) in ranges.iter().enumerate() {
        'outer: for (j, column) in columns.iter().enumerate() {
            for val in column {
                if (val < min1) || (val > max1 && val < min2) || (val > max2) {
                    continue 'outer;
                }
            }
            range_possibilities[i].push(j);
        }
    }

    let mut solved_ranges = vec![num_columns+1; 20];
    for i in 1..=num_columns {
        for (j, range) in range_possibilities.iter().enumerate() {
            if range.len() == i {
                for col in range {
                    if !solved_ranges.contains(col) {
                        solved_ranges[j] = *col;
                    }
                }
            }
        }
    }

    return solved_ranges.iter().take(6).map(|&x| columns[x][0]).product();
}

fn solve_both(inputs: &Vec::<String>) -> (usize, usize) {
    let range_matcher =
        Regex::new(r": ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
    let mut ranges = RangeVec::new();
    let mut smallest = 1000;
    let mut largest = 0;
    for line in inputs {
        if let Some(m) = range_matcher.captures(line) {
            let (min1, max1, min2, max2) = (
                m.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                m.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                m.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                m.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            );
            smallest = cmp::min(smallest, min1);
            largest = cmp::max(largest, max2);
            ranges.push(((min1, max1), (min2, max2)));
        }
    }

    let ticket_matcher = Regex::new(r"^[0-9]").unwrap();
    let mut invalid_numbers = Vec::<usize>::new();
    let mut columns = vec![Vec::<usize>::new(); ranges.len()];
    for line in inputs {
        if ticket_matcher.captures(&line).is_none() {
            continue;
        }
        let vals: Vec<usize> = line.split(",")
            .map(|x| x.parse::<usize>().unwrap()).collect();
        let invalid: Vec<usize> = vals.iter()
            .filter(|&x| *x < smallest || *x > largest).map(|x| *x).collect();
        if invalid.len() > 0 {
            invalid_numbers.extend(invalid);
            continue;
        }
        let _: () = columns.iter_mut()
            .zip(vals.iter()).map(|(x, y)| x.push(*y)).collect();
    }

    (invalid_numbers.iter().sum(), solve_ranges(&columns, &ranges))
}

fn main() {
    let inputs = collect_inputs();
    let (answer_pt1, answer_pt2) = solve_both(&inputs);
    println!("Answer part 1: {:?}", answer_pt1);
    println!("Answer part 2: {:?}", answer_pt2);
}
