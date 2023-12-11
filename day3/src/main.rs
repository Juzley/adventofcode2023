use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Number {
    number: u32,
    row: isize,
    start_col: isize,
    end_col: isize,
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut result = 0;

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: HashSet<(isize, isize)> = HashSet::new();

    let mut cur_num: Option<Number> = None;

    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '.' || c == '\r' || c == '\n' {
                // Blank space, check if we terminated a number.
                if cur_num.is_some() {
                    let mut num = cur_num.take().unwrap();
                    num.end_col = (col - 1) as isize;
                    numbers.push(num);
                }
                continue;
            } else if c.is_digit(10) {
                // Start or continue a number.
                let digit: u32 = c.to_digit(10).unwrap();
                let num = cur_num.get_or_insert(Number {
                    number: 0,
                    row: row as isize,
                    start_col: col as isize,
                    end_col: 0 as isize,
                });
                num.number = num.number * 10 + digit;
            } else {
                // Symbol - need to add to the symbols list and also check if we've terminated a number.
                symbols.insert((row as isize, col as isize));

                if cur_num.is_some() {
                    let mut num = cur_num.take().unwrap();
                    num.end_col = (col - 1) as isize;
                    numbers.push(num);
                }
            }
        }

        // End of the line, check if we still need to terminate a number.
        if cur_num.is_some() {
            let mut num = cur_num.take().unwrap();
            num.end_col = (line.len() - 1) as isize;
            numbers.push(num);
        }
    }

    'check_nums: for num in numbers {
        for col in num.start_col..=num.end_col {
            if symbols.contains(&(num.row, col - 1))
                || symbols.contains(&(num.row, col + 1))
                || symbols.contains(&(num.row - 1, col))
                || symbols.contains(&(num.row + 1, col))
                || symbols.contains(&(num.row - 1, col - 1))
                || symbols.contains(&(num.row - 1, col + 1))
                || symbols.contains(&(num.row + 1, col - 1))
                || symbols.contains(&(num.row + 1, col + 1))
            {
                result += num.number;
                continue 'check_nums;
            }
        }
    }

    return result;
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut result = 0;

    let mut numbers: Vec<Number> = Vec::new();
    let mut gears: Vec<(isize, isize)> = Vec::new();

    let mut cur_num: Option<Number> = None;
    for (row_, line) in lines.into_iter().enumerate() {
        let row = row_ as isize;
        for (col_, c) in line.chars().enumerate() {
            let col = col_ as isize;
            if c.is_digit(10) {
                // Start or continue a number.
                let digit: u32 = c.to_digit(10).unwrap();
                let num = cur_num.get_or_insert(Number {
                    number: 0,
                    row: row,
                    start_col: col,
                    end_col: 0,
                });
                num.number = num.number * 10 + digit;
            } else {
                // Not a number, terminate any current number and check for the gear symbol.
                if cur_num.is_some() {
                    let mut num = cur_num.take().unwrap();
                    num.end_col = col - 1;
                    numbers.push(num);
                }

                if c == '*' {
                    gears.push((row, col));
                }
            }
        }

        // End of the line, check if we still need to terminate a number.
        if cur_num.is_some() {
            let mut num = cur_num.take().unwrap();
            num.end_col = (line.len() - 1) as isize;
            numbers.push(num);
        }
    }

    // Could do something more efficient to optimize number lookup, but input isn't that big.
    for (row, col) in gears {
        let mut touching_nums = vec![];
        for num in &numbers {
            let row_range = (num.row - 1)..=(num.row + 1);
            let col_range = (num.start_col - 1)..=(num.end_col + 1);
            if row_range.contains(&row) && col_range.contains(&col) {
                touching_nums.push(num.number);
            }
        }

        if touching_nums.len() == 2 {
            result += touching_nums[0] * touching_nums[1];
        }
    }

    return result;
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
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ];

        let result = part1(&lines);
        assert_eq!(result, 4361);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ];

        let result = part2(&lines);
        assert_eq!(result, 467835);
    }
}
