// A collection of common AoC helpers.

use anyhow::Context;
use itertools::Itertools;
use std::path::PathBuf;

pub fn render(chars: &[Vec<char>]) -> String {
    chars.iter().map(|row| row.iter().collect::<String>()).join("\n")
}

pub fn concat_nums(nums: &[usize]) -> usize {
    let num_str = nums.iter().map(|n| n.to_string()).join("");
    num_str
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("Cannot parse to usize: {}", num_str))
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

/// Reads a file, assumed to hold files of identical lengths, into a vector of vectors.
pub fn read_to_char_grid(input_fpath: &PathBuf) -> Vec<Vec<char>> {
    let in_txt = std::fs::read_to_string(input_fpath).unwrap_or_else(|_| panic!("Read input from {:?}", input_fpath));
    let rows = in_txt.split_terminator('\n');
    rows.map(|s| s.chars().collect()).collect()
}

pub fn parse_color_hex(spec: &str) -> anyhow::Result<(u8, u8, u8)> {
    let safe_chars: &str = spec.trim_start_matches("#");
    let r = u8::from_str_radix(&safe_chars[0..2], 16).with_context(|| format!("R from {spec}"))?;
    let g = u8::from_str_radix(&safe_chars[2..4], 16).with_context(|| format!("G from {spec}"))?;
    let b = u8::from_str_radix(&safe_chars[4..6], 16).with_context(|| format!("B from {spec}"))?;
    Ok((r, g, b))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_color_hex_nominal() -> anyhow::Result<()> {
        assert_eq!((255, 255, 255), parse_color_hex("#FFFFFF")?);
        assert_eq!((255, 0, 255), parse_color_hex("#FF00FF")?);
        assert_eq!((255, 0, 255), parse_color_hex("FF00FF")?);
        assert_eq!((0, 0, 0), parse_color_hex("000000")?);
        assert_eq!((202, 161, 115), parse_color_hex("caa173")?);
        Ok(())
    }

    #[test]
    fn test_parse_color_hex_error() {
        assert!(parse_color_hex("GG1212").is_err());
    }
}
