use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_start(map: &Vec<Vec<char>>) -> (i32, i32) {
    for (r, row) in map.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                return (r as i32, c as i32);
            }
        }
    }

    return (0, 0);
}

fn find_valid_starts(map: &Vec<Vec<char>>, start: (i32, i32)) -> Vec<(i32, i32)> {
    let mut starts = vec![];
    let map_height = map.len() as i32;
    let map_width = map[0].len() as i32;

    // Check whether the square above could be a valid path
    if start.0 > 0 {
        let candidate = map[(start.0 - 1) as usize][start.1 as usize];
        if candidate == 'F' || candidate == '7' || candidate == '|' {
            starts.push((start.0 - 1, start.1));
        }
    }

    // Check whether the square below could be a valid path
    if start.0 < map_height - 1 {
        let candidate = map[(start.0 + 1) as usize][start.1 as usize];
        if candidate == 'J' || candidate == 'L' || candidate == '|' {
            starts.push((start.0 + 1, start.1));
        }
    }

    // Check whether the square to the left could be a valid path
    if start.1 > 0 {
        let candidate = map[start.0 as usize][(start.1 - 1) as usize];
        if candidate == '-' || candidate == 'F' || candidate == 'J' || candidate == 'L' || candidate == '7' {
            starts.push((start.0, start.1 - 1));
        }
    }

    // Check whether the square to the right could be a valid path
    if start.1 < map_width - 1 {
        let candidate = map[start.0 as usize][(start.1 + 1) as usize];
        if candidate == '-' || candidate == 'F' || candidate == 'J' || candidate == 'L' || candidate == '7' {
            starts.push((start.0, start.1 + 1));
        }
    }

    return starts;
}

fn find_loop(map: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let map_height = map.len() as i32;
    let map_width = map[0].len() as i32;
    let start = find_start(&map);
    let starts = find_valid_starts(&map, start);

    for s in starts {
        let mut cur = s;
        let mut prev: (i32, i32) = start;
        let mut result = HashSet::new();

        'find_loop: loop {
            result.insert(cur);

            let candidates: [(i32, i32); 2];
            match map[cur.0 as usize][cur.1 as usize] {
                '.' => break 'find_loop, // Hit a blank square, not a loop.
                'S' => return result, // End of loop
                '-' => candidates = [(cur.0, cur.1 - 1), (cur.0, cur.1 + 1)],
                '|' => candidates = [(cur.0 - 1, cur.1), (cur.0 + 1, cur.1)],
                'F' => candidates = [(cur.0 + 1, cur.1), (cur.0, cur.1 + 1)],
                '7' => candidates = [(cur.0 + 1, cur.1), (cur.0, cur.1 - 1)],
                'L' => candidates = [(cur.0 - 1, cur.1), (cur.0, cur.1 + 1)],
                'J' => candidates = [(cur.0 - 1, cur.1), (cur.0, cur.1 - 1)],
                _ => candidates = [(0, 0), (0, 0)],
            }

            let next = candidates
                .iter()
                .filter(|&&c| {
                    c != prev && c.0 >= 0 && c.0 < map_height && c.1 >= 0 && c.1 < map_width
                })
                .next();
            match next {
                None => break 'find_loop, // No valid candidates, not a loop.
                Some(&c) => {
                    prev = cur;
                    cur = c
                }
            }
        }
    }

    return HashSet::new();
}

fn part2(map: &Vec<Vec<char>>) -> u32 {
    let path = find_loop(map);
    let mut result = 0;
    for (r, row) in map.iter().enumerate() {
        let mut inside: bool = false;
        let mut prev = '.';
        for (c, &ch) in row.iter().enumerate() {
            let coord = (r as i32, c as i32);
            let contains = path.contains(&coord);

            if !contains && inside {
                result += 1;
            } else if contains {
                // Raycasting algorithm - scan for intersections with vertical lines.
                // Treat pairs of corners moving in opposite directions as vertical.
                if ch == '|' || (ch == 'J' && prev == 'F') || (ch == '7' && prev == 'L') {
                    inside = !inside; 
                    prev = '.';
                } else if ch == 'J' || ch == 'F' || ch == 'L' || ch == '7' {
                    prev = ch;
                }
            }
        }
    }

    return result;
}

fn part1(map: &Vec<Vec<char>>) -> u32 {
    let path = find_loop(&map);
    return (path.len() as u32 + 1) / 2;
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<char>> {
    return lines.iter().map(|l| l.chars().collect()).collect();
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let map = parse_input(&lines);

    let pt1_result = part1(&map);
    let pt2_result = part2(&map);
    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test_1() {
        let lines = vec![
            String::from("....."),
            String::from(".S-7."),
            String::from(".|.|."),
            String::from(".L-J."),
            String::from("....."),
        ];

        let map = parse_input(&lines);
        let result = part1(&map);
        assert_eq!(result, 4);
    }

    #[test]
    fn pt1_test_2() {
        let lines = vec![
            String::from("7-F7-"),
            String::from(".FJ|7"),
            String::from("SJLL7"),
            String::from("|F--J"),
            String::from("LJ.LJ"),
        ];

        let map = parse_input(&lines);
        let result = part1(&map);
        assert_eq!(result, 8);
    }

    #[test]
    fn pt2_test_1() {
        let lines = vec![
            String::from("..........."),
            String::from(".S-------7."),
            String::from(".|F-----7|."),
            String::from(".||.....||."),
            String::from(".||.....||."),
            String::from(".|L-7.F-J|."),
            String::from(".|..|.|..|."),
            String::from(".L--J.L--J."),
            String::from("..........."),
        ];

        let map = parse_input(&lines);
        let result = part2(&map);
        assert_eq!(result, 4);
    }

    #[test]
    fn pt2_test_2() {
        let lines = vec![
            String::from(".F----7F7F7F7F-7...."),
            String::from(".|F--7||||||||FJ...."),
            String::from(".||.FJ||||||||L7...."),
            String::from("FJL7L7LJLJ||LJ.L-7.."),
            String::from("L--J.L7...LJS7F-7L7."),
            String::from("....F-J..F7FJ|L7L7L7"),
            String::from("....L7.F7||L7|.L7L7|"),
            String::from(".....|FJLJ|FJ|F7|.LJ"),
            String::from("....FJL-7.||.||||..."),
            String::from("....L---J.LJ.LJLJ..."),
        ];

        let map = parse_input(&lines);
        let result = part2(&map);
        assert_eq!(result, 8);
    }

    #[test]
    fn pt2_test_3() {
        let lines = vec![
            String::from("FF7FSF7F7F7F7F7F---7"),
            String::from("L|LJ||||||||||||F--J"),
            String::from("FL-7LJLJ||||||LJL-77"),
            String::from("F--JF--7||LJLJ7F7FJ-"),
            String::from("L---JF-JLJ.||-FJLJJ7"),
            String::from("|F|F-JF---7F7-L7L|7|"),
            String::from("|FFJF7L7F-JF7|JL---7"),
            String::from("7-L-JL7||F7|L7F-7F7|"),
            String::from("L.L7LFJ|||||FJL7||LJ"),
            String::from("L7JLJL-JLJLJL--JLJ.L"),
        ];

        let map = parse_input(&lines);
        let result = part2(&map);
        assert_eq!(result, 10);
    }
}
