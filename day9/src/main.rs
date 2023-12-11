use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(lines: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for line in lines {
        let mut diffs: Vec<Vec<i32>> = vec![line.clone()];
        loop {
            let mut buf: Vec<i32> = vec![];
            let last_diffs: &Vec<i32> = diffs.last().unwrap();

            for i in 1..last_diffs.len() {
                buf.push(last_diffs[i] - last_diffs[i-1]);
            }

            if buf.iter().all(|&x| x == 0) {
                break;
            } else {
                diffs.push(buf);
            }
        }

        let mut prev_val = 0;
        while !diffs.is_empty() {
            prev_val += diffs.pop().unwrap().last().unwrap();
        }

        result += prev_val;
    }

    return result;
}

fn part2(lines: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for line in lines {
        let mut diffs: Vec<Vec<i32>> = vec![line.clone()];
        loop {
            let mut buf: Vec<i32> = vec![];
            let last_diffs: &Vec<i32> = diffs.last().unwrap();

            for i in 1..last_diffs.len() {
                buf.push(last_diffs[i] - last_diffs[i-1]);
            }

            if buf.iter().all(|&x| x == 0) {
                break;
            } else {
                diffs.push(buf);
            }
        }

        let mut prev_val = 0;
        while !diffs.is_empty() {
            prev_val = diffs.pop().unwrap().first().unwrap() - prev_val;
        }

        result += prev_val;
    }

    return result;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<Vec<i32>> = reader
        .lines()
        .map(|l| l.unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect();

    let pt1_result = part1(&lines);
    let pt2_result = part2(&lines);
    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];

        let result = part1(&lines);
        assert_eq!(result, 114);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];

        let result = part2(&lines);
        assert_eq!(result, 2);
    }
}