use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const RED_COUNT: u32 = 12;
const GREEN_COUNT: u32 = 13;
const BLUE_COUNT: u32 = 14;

fn get_count(regex: &Regex, line: &str) -> u32 {
    let caps = regex.captures(line);
    if caps.is_none() {
        return 0;
    }

    return caps.unwrap()["count"].parse::<u32>().unwrap_or(0);
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut pt1_result = 0;
    let mut pt2_result = 0;
    let red_regex = Regex::new(r"(?<count>\d+) red").unwrap();
    let green_regex = Regex::new(r"(?<count>\d+) green").unwrap();
    let blue_regex = Regex::new(r"(?<count>\d+) blue").unwrap();

    for (i, line) in reader.lines().enumerate() {
        let mut possible = true;
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for draw in line.unwrap().split(";") {
            let red_count = get_count(&red_regex, draw);
            let green_count = get_count(&green_regex, draw);
            let blue_count = get_count(&blue_regex, draw);

            min_red = std::cmp::max(min_red, red_count);
            min_green = std::cmp::max(min_green, green_count);
            min_blue = std::cmp::max(min_blue, blue_count);

            if red_count > RED_COUNT || green_count > GREEN_COUNT || blue_count > BLUE_COUNT {
                possible = false;
            }
        }

        if possible {
            pt1_result += i + 1;
        }

        pt2_result += min_red * min_green * min_blue;
    }

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}
