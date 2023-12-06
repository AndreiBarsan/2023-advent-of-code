use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
struct Number {
    value: usize,
    row: usize,
    start_idx: usize,
    end_idx: usize,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    row: usize,
    col: usize,
}

impl Number {
    fn borders(&self, row: usize, col: usize) -> bool {
        (self.row).abs_diff(row) <= 1
            && col >= (self.start_idx.checked_sub(1).unwrap_or(0))
            && col <= (self.end_idx + 1)
    }
}

fn parse_number(chrs: &Vec<char>, start_idx: usize) -> (usize, usize) {
    let mut cur = 0usize;
    let mut idx = start_idx;
    while let Some(digit) = chrs[idx].to_digit(10) {
        cur = cur * 10 + (digit as usize);
        idx += 1;
        if idx >= chrs.len() {
            break;
        }
    }
    (cur, idx)
}

fn parse_schematic(lines: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = vec![];
    let mut symbols = vec![];
    for (row_idx, row) in lines.split_terminator("\n").enumerate() {
        let mut col = 0;
        let chrs: Vec<char> = row.chars().collect();
        while col < row.len() {
            let ch = chrs[col];
            if ch.is_digit(10) {
                let old_col = col;
                let (num, new_col) = parse_number(&chrs, col);
                numbers.push(Number {
                    value: num,
                    row: row_idx,
                    start_idx: old_col,
                    end_idx: new_col - 1,
                });
                col = new_col;
            } else if ch != '.' {
                symbols.push(Symbol {
                    row: row_idx,
                    col,
                    symbol: ch,
                });
                col += 1;
            } else {
                col += 1;
            }
        }
    }

    (numbers, symbols)
}

fn day_03_gear_ratios(input_fpath: &PathBuf) -> (usize, usize) {
    let lines = std::fs::read_to_string(input_fpath)
        .expect(format!("Read input from {:?}", input_fpath).as_str());

    let (numbers, symbols) = parse_schematic(&lines);
    // println!("{:?}", numbers);
    // println!("{:?}", symbols);
    // Naive quadratic search: We can make this more efficient by only searching neighboring row symbols.
    let valid_codes: Vec<usize> = numbers
        .iter()
        .filter(|n| symbols.iter().any(|s| n.borders(s.row, s.col)))
        .map(|n| n.value)
        .collect();
    // println!("{:?}", valid_codes);
    let part_one_sol = valid_codes.iter().sum();

    let gear_ids = symbols
        .iter()
        .filter(|s| s.symbol == '*')
        .map(|s| {
            numbers
                .iter()
                .filter(|n| n.borders(s.row, s.col))
                .map(|n| *n)
                .collect::<Vec<Number>>()
        })
        .filter(|matched_numbers| matched_numbers.len() == 2)
        .map(|matched_numbers| matched_numbers[0].value * matched_numbers[1].value)
        .sum();

    (part_one_sol, gear_ids)
}

fn main() {
    println!(
        "{:?}",
        day_03_gear_ratios(&PathBuf::from("input/03-demo.txt"))
    );
    println!("{:?}", day_03_gear_ratios(&PathBuf::from("input/03.txt")));
}
