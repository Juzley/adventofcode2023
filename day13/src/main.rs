use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut patterns = vec![];

    let mut cur_pattern = vec![];
    for line in lines {
        if line.is_empty() {
            if !cur_pattern.is_empty() {
                patterns.push(cur_pattern);
            }
            cur_pattern = vec![];
        } else {
            cur_pattern.push(line.clone());
        }
    }
    patterns.push(cur_pattern);

    return patterns;
}

fn find_reflections(pattern: &Vec<String>) -> usize {
    'outer: for axis in 0..(pattern.len() - 1) {
        let run_len = std::cmp::min(axis + 1, pattern.len() - axis - 1);

        for i in 0..run_len {
            let fwd = axis - i;
            let rev = axis + 1 + i;

            if pattern[fwd] != pattern[rev] {
                continue 'outer;
            }
        }
        return axis + 1;
    }

    return 0;
}

fn find_corrected_reflections(pattern: &Vec<String>) -> usize {
    'outer: for axis in 0..(pattern.len() - 1) {
        let run_len = std::cmp::min(axis + 1, pattern.len() - axis - 1);
        let mut found_edit = false;

        for i in 0..run_len {
            let fwd = axis - i;
            let rev = axis + 1 + i;

            let edit_distance = pattern[fwd]
                .chars()
                .zip(pattern[rev].chars())
                .filter(|(ch_a, ch_b)| ch_a != ch_b)
                .count();
            if edit_distance > 1 {
                // Strings too dissimilar.
                continue 'outer;
            } else if edit_distance == 1 {
                if found_edit {
                    // This is the second edit... no good.
                    continue 'outer;
                } else {
                    found_edit = true;
                }
            }
        }

        if found_edit {
            return axis + 1;
        }
    }

    return 0;
}

fn transpose_pattern(pattern: &Vec<String>) -> Vec<String> {
    // Transpose the pattern so that rows are columns and columns are rows
    let mut new_pattern = vec![];
    for i in 0..pattern[0].len() {
        let mut row = String::new();
        for j in 0..pattern.len() {
            row.push(pattern[j].chars().nth(i).unwrap());
        }
        new_pattern.push(row);
    }

    return new_pattern;
}

fn part1(patterns: &Vec<Vec<String>>) -> u32 {
    let mut result = 0;
    for pattern in patterns {
        let horizontal_reflection = find_reflections(pattern);
        let transposed = transpose_pattern(pattern);
        let vertical_reflection = find_reflections(&transposed);

        result += vertical_reflection as u32 + horizontal_reflection as u32 * 100;
    }

    return result;
}

fn part2(patterns: &Vec<Vec<String>>) -> u32 {
    let mut result = 0;
    for pattern in patterns {
        let horizontal_reflection = find_corrected_reflections(pattern);
        let transposed = transpose_pattern(pattern);
        let vertical_reflection = find_corrected_reflections(&transposed);

        result += vertical_reflection as u32 + horizontal_reflection as u32 * 100;
    }

    return result;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let patterns = parse_input(&lines);

    let pt1_result = part1(&patterns);
    let pt2_result = part2(&patterns);
    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let lines = vec![
            String::from("#.##..##."),
            String::from("..#.##.#."),
            String::from("##......#"),
            String::from("##......#"),
            String::from("..#.##.#."),
            String::from("..##..##."),
            String::from("#.#.##.#."),
            String::from(""),
            String::from("#...##..#"),
            String::from("#....#..#"),
            String::from("..##..###"),
            String::from("#####.##."),
            String::from("#####.##."),
            String::from("..##..###"),
            String::from("#....#..#"),
        ];

        let patterns = parse_input(&lines);
        assert_eq!(part1(&patterns), 405);
    }

    #[test]
    fn part2_test() {
        let lines = vec![
            String::from("#.##..##."),
            String::from("..#.##.#."),
            String::from("##......#"),
            String::from("##......#"),
            String::from("..#.##.#."),
            String::from("..##..##."),
            String::from("#.#.##.#."),
            String::from(""),
            String::from("#...##..#"),
            String::from("#....#..#"),
            String::from("..##..###"),
            String::from("#####.##."),
            String::from("#####.##."),
            String::from("..##..###"),
            String::from("#....#..#"),
        ];

        let patterns = parse_input(&lines);
        assert_eq!(part2(&patterns), 400);
    }
}
