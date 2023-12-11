/*
Time:        40     82     84     92
Distance:   233   1011   1110   1487
*/

fn part1(races: &Vec<(u64, u64)>) -> u64 {
    let mut result = 1;
    for race in races {
        let race_time = race.0;
        let record = race.1;
        let mut win_count = 0;

        for hold_time in 1..race_time {
            let remainder = race_time - hold_time;
            let distance = remainder * hold_time;

            if distance > record {
                win_count += 1;
            }
        }

        result *= win_count;
    }

    return result;
}

fn main() {
    let races = vec![(40, 233), (82, 1011), (84, 1110), (92, 1487)];
    let pt1_result = part1(&races);

    let races = vec![(40828492, 233101111101487u64)];
    let pt2_result = part1(&races);

    println!("{}, {}", pt1_result, pt2_result);
}
