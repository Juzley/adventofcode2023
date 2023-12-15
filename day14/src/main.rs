use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(dish: &Vec<Vec<char>>) -> usize {
    let mut result = 0;

    for col in 0..dish[0].len() {
        let mut next_open = 0;
        for row in 0..dish.len() {
            match dish[row][col] {
                'O' => {
                    println!("Rock at {}, {}. Dish len {}", col, next_open, dish.len());
                    result += dish.len() - next_open;
                    next_open = next_open + 1;
                }
                '#' => {
                    next_open = row + 1;
                    println!("Block at {}, {}, set next open to {}", col, row, next_open);
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
    println!("Part 1: {}", pt1_result);
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
}
