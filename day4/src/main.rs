use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(lines: &Vec<String>) -> u32 {
    let mut result = 0;

    let re =
        Regex::new(r"Card +(?<cardnum>\d+): +(?<winningnums>\d.*) \| +(?<guesses>\d.*)$").unwrap();
    for line in lines {
        let captures = re.captures(line).unwrap();
        let winning_nums: HashSet<u32> = captures["winningnums"]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let guesses: Vec<u32> = captures["guesses"]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let matching = guesses.iter().filter(|g| winning_nums.contains(&g)).count() as u32;
        if matching > 0 {
            result += u32::pow(2, matching - 1);
        }
    }

    return result;
}

fn part2(lines: &Vec<String>) -> u32 {
    let re =
        Regex::new(r"Card +(?<cardnum>\d+): +(?<winningnums>\d.*) \| +(?<guesses>\d.*)$").unwrap();

    let card_match_counts: Vec<u32> = lines
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let winning_nums: HashSet<u32> = captures["winningnums"]
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            let guesses: Vec<u32> = captures["guesses"]
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            let matching = guesses.iter().filter(|g| winning_nums.contains(&g)).count() as u32;
            matching
        })
        .collect();

    let mut card_counts = vec![1 as u32; card_match_counts.len()];
    for i in 0..card_match_counts.len() {
        for j in 1..=card_match_counts[i] {
            card_counts[i + j as usize] += card_counts[i];
        }
    }

    return card_counts.iter().sum();
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

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
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let result = part1(&lines);
        assert_eq!(result, 13);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let result = part2(&lines);
        assert_eq!(result, 30);
    }
}
