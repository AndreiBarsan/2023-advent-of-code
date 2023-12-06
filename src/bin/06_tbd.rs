use aoc23::concat_nums;
use std::path::PathBuf;

fn get_travel_dist(hold_time: usize, total_time: usize) -> usize {
    assert!(total_time >= hold_time);
    let travel_time = total_time - hold_time;
    let speed_mps = hold_time;
    let dist = travel_time * speed_mps;
    dist
}

fn get_successful_strategies(time: usize, distance_record_to_beat: usize) -> Vec<usize> {
    let succ: Vec<usize> = (0..(time as u64))
        .map(|t| get_travel_dist(t as usize, time as usize))
        .filter(|d| d > &distance_record_to_beat)
        .collect();
    succ
}

fn day_06_tbd(input_fpath: &PathBuf) -> (usize, usize) {
    let in_txt = std::fs::read_to_string(input_fpath)
        .expect(format!("Read input from {:?}", input_fpath).as_str());
    let lines: Vec<&str> = in_txt.split("\n").collect();

    let times: Vec<usize> = lines[0]
        .strip_prefix("Time:")
        .expect("")
        .split(' ')
        .map(|nr| nr.trim())
        .filter(|nr| nr.len() > 0)
        .map(|nr| usize::from_str_radix(nr, 10).expect(""))
        .collect();
    let distances: Vec<usize> = lines[1]
        .strip_prefix("Distance:")
        .expect("")
        .split(' ')
        .map(|nr| nr.trim())
        .filter(|nr| nr.len() > 0)
        .map(|nr| usize::from_str_radix(nr, 10).expect(""))
        .collect();

    // Part one: Compute number of ways to win for each scenario, and multiply them all together
    let part_one_successes = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance_record)| get_successful_strategies(*time, *distance_record).len());
    let part_one_answer = part_one_successes.product::<usize>();

    // Part two: The same, except the different numbers were actually all chunks of one big number
    let big_time = concat_nums(&times);
    let big_dist = concat_nums(&distances);
    let part_two_answer = get_successful_strategies(big_time, big_dist).len();

    (part_one_answer, part_two_answer)
}

fn main() {
    println!("{:?}", day_06_tbd(&PathBuf::from("input/06-demo.txt")));
    println!("{:?}", day_06_tbd(&PathBuf::from("input/06.txt")));
}
