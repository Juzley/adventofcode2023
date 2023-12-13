use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_arrangements_helper(
    cur_spring: char,
    remaining_springs: &[char],
    mut runs: &[u8],
    mut cur_run_count: u8,
) -> u64 {
    if cur_spring == '.' {
        if cur_run_count > 0 {
            // Terminate the current run
            if cur_run_count != runs[0] {
                return 0;
            }
            cur_run_count = 0;
            runs = &runs[1..];
        }
    } else if cur_spring == '#' {
        cur_run_count += 1;
        if runs.is_empty() {
            // We already consumed all the springs, this can't be a solution.
            return 0;
        } else if cur_run_count > runs[0] {
            // The run is too big, this can't be a solution
            return 0;
        }
    }

    if remaining_springs.is_empty() {
        // Reached the end, check if this was a valid solution.
        if cur_run_count > 0 {
            // Currently in a run, check if this matches expectations.
            if runs.len() == 1 && cur_run_count == runs[0] {
                return 1;
            } else {
                return 0;
            }
        } else {
            // Not in a run, check we aren't expecting to find runs still.
            if runs.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }
    } else {
        let next_spring = remaining_springs[0];

        if next_spring == '?' {
            // Don't know what the next thing is, try both empty and spring.
            let with_empty =
                find_arrangements_helper('.', &remaining_springs[1..], runs, cur_run_count);
            let with_spring =
                find_arrangements_helper('#', &remaining_springs[1..], runs, cur_run_count);
            return with_empty + with_spring;
        } else {
            return find_arrangements_helper(
                next_spring,
                &remaining_springs[1..],
                runs,
                cur_run_count,
            );
        }
    }
}

fn find_arrangements(line: &(Vec<char>, Vec<u8>)) -> u64 {
    let springs = &line.0;
    let runs = &line.1;
    let next_spring = springs[0];

    let result;
    if next_spring == '?' {
        result = find_arrangements_helper('.', &springs[1..], &runs, 0)
            + find_arrangements_helper('#', &springs[1..], &runs, 0);

    } else {
        result = find_arrangements_helper(next_spring, &springs[1..], &runs, 0);
    }

    return result;
}

fn part1(lines: &Vec<(Vec<char>, Vec<u8>)>) -> u64 {
    return lines.iter().fold(0, |acc, l| acc + find_arrangements(l));
}

fn part2(lines: &Vec<(Vec<char>, Vec<u8>)>) -> u64 {
    return 0;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<(Vec<char>, Vec<u8>)> = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let parts: Vec<_> = line.split_whitespace().collect();
            let springs = parts[0].chars().collect();
            let runs = parts[1].split(',').map(|r| r.parse().unwrap()).collect();

            (springs, runs)
        })
        .collect();

    let pt1_result = part1(&lines);
    let pt2_result = part2(&lines);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single_line_test(spring_str: &str, runs: Vec<u8>, expected_result: u64) {
        let springs: Vec<_> = spring_str.chars().collect();
        assert_eq!(find_arrangements(&(springs, runs)), expected_result);
    }

    fn single_line_extended_test(spring_str: &str, runs: Vec<u8>, expected_result: u64) {
        let springs: Vec<_> = spring_str.chars().collect();
        assert_eq!(find_arrangements_extended(&(springs, runs)), expected_result);
    }


    #[test]
    fn test_single_line_1() {
        single_line_test("#.#.###", vec![1, 1, 3], 1);
    }

    #[test]
    fn test_single_line_2() {
        single_line_test("???.###", vec![1, 1, 3], 1);
    }

    #[test]
    fn test_single_line_3() {
        single_line_test(".??..??...?##.", vec![1, 1, 3], 4);
    }

    #[test]
    fn test_single_line_4() {
        single_line_test("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6], 1);
    }

    #[test]
    fn test_single_line_5() {
        single_line_test("????.#...#...", vec![4, 1, 1], 1);
    }

    #[test]
    fn test_single_line_6() {
        single_line_test("????.######..#####.", vec![1, 6, 5], 4);
    }

    #[test]
    fn test_single_line_7() {
        single_line_test("?###????????", vec![3, 2, 1], 10);
    }

    #[test]
    fn pt1_test() {
        let raw_lines: Vec<(&str, Vec<u8>)> = vec![
            ("???.###", vec![1, 1, 3]),
            (".??..??...?##.", vec![1, 1, 3]),
            ("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6]),
            ("????.#...#...", vec![4, 1, 1]),
            ("????.######..#####.", vec![1, 6, 5]),
            ("?###????????", vec![3, 2, 1]),
        ];
        let lines: Vec<(Vec<char>, Vec<u8>)> = raw_lines
            .iter()
            .map(|(s, r)| (s.chars().collect(), r.clone()))
            .collect();

        let result = part1(&lines);
        assert_eq!(result, 21);
    }
}
