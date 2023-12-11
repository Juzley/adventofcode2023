use std::cmp::{min, max};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_distances(lines: &Vec<String>, expansion_factor: u64) -> u64 {
    let mut galaxies = vec![];
    for (row, line) in lines.iter().enumerate() {
        for (col, line) in line.chars().enumerate() {
            if line == '#' {
                galaxies.push((row, col));
            }
        }
    }

    let populated_rows: HashSet<_> = galaxies.iter().map(|(r, _)| r).collect();
    let populated_cols: HashSet<_> = galaxies.iter().map(|(_, c)| c).collect();

    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let a = galaxies[i];
            let b = galaxies[j];
            let mut distance = 0;

            let start_row = min(a.0, b.0);
            let end_row = max(a.0, b.0);
            for r in start_row..end_row {
                if populated_rows.contains(&r) {
                    distance += 1;
                } else {
                    distance += expansion_factor;
                }
            }

            let start_col = min(a.1, b.1);
            let end_col = max(a.1, b.1);
            for c in start_col..end_col {
                if populated_cols.contains(&c) {
                    distance += 1;
                } else {
                    distance += expansion_factor;
                }
            }

            total_distance += distance;
        }
    }

    return total_distance;
}

fn part1(lines: &Vec<String>) -> u64 {
    return find_distances(lines, 2);
}

fn part2(lines: &Vec<String>) -> u64 {
    return find_distances(lines, 1000000);
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
            String::from("...#......"),
            String::from(".......#.."),
            String::from("#........."),
            String::from(".........."),
            String::from("......#..."),
            String::from(".#........"),
            String::from(".........#"),
            String::from(".........."),
            String::from(".......#.."),
            String::from("#...#....."),
        ];

        let result = part1(&lines);
        assert_eq!(result, 374);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("...#......"),
            String::from(".......#.."),
            String::from("#........."),
            String::from(".........."),
            String::from("......#..."),
            String::from(".#........"),
            String::from(".........#"),
            String::from(".........."),
            String::from(".......#.."),
            String::from("#...#....."),
        ];

        let result = find_distances(&lines, 10);
        assert_eq!(result, 1030);

        let result = find_distances(&lines, 100);
        assert_eq!(result, 8410);
    }
}
