use std::fs::File;
use std::io::{BufRead, BufReader};

fn helper(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        let mut first: Option<u32> = None;
        let mut last: u32 = 0;
        for c in line.unwrap().chars() {
            if c.is_digit(10) {
                let num: u32 = c.to_digit(10).unwrap();
                if first.is_none() {
                    first = Some(num);
                }
                last = num;
            }
        }

        result += first.unwrap() * 10 + last;
    }

    return result;
}

fn main() {
    let pt1 = helper("input");

    // "input2" has "one", "two" etc replaced with the digits.
    // Some care required for things like "twoone" = 21, "eighthree" = 83
    let pt2 = helper("input_pt2");

    println!("Part 1: {}, Part 2: {}", pt1, pt2);
}
