use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn continue_path((row, col): (i32, i32), from: Direction) -> ((i32, i32), Direction) {
    match from {
        Direction::Right => return ((row, col - 1), from),
        Direction::Left => return ((row, col + 1), from),
        Direction::Up => return ((row + 1, col), from),
        Direction::Down => return ((row - 1, col), from),
    }
}

fn count_energized(entry: ((i32, i32), Direction), grid: &Vec<Vec<char>>) -> u32 {
    let mut paths: Vec<((i32, i32), Direction)> = vec![entry];
    let mut visited: HashSet<((i32, i32), Direction)> = HashSet::new();

    while !paths.is_empty() {
        let (loc, from) = paths.pop().unwrap();
        let row = loc.0;
        let col = loc.1;
        visited.insert((loc, from));

        let mut candidates = vec![];
        match grid[row as usize][col as usize] {
            '|' => {
                if from == Direction::Right || from == Direction::Left {
                    candidates.push(((row - 1, col), Direction::Down));
                    candidates.push(((row + 1, col), Direction::Up));
                } else {
                    candidates.push(continue_path(loc, from));
                }
            }
            '/' => match from {
                Direction::Up => candidates.push(((row, col - 1), Direction::Right)),
                Direction::Down => candidates.push(((row, col + 1), Direction::Left)),
                Direction::Left => candidates.push(((row - 1, col), Direction::Down)),
                Direction::Right => candidates.push(((row + 1, col), Direction::Up)),
            }
            '\\' => match from {
                Direction::Up => candidates.push(((row, col + 1), Direction::Left)),
                Direction::Down => candidates.push(((row, col - 1), Direction::Right)),
                Direction::Left => candidates.push(((row + 1, col), Direction::Up)),
                Direction::Right => candidates.push(((row - 1, col), Direction::Down)),
            },
            '-' => {
                if from == Direction::Up || from == Direction::Down {
                    candidates.push(((row, col - 1), Direction::Right));
                    candidates.push(((row, col + 1), Direction::Left));
                } else {
                    candidates.push(continue_path(loc, from));
                }
            }
            _ => candidates.push(continue_path(loc, from)),
        }

        paths.extend(candidates.iter().filter(|&c| {
            let row = c.0.0;
            let col = c.0.1;

            !visited.contains(&c)
                && row >= 0
                && row < grid.len() as i32
                && col >= 0
                && col < grid[0].len() as i32
        }));
    }

    let energized: HashSet<_> = visited.iter().map(|v| v.0).collect();

    /*for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if energized.contains(&(i as i32, j as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }*/

    return energized.len() as u32;
}

fn part1(grid: &Vec<Vec<char>>) -> u32 {
    return count_energized(((0, 0), Direction::Left), grid);
}

fn part2(grid: &Vec<Vec<char>>) -> u32 {
    let mut max = 0;

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    
    for i in 0..width {
        max = *vec![
            max,
            count_energized(((i, 0), Direction::Left), &grid),
            count_energized(((i, height - 1), Direction::Right), &grid),
        ].iter().max().unwrap();
    }

    for i in 0..height {
        max = *vec![
            max,
            count_energized(((0, i), Direction::Up), &grid),
            count_energized(((0, width - 1), Direction::Down), &grid),
        ].iter().max().unwrap();
    }

    return max;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let pt1_result = part1(&lines);
    let pt2_result = part2(&lines);
    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let lines = vec![
            String::from(".|...\\...."),
            String::from("|.-.\\....."),
            String::from(".....|-..."),
            String::from("........|."),
            String::from(".........."),
            String::from(".........\\"),
            String::from("..../.\\\\.."),
            String::from(".-.-/..|.."),
            String::from(".|....-|.\\"),
            String::from("..//.|...."),
        ]
        .iter()
        .map(|s| s.chars().collect())
        .collect();

        let result = part1(&lines);
        assert_eq!(result, 46);
    }

    #[test]
    fn part2_test() {
        let lines = vec![
            String::from(".|...\\...."),
            String::from("|.-.\\....."),
            String::from(".....|-..."),
            String::from("........|."),
            String::from(".........."),
            String::from(".........\\"),
            String::from("..../.\\\\.."),
            String::from(".-.-/..|.."),
            String::from(".|....-|.\\"),
            String::from("..//.|...."),
        ]
        .iter()
        .map(|s| s.chars().collect())
        .collect();

        let result = part2(&lines);
        assert_eq!(result, 51);
    }
}
