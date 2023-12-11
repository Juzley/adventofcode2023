use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq)]
struct GardenMapEntry {
    src_start: i64,
    dst_start: i64,
    count: i64,
}

fn remap_range(
    remap_range: &Range<i64>,
    src_range: &Range<i64>,
    dst_range: &Range<i64>,
) -> (Option<Range<i64>>, Vec<Range<i64>>) {
    if src_range.start <= remap_range.start && src_range.end >= remap_range.end {
        // Src range completely encloses remap range - remap this whole range, there's no remainder.
        let offset = dst_range.start - src_range.start;
        let remapped = (remap_range.start + offset)..(remap_range.end + offset);
        return (Some(remapped), vec![]);
    } else if remap_range.start <= src_range.start && remap_range.end >= src_range.end {
        // Remap range completely encloses remap range - need to split the remap range in 3.
        let remainders = [
            remap_range.start..src_range.start,
            src_range.end..remap_range.end,
        ]
        .into_iter()
        .filter(|x| x.start != x.end)
        .collect();
        return (Some(dst_range.clone()), remainders);
    } else if remap_range.start < src_range.start && remap_range.end > src_range.start {
        // Partial overlap, source range is higher
        let overlap_count = remap_range.end - src_range.start;
        let remapped = dst_range.start..(dst_range.start + overlap_count);

        if remap_range.start != src_range.start {
            return (Some(remapped), vec![remap_range.start..src_range.start]);
        } else {
            return (Some(remapped), vec![]);
        }
    } else if src_range.start < remap_range.start && src_range.end > remap_range.start {
        // Partial overlap, source range is lower
        let overlap_count = dst_range.start - src_range.start;
        let remapped = (remap_range.start + overlap_count)..dst_range.end;
        if src_range.end != remap_range.end {
            return (Some(remapped), vec![src_range.end..remap_range.end]);
        } else {
            return (Some(remapped), vec![]);
        }
    } else {
        // No overlap
        return (None, vec![remap_range.clone()]);
    }
}

fn apply_map_entry(
    src_range: &Range<i64>,
    dst_range: &Range<i64>,
    seed_ranges: &Vec<Range<i64>>,
) -> (Vec<Range<i64>>, Vec<Range<i64>>) {
    let mut unmoved = vec![];
    let mut moved = vec![];
    let mut ranges = seed_ranges.clone();

    while !ranges.is_empty() {
        let cur_range = ranges.pop().unwrap();
        let (remapped, mut remainders) = remap_range(&cur_range, &src_range, &dst_range);

        unmoved.append(&mut remainders);
        if let Some(new_range) = remapped {
            if new_range.start == 0 || new_range.contains(&1459646077) {
                println!(
                    "Map Entry {:?} -> {:?} moved {:?} -> {:?}",
                    src_range, dst_range, cur_range, new_range
                );
            }
            moved.push(new_range);
        }
    }

    println!(
        "Map Entry {:?} -> {:?} processed {:?}. Moved {:?}, Unmoved {:?}",
        src_range, dst_range, seed_ranges, moved, unmoved
    );

    return (moved, unmoved);
}

fn apply_map(
    map: &Vec<(Range<i64>, Range<i64>)>,
    seed_ranges: &Vec<Range<i64>>,
) -> Vec<Range<i64>> {
    let mut to_process = seed_ranges.clone();
    let mut result = vec![];

    for map_entry in map {
        let (mut moved, unmoved) = apply_map_entry(&map_entry.1, &map_entry.0, &to_process);

        // Check whether the next map entry moves any of the ranges that weren't moved by
        // this entry.
        to_process = unmoved;

        result.append(&mut moved);
    }

    result.append(&mut to_process);

    return result;
}

fn part2_parse(lines: &Vec<String>) -> (Vec<Range<i64>>, Vec<Vec<(Range<i64>, Range<i64>)>>) {
    let seeds_regex = Regex::new(r"seeds: (?<seeds>.*)").unwrap();
    let map_delim_regex = Regex::new(r".* map:").unwrap();
    let map_entry_regex = Regex::new(r"(?<dst_start>\d+) (?<src_start>\d+) (?<count>\d+)").unwrap();

    let mut seed_ranges: Vec<Range<i64>> = vec![];
    let mut maps: Vec<Vec<(Range<i64>, Range<i64>)>> = vec![];
    let mut cur_map: Vec<(Range<i64>, Range<i64>)> = vec![];

    for line in lines {
        if let Some(caps) = seeds_regex.captures(line) {
            let tmp: Vec<i64> = caps["seeds"]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            seed_ranges = tmp.chunks(2).map(|s| s[0]..(s[0] + s[1])).collect();
        } else if let Some(_caps) = map_delim_regex.captures(line) {
            if !cur_map.is_empty() {
                maps.push(cur_map);
                cur_map = vec![];
            }
        } else if let Some(caps) = map_entry_regex.captures(line) {
            let src_start = caps["src_start"].parse().unwrap();
            let dst_start = caps["dst_start"].parse().unwrap();
            let count: i64 = caps["count"].parse().unwrap();

            cur_map.push((
                dst_start..(dst_start + count),
                src_start..(src_start + count),
            ));
        }
    }
    maps.push(cur_map);

    return (seed_ranges, maps);
}

fn part2(lines: &Vec<String>) -> i64 {
    let (seed_ranges, maps) = part2_parse(lines);

    let mut min_location = i64::MAX;
    for seed_range in seed_ranges {
        println!("New seeds {:?}", seed_range);
        let mut cur_ranges = vec![seed_range];

        for map in &maps {
            println!("New map, ranges {:?}", cur_ranges);
            cur_ranges = apply_map(map, &cur_ranges);
        }

        cur_ranges.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
        println!("Result: {:?}", cur_ranges);
        min_location = std::cmp::min(min_location, cur_ranges[0].start);
    }

    return min_location;
}

fn part1(lines: &Vec<String>) -> i64 {
    let seeds_regex = Regex::new(r"seeds: (?<seeds>.*)").unwrap();
    let map_delim_regex = Regex::new(r".* map:").unwrap();
    let map_entry_regex = Regex::new(r"(?<dst_start>\d+) (?<src_start>\d+) (?<count>\d+)").unwrap();

    let mut seeds: Vec<i64> = vec![];
    let mut maps: Vec<Vec<GardenMapEntry>> = vec![];
    let mut cur_map: Vec<GardenMapEntry> = vec![];

    for line in lines {
        if let Some(caps) = seeds_regex.captures(line) {
            seeds = caps["seeds"]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
        } else if let Some(_caps) = map_delim_regex.captures(line) {
            if !cur_map.is_empty() {
                maps.push(cur_map);
                cur_map = vec![];
            }
        } else if let Some(caps) = map_entry_regex.captures(line) {
            cur_map.push(GardenMapEntry {
                src_start: caps["src_start"].parse().unwrap(),
                dst_start: caps["dst_start"].parse().unwrap(),
                count: caps["count"].parse().unwrap(),
            });
        }
    }
    maps.push(cur_map);

    let mut min_location = i64::MAX;
    for seed in seeds {
        let mut cur_idx = seed;
        for map in &maps {
            let entry = map
                .iter()
                .find(|&&e| cur_idx >= e.src_start && cur_idx < e.src_start + e.count);
            cur_idx = entry
                .map(|e| (cur_idx - e.src_start) + e.dst_start)
                .unwrap_or(cur_idx);
        }
        min_location = std::cmp::min(cur_idx, min_location);
    }

    return min_location;
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
    fn remap_range_no_overlap() {
        let (remapped, remainders) = remap_range(&(0..4), &(5..7), &(8..10));
        assert_eq!(remapped, None);
        assert_eq!(remainders, vec![(0..4)]);
    }

    #[test]
    fn remap_range_src_encloses() {
        let (remapped, remainders) = remap_range(&(1..3), &(0..4), &(5..9));
        assert_eq!(remapped, Some(6..8));
        assert!(remainders.is_empty());
    }

    #[test]
    fn remap_range_remap_encloses() {
        let (remapped, remainders) = remap_range(&(0..4), &(1..3), &(5..7));
        assert_eq!(remapped, Some(5..7));
        assert_eq!(remainders, vec![0..1, 3..4]);
    }

    #[test]
    fn remap_range_partial_remap_lower() {
        let (remapped, remainders) = remap_range(&(0..4), &(2..6), &(6..10));
        assert_eq!(remapped, Some(6..8));
        assert_eq!(remainders, vec![0..2]);
    }

    #[test]
    fn remap_range_partial_remap_higher() {
        let (remapped, remainders) = remap_range(&(2..5), &(0..4), &(6..10));
        assert_eq!(remapped, Some(8..10));
        assert_eq!(remainders, vec![4..5]);
    }

    #[test]
    fn pt1_test() {
        let lines = vec![
            String::from("seeds: 79 14 55 13"),
            String::from("seed-to-soil map:"),
            String::from("50 98 2"),
            String::from("52 50 48"),
            String::from("soil-to-fertilizer map:"),
            String::from("0 15 37"),
            String::from("37 52 2"),
            String::from("39 0 15"),
            String::from("fertilizer-to-water map:"),
            String::from("49 53 8"),
            String::from("0 11 42"),
            String::from("42 0 7"),
            String::from("57 7 4"),
            String::from("water-to-light map:"),
            String::from("88 18 7"),
            String::from("18 25 70"),
            String::from("light-to-temperature map:"),
            String::from("45 77 23"),
            String::from("81 45 19"),
            String::from("68 64 13"),
            String::from("temperature-to-humidity map:"),
            String::from("0 69 1"),
            String::from("1 0 69"),
            String::from("humidity-to-location map:"),
            String::from("60 56 37"),
            String::from("56 93 4"),
        ];

        let result = part1(&lines);
        assert_eq!(result, 35);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("seeds: 79 14 55 13"),
            String::from("seed-to-soil map:"),
            String::from("50 98 2"),
            String::from("52 50 48"),
            String::from("soil-to-fertilizer map:"),
            String::from("0 15 37"),
            String::from("37 52 2"),
            String::from("39 0 15"),
            String::from("fertilizer-to-water map:"),
            String::from("49 53 8"),
            String::from("0 11 42"),
            String::from("42 0 7"),
            String::from("57 7 4"),
            String::from("water-to-light map:"),
            String::from("88 18 7"),
            String::from("18 25 70"),
            String::from("light-to-temperature map:"),
            String::from("45 77 23"),
            String::from("81 45 19"),
            String::from("68 64 13"),
            String::from("temperature-to-humidity map:"),
            String::from("0 69 1"),
            String::from("1 0 69"),
            String::from("humidity-to-location map:"),
            String::from("60 56 37"),
            String::from("56 93 4"),
        ];

        let result = part2(&lines);
        assert_eq!(result, 46);
    }
}
