use std::path::PathBuf;

// Time:      7  15   30
// Distance:  9  40  200

fn get_travel_dist(hold_time: usize, total_time: usize) -> usize {
    assert!(total_time >= hold_time);
    let travel_time = total_time - hold_time;
    let speed_mps= hold_time;
    let dist = travel_time * speed_mps;
    dist
}


fn day_06_tbd(input_fpath: &PathBuf) -> (usize, usize) {
    let in_txt = std::fs::read_to_string(input_fpath).expect(format!("Read input from {:?}", input_fpath).as_str());
    let lines: Vec<&str> = in_txt.split("\n").collect();

    let times: Vec<usize> = lines[0].strip_prefix("Time:").expect("").split(' ').map(|nr| nr.trim())
        .filter(|nr| nr.len() > 0).map(|nr| usize::from_str_radix(nr, 10).expect("")).collect();
    let distances: Vec<usize> = lines[1].strip_prefix("Distance:").expect("").split(' ').map(|nr| nr.trim())
        .filter(|nr| nr.len() > 0).map(|nr| usize::from_str_radix(nr, 10).expect("")).collect();

    println!("Distances:{:?}", distances);

    let mut succs: Vec<usize> = vec![];
    for (time, distance_record) in times.iter().zip(distances.iter()) {
        let succ: Vec<usize> = (0..(*time as u64)).map(|t| get_travel_dist(t as usize, *time as usize))
            .filter(|d| d > distance_record).collect();
        succs.push(succ.len())
    }

    let mut f = 1;
    for s in succs {
        f *= s;
    }

    let times = vec![59796575usize];
    let distances = vec![597123410321328usize];

    let mut succs: Vec<usize> = vec![];
    for (time, distance_record) in times.iter().zip(distances.iter()) {
        let succ: Vec<usize> = (0..(*time as u64)).map(|t| get_travel_dist(t as usize, *time as usize))
            .filter(|d| d > distance_record).collect();
        succs.push(succ.len())
    }

    let mut f2 = 1;
    for s in succs {
        f2 *= s;
    }


    // let seed_line = lines[0].strip_prefix("seeds: ").expect("");
    // let seed_ids: Vec<usize> = seed_line.split(' ').map(|nr| usize::from_str_radix(nr, 10).expect("")).collect();

    (f, f2)
}

fn main() {
    println!("{:?}", day_06_tbd(&PathBuf::from("input/06-demo.txt")));
    println!("{:?}", day_06_tbd(&PathBuf::from("input/06.txt")));
}