// A collection of common AoC helpers.

use itertools::Itertools;

pub fn render(chars: &Vec<Vec<char>>) -> String {
    chars
        .iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n")
}

pub fn concat_nums(nums: &Vec<usize>) -> usize {
    let num_str = nums.iter().map(|n| n.to_string()).into_iter().join("");
    usize::from_str_radix(&num_str, 10)
        .expect(format!("Cannot parse to usize: {}", num_str).as_str())
}

pub fn chunk_lines_by_blank(lines: &Vec<String>) -> Vec<Vec<String>> {
    // TODO(andrei): Consider solving with iterators.
    let mut out = vec![];
    let mut cur = vec![];

    for line in lines {
        if line.is_empty() {
            out.push(cur.clone());
            cur = vec![];
        } else {
            cur.push(line.clone());
        }
    }
    out.push(cur.clone());

    out
}
