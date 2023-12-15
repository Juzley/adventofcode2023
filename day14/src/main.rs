use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
enum TileType {
    Empty,
    SquareRock,
    RoundRock(u64),
}

#[allow(dead_code)]
fn print_dish(dish: &Vec<Vec<TileType>>) {
    for row in dish {
        for tile in row {
            match tile {
                TileType::Empty => print!("."),
                TileType::SquareRock => print!("#"),
                TileType::RoundRock(_) => print!("O"),
            }
        }
        println!("");
    }
}

fn rotate_dish(dish: &Vec<Vec<TileType>>) -> Vec<Vec<TileType>> {
    // Rotate the dish by 90 degrees clockwise, returning a new dish
    let mut new_dish = vec![vec![TileType::Empty; dish[0].len()]; dish.len()];

    for col in 0..dish[0].len() {
        for row in 0..dish.len() {
            new_dish[col][dish.len() - row - 1] = dish[row][col];
        }
    }

    return new_dish;
}

fn roll_rocks(dish: &Vec<Vec<TileType>>) -> Vec<Vec<TileType>> {
    let mut new_dish = dish.clone();

    for col in 0..dish[0].len() {
        let mut next_open = 0;

        for row in 0..dish.len() {
            match dish[row][col] {
                TileType::RoundRock(l) => {
                    new_dish[next_open][col] = TileType::RoundRock(l);
                    if next_open != row {
                        new_dish[row][col] = TileType::Empty;
                    }
                    next_open = next_open + 1;
                }
                TileType::SquareRock => next_open = row + 1,
                TileType::Empty => (),
            }
        }
    }

    return new_dish;
}

fn calc_load(dish: &Vec<Vec<TileType>>) -> usize {
    let mut load = 0;
    for (row, line) in dish.iter().enumerate() {
        for tile in line {
            if let TileType::RoundRock(_) = tile {
                load += dish.len() - row;
            }
        }
    }

    return load;
}

// Finds a cycle in the sequence of numbers, returning the index of the first number and the length of the cycle
fn find_cycle(
    seq: &Vec<usize>,
    min_cycle_len: usize,
    max_cycle_len: usize,
) -> Option<(usize, usize)> {
    'start_idx_loop: for start_idx in 0..seq.len() {
        for cycle_len in min_cycle_len..=max_cycle_len {
            if start_idx + cycle_len + cycle_len > seq.len() {
                // The sequence is too short to have a loop starting at this index for this loop size,
                // move onto the next index
                continue 'start_idx_loop;
            }

            let a = &seq[start_idx..(start_idx + cycle_len)];
            let b = &seq[(start_idx + cycle_len)..(start_idx + cycle_len + cycle_len)];

            if a == b {
                return Some((start_idx, cycle_len));
            }
        }
    }

    return None;
}

fn part2(dish: &Vec<Vec<char>>) -> usize {
    // Translate the input into a vector of vectors of tile types
    let mut start_dish = vec![];
    let mut rock_label = 0;
    let mut paths: HashMap<u64, Vec<(usize, usize)>> = HashMap::new();
    for (row, line) in dish.iter().enumerate() {
        let mut new_row = vec![];
        for (col, c) in line.iter().enumerate() {
            match c {
                '.' => new_row.push(TileType::Empty),
                '#' => new_row.push(TileType::SquareRock),
                'O' => {
                    new_row.push(TileType::RoundRock(rock_label));
                    paths.insert(rock_label, vec![(row, col)]);
                    rock_label += 1;
                }
                _ => panic!("Unknown character"),
            }
        }
        start_dish.push(new_row);
    }

    let mut current_dish = start_dish.clone();
    let mut loads = vec![];
    // Run a few cycles to get enough loads to find a cycle.
    for _cycle in 0..200 {
        for _rotation in 0..4 {
            current_dish = roll_rocks(&current_dish);
            current_dish = rotate_dish(&current_dish);
        }

        // Find where each round rock is, update the rock paths with the new positions
        for (row, line) in current_dish.iter().enumerate() {
            for (col, tile) in line.iter().enumerate() {
                if let TileType::RoundRock(l) = tile {
                    paths.entry(*l).and_modify(|e| e.push((row, col)));
                }
            }
        }

        loads.push(calc_load(&current_dish));
    }

    let cycle = find_cycle(&loads, 3, 50);
    match cycle {
        Some((start_idx, cycle_len)) => {
            return loads[((1000000000 - start_idx) % cycle_len) + start_idx - 1]
        }
        None => panic!("Could not find a cycle"),
    }
}

fn part1(dish: &Vec<Vec<char>>) -> usize {
    let mut result = 0;

    for col in 0..dish[0].len() {
        let mut next_open = 0;
        for row in 0..dish.len() {
            match dish[row][col] {
                'O' => {
                    result += dish.len() - next_open;
                    next_open = next_open + 1;
                }
                '#' => {
                    next_open = row + 1;
                }
                _ => (),
            }
        }
    }

    return result;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let dish = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let pt1_result = part1(&dish);
    let pt2_result = part2(&dish);
    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let lines = vec![
            String::from("O....#....").chars().collect(),
            String::from("O.OO#....#").chars().collect(),
            String::from(".....##...").chars().collect(),
            String::from("OO.#O....O").chars().collect(),
            String::from(".O.....O#.").chars().collect(),
            String::from("O.#..O.#.#").chars().collect(),
            String::from("..O..#O..O").chars().collect(),
            String::from(".......O..").chars().collect(),
            String::from("#....###..").chars().collect(),
            String::from("#OO..#....").chars().collect(),
        ];

        let result = part1(&lines);
        assert_eq!(result, 136);
    }

    #[test]
    fn part2_test() {
        let lines = vec![
            String::from("O....#....").chars().collect(),
            String::from("O.OO#....#").chars().collect(),
            String::from(".....##...").chars().collect(),
            String::from("OO.#O....O").chars().collect(),
            String::from(".O.....O#.").chars().collect(),
            String::from("O.#..O.#.#").chars().collect(),
            String::from("..O..#O..O").chars().collect(),
            String::from(".......O..").chars().collect(),
            String::from("#....###..").chars().collect(),
            String::from("#OO..#....").chars().collect(),
        ];

        let result = part2(&lines);
        assert_eq!(result, 64);
    }
}
