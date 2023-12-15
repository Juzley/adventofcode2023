use std::fs::File;
use std::io::{BufRead, BufReader};

fn hash_str(s: &str) -> u64 {
    return s.chars().fold(0, |acc, c| ((acc + c as u64) * 17) % 256);
}

fn part1(line: &str) -> u64 {
    return line.split(",").fold(0, |result, s| result + hash_str(s));
}

fn part2(line: &str) -> u64 {
    let mut boxes: Vec<Vec<(String, u64)>> = vec![vec![]; 256];

    for op in line.split(',') {
        if let Some(split_idx) = op.chars().position(|s| s == '=') {
            let key = op[..split_idx].to_string();
            let value = op[(split_idx + 1)..].parse::<u64>().unwrap();
            let box_idx = hash_str(&key) as usize;

            if let Some(entry) = boxes[box_idx].iter().position(|e| e.0 == key) {
                boxes[box_idx][entry].1 = value;
            } else {
                boxes[box_idx].push((key.to_string(), value));
            }
        } else {
            let key = op.split('-').next().unwrap();
            let box_idx = hash_str(key) as usize;
            boxes[box_idx].retain(|e| e.0 != key);
        }
    }

    let mut result = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, &(_, v)) in b.iter().enumerate() {
            result += (i + 1) * (j + 1) * (v as usize);
        }
    }

    return result as u64;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().map(|l| l.unwrap()).next().unwrap();

    let pt1_result = part1(&line);
    let pt2_result = part2(&line);
    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_a() {
        let line = "HASH";
        assert_eq!(part1(line), 52);
    }

    #[test]
    fn part1_test_b() {
        let line = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(line), 1320);
    }

    #[test]
    fn part2_test() {
        let line = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(line), 145);
    }
}
