use std::{path::PathBuf, collections::HashMap};

#[derive(Debug)]
struct Location {
    name: String,
    left: String,
    right: String,
}

fn parse_location(line: &&str) -> Location {
    // AAA = (BBB, CCC)

    let sides: Vec<&str> = line.split("=").collect();
    let name = sides[0].trim().to_string();
    let junk: &[_] = &['(', ')', ',', ' '];
    let children: Vec<&str> = sides[1].split(",").map(|c| c.trim_matches(junk)).collect();

    let left = children[0].to_string();
    let right = children[1].to_string();

    Location { name, left, right }
}


fn day_08_haunted_wasteland(input_fpath: &PathBuf) -> (usize, usize) {
    let in_txt = std::fs::read_to_string(input_fpath)
        .expect(format!("Read input from {:?}", input_fpath).as_str());
    let lines: Vec<&str> = in_txt.split_terminator("\n").collect();

    let lr_steps: Vec<char> = lines[0].chars().collect();
    let locations: HashMap<String, Location> = lines[2..].iter().map(parse_location).map(|l| (l.name.clone(), l)).collect();

    // TODO(andrei): Clean up code and add tests.
    // let mut pos = &locations["AAA"];
    // let mut sidx = 0;
    let mut steps = 0;
    // loop {
    //     if pos.name == "ZZZ" {
    //         break;
    //     }

    //     if lr_steps[sidx] == 'L' {
    //         pos = &locations[&pos.left];
    //     }
    //     else {
    //         pos = &locations[&pos.right];
    //     }

    //     steps += 1;
    //     sidx = (sidx + 1) % lr_steps.len();
    // }

    let mut multipos: Vec<&Location> = locations.values().filter(|l| l.name.ends_with("A")).collect();
    let n_ghosts = multipos.len();
    println!("{:?}", multipos);

    let mut counts: Vec<usize> = Vec::new();
    for mp in multipos {
        let mut pp = mp;
        let mut sidx = 0;
        let mut steps2 = 0;
        loop {
            if pp.name.ends_with("Z") {
                break;
            }

            if lr_steps[sidx] == 'L' {
                pp = &locations[&pp.left];
            }
            else {
                pp = &locations[&pp.right];
            }

            steps2 += 1;
            sidx = (sidx + 1) % lr_steps.len();
        }
        counts.push(steps2);
    }


    let part_one_answer = steps;
    let part_two_answer = counts.into_iter()
        .reduce(|l, r| l * r / (&gcdusize(l, r))).expect("");

    (part_one_answer, part_two_answer)
}

pub fn gcdusize(n: usize, m: usize) -> usize {
    gcd(n as u64, m as u64) as usize
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
  assert!(n != 0 && m != 0);
  while m != 0 {
    if m < n {
      std::mem::swap(&mut m, &mut n);
    }
    m %= n;
  }
  n
}

fn main() {
    // println!("{:?}", day_08_tbd(&PathBuf::from("input/08-demo-01.txt")));
    println!("{:?}", day_08_haunted_wasteland(&PathBuf::from("input/08-demo-02.txt")));
    println!("{:?}", day_08_haunted_wasteland(&PathBuf::from("input/08.txt")));
}