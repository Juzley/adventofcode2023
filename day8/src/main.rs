
use num;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(lines: &Vec<String>) -> (String, HashMap<String, (String, String)>) {
    let re = Regex::new(r"(?<node>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    let directions: String = lines[0].clone();
    let mut network: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        if let Some(caps) = re.captures(line) {
            network.insert(
                caps["node"].to_string(),
                (caps["left"].to_string(), caps["right"].to_string()),
            );
        }
    }

    return (directions, network);
}

fn part1(lines: &Vec<String>) -> u32 {
    let (directions, network) = parse_input(lines);

    let mut cur_node = "AAA";
    let mut steps = 1;
    for dir in directions.chars().cycle() {
        if dir == 'L' {
            cur_node = &network[cur_node].0;
        } else {
            cur_node = &network[cur_node].1;
        }

        if cur_node == "ZZZ" {
            break;
        }

        steps += 1;
    }

    return steps;
}

fn find_loop_length(
    start: &String,
    directions: &String,
    network: &HashMap<String, (String, String)>,
) -> u32 {
    let mut cur_node = start;
    let mut visited: HashMap<(String, usize), u32> = HashMap::new();
    let mut steps = 1;
    for (i, dir) in directions.chars().enumerate().cycle() {
        if let Some(last_steps) = visited.get(&(cur_node.clone(), i)) {
            return steps - last_steps;
        } else {
            visited.insert((cur_node.to_string(), i), steps);
        }

        if dir == 'L' {
            cur_node = &network[cur_node].0;
        } else {
            cur_node = &network[cur_node].1;
        }
        steps += 1;
    }

    return 0;
}

fn part2(lines: &Vec<String>) -> u64 {
    let (directions, network) = parse_input(lines);
    let loop_lengths: Vec<u64> = network
        .keys()
        .filter(|n| n.chars().nth(2).unwrap() == 'A')
        .map(|s| find_loop_length(s, &directions, &network) as u64)
        .collect();
    return loop_lengths.iter().fold(1, |acc, &l| num::integer::lcm(acc, l));
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
    fn pt1_test_a() {
        let lines = vec![
            String::from("RL"),
            String::from("AAA = (BBB, CCC)"),
            String::from("BBB = (DDD, EEE)"),
            String::from("CCC = (ZZZ, GGG)"),
            String::from("DDD = (DDD, DDD)"),
            String::from("EEE = (EEE, EEE)"),
            String::from("GGG = (GGG, GGG)"),
            String::from("ZZZ = (ZZZ, ZZZ)"),
        ];

        let result = part1(&lines);
        assert_eq!(result, 2);
    }

    #[test]
    fn pt1_test_b() {
        let lines = vec![
            String::from("LLR"),
            String::from("AAA = (BBB, BBB)"),
            String::from("BBB = (AAA, ZZZ)"),
            String::from("ZZZ = (ZZZ, ZZZ)"),
        ];

        let result = part1(&lines);
        assert_eq!(result, 6);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("LR"),
            String::from("11A = (11B, XXX)"),
            String::from("11B = (XXX, 11Z)"),
            String::from("11Z = (11B, XXX)"),
            String::from("22A = (22B, XXX)"),
            String::from("22B = (22C, 22C)"),
            String::from("22C = (22Z, 22Z)"),
            String::from("22Z = (22B, 22B)"),
            String::from("XXX = (XXX, XXX)"),
        ];

        let result = part2(&lines);
        assert_eq!(result, 6);
    }
}
