use std::cmp::Ordering;
use std::{path::PathBuf, collections::HashMap};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Card<'a> {
    val: &'a str,
}

fn get_value(card: char, joker_enabled: bool) -> u8 {
    if let Some(digit) = card.to_digit(10) {
        digit as u8
    }
    else {
        match (card, joker_enabled) {
            ('T', _) => 10,
            // A very important distinction without it it's easy to make subtle mistakes in Part 2, since a buffed
            // joker when it comes to getting a hand's tier gets nerfed when used to rank hands within a tier!
            ('J', true) => 1,
            ('J', false) => 11,
            ('Q', _) => 12,
            ('K', _) => 13,
            ('A', _) => 14,
            _ => panic!("Invalid card {}", card),
        }
    }
}

impl Card<'_> {
    fn get_counts(&self) -> HashMap<char, usize> {
        self.val.chars().counts()
    }

    fn get_card_vals(&self, joker_enabled: bool) -> Vec<u8> {
        self.val.chars().map(|card| get_value(card, joker_enabled)).collect()
    }

    fn get_tier(&self) -> usize {
        let mut c: Vec<(char, usize)> = self.get_counts().into_iter().collect();
        c.sort_by(|e1, e2| e2.1.cmp(&e1.1));
        let second_highest_count = c.get(1).map(|x| x.1).unwrap_or_default();

        match (c[0].1, second_highest_count) {
            (5, _) => 0,
            (4, _) => 1,
            (3, 2) => 2,
            (3, _) => 3,
            (2, 2) => 4,
            (2, _) => 5,
            _ => 6
        }
    }

    fn get_tier_joker(&self) -> usize {
        let counts = self.get_counts();
        let mut c: Vec<(char, usize)> = counts.clone().into_iter().collect();
        c.sort_by(|e1, e2| e2.1.cmp(&e1.1));

        if c[0].1 == 5 {
            // Only one way to get a pure five-of-a-kind.
            return 0;
        }

        let n_j = counts.get(&'J').unwrap_or(&0);
        let c_non_j_map: HashMap<char, usize> = counts.clone().into_iter().filter(|c| c.0 != 'J').collect();
        let mut c_non_j: Vec<(char, usize)> = c_non_j_map.clone().into_iter().collect();
        c_non_j.sort_by(|e1, e2| e2.1.cmp(&e1.1));

        let highest_nonj_count = c_non_j.get(0).map(|x|x.1).unwrap_or_default();
        let highest_theoretical = highest_nonj_count + n_j;
        let second_highest_count = c_non_j.get(1).map(|x|x.1).unwrap_or_default();
        assert!(second_highest_count <= highest_theoretical);

        match (highest_theoretical, second_highest_count) {
            (5, _) => 0,
            (4, _) => 1,
            (3, 2) => 2,
            (3, _) => 3,
            (2, 2) => 4,
            (2, _) => 5,
            _ => 6
        }

    }
}

impl Card<'_> {
    fn cmp_joker(&self, other: &Self, joker_enabled: bool) -> Ordering {
        for (v1, v2) in self.get_card_vals(joker_enabled).iter().zip(other.get_card_vals(joker_enabled).iter()) {
            if v1 > v2 {
                return Ordering::Greater;
            }
            else if v1 < v2 {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}


fn parse_card_bid<'a>(line: &'a str) -> (Card<'a>, u64) {
    let parts: Vec<&str> = line.split_terminator(" ").collect();
    (
        Card{ val: parts[0] },
        u64::from_str_radix(parts[1], 10).expect("Parse bid value failed")
    )
}



fn day_07_camel_cards(input_fpath: &PathBuf) -> (usize, usize) {
    let in_txt = std::fs::read_to_string(input_fpath)
        .expect(format!("Read input from {:?}", input_fpath).as_str());
    let lines: Vec<&str> = in_txt.split("\n").collect();
    let card_bids: Vec<(Card, u64)> = lines.iter().map(|l| parse_card_bid(l)).collect();
    let mut tiers: Vec<Vec<(&Card, u64)>> = Vec::new();
    for _ in 0..7 {
        tiers.push(Vec::new());
    }
    for (card, bid) in card_bids.iter() {
        tiers[card.get_tier()].push((card, *bid));
    }
    let mut all: Vec<(&Card, u64)> = Vec::new();
    for tier in tiers.iter_mut().rev() {
        tier.sort_by(|l, r| l.0.cmp_joker(&r.0, false));
        let mut tc = tier.clone();
        all.append(&mut tc);
    }

    let raw: Vec<usize> = all.iter().enumerate().map(|(idx, cv)| (idx + 1) * (cv.1 as usize)).collect();
    let part_one_answer: usize = raw.iter().sum();

    let mut all_joker: Vec<(&Card, u64)> = Vec::new();
    let mut jtiers: Vec<Vec<(&Card, u64)>> = Vec::new();
    for _ in 0..7 {
        jtiers.push(Vec::new());
    }
    for (card, bid) in card_bids.iter() {
        jtiers[card.get_tier_joker()].push((card, *bid));
    }
    for tier in jtiers.iter_mut().rev() {
        tier.sort_by(|l, r| l.0.cmp_joker(&r.0, true));
        let mut tc = tier.clone();
        all_joker.append(&mut tc);
    }

    let part_two_answer = all_joker.iter().enumerate().map(|(idx, cv)| (idx + 1) * (cv.1 as usize)).sum();

    (part_one_answer, part_two_answer)
}

fn main() {
    println!("{:?}", day_07_camel_cards(&PathBuf::from("input/07-demo-00.txt")));
    println!("{:?}", day_07_camel_cards(&PathBuf::from("input/07-demo-01.txt")));
    println!("{:?}", day_07_camel_cards(&PathBuf::from("input/07.txt")));
}