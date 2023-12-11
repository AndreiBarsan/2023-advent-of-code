use std::path::PathBuf;


struct Universe {
    galaxies: Vec<(usize, usize)>
}

impl Universe {
    fn get_max_size(&self) -> (usize, usize) {
        let max_rows = self.galaxies.iter().map(|coord| coord.0).max().expect("Non-empty galaxy");
        let max_cols = self.galaxies.iter().map(|coord| coord.1).max().expect("Non-empty galaxy");
        (max_rows, max_cols)
    }

    fn find_expansions(&self) -> (Vec<usize>, Vec<usize>) {
        let (max_rows, max_cols) = self.get_max_size();
        let mut expanded_rows = Vec::new();
        let mut expanded_cols = Vec::new();

        for rr in 0..max_rows {
            let n_gal_in_row = self.galaxies.iter().filter(|g| g.0 == rr).count();
            if n_gal_in_row == 0 {
                expanded_rows.push(rr);
            }
        }
        for cc in 0..max_cols {
            let n_gal_in_col = self.galaxies.iter().filter(|g| g.1 == cc).count();
            if n_gal_in_col == 0 {
                expanded_cols.push(cc);
            }
        }

        (expanded_rows, expanded_cols)
    }

    fn expand(&self, ex_rows: &[usize], ex_cols: &[usize], factor: usize) -> Universe {
        let mut new_galaxies = self.galaxies.clone();
        for row in ex_rows {
            for (g_old, g_new) in self.galaxies.iter().zip(new_galaxies.iter_mut()) {
                if g_old.0 > *row {
                    g_new.0 += factor;
                }
            }
        }
        for col in ex_cols {
            for (g_old, g_new) in self.galaxies.iter().zip(new_galaxies.iter_mut()) {
                if g_old.1 > *col {
                    g_new.1 += factor;
                }
            }
        }
        Universe { galaxies: new_galaxies }
    }

    fn all_pairwise_distances(&self) -> Vec<usize> {
        let mut distances = Vec::new();
        // C-style, baby! (There's probably also a pretty Rust approach, though.)
        for i in 0..self.galaxies.len() - 1 {
            for j in i + 1..self.galaxies.len() {
                let gi = self.galaxies[i];
                let gj = self.galaxies[j];
                let manhattan_distance = gi.0.abs_diff(gj.0) + gi.1.abs_diff(gj.1);
                // println!("{}, {}, {}", i, j, manhattan_distance);
                distances.push(manhattan_distance)
            }
        }
        distances
    }

}

fn parse_galaxies(input: (usize, &str)) -> Vec<(usize, usize)> {
    let (row, data) = input;
    data.chars().enumerate().filter(|(_, ch)| *ch == '#').map(|(col, _)| (row, col)).collect()
}

fn day_11_cosmic_expansion(input_fpath: &PathBuf) -> (usize, usize) {
    let in_txt = std::fs::read_to_string(input_fpath)
        .unwrap_or_else(|_| panic!("Read input from {:?}", input_fpath));
    let galaxies: Vec<(usize, usize)> = in_txt.split_terminator('\n').enumerate().flat_map(parse_galaxies).collect();
    let univ = Universe { galaxies };
    let (ex_rows, ex_cols) = univ.find_expansions();
    println!("Expanding {} rows and {} columns.", ex_rows.len(), ex_cols.len());

    // Factor = Actual target multiplier - 1. In Part 1, we double each empty row and col.
    let univ_v1 = univ.expand(&ex_rows, &ex_cols, 2 - 1);
    let univ_v2 = univ.expand(&ex_rows, &ex_cols, 1_000_000 - 1);

    let part_one_answer: usize = univ_v1.all_pairwise_distances().iter().sum();
    let part_two_answer: usize = univ_v2.all_pairwise_distances().iter().sum();

    (part_one_answer, part_two_answer)
}

fn main() {
    println!(
        "{:?}",
        day_11_cosmic_expansion(&PathBuf::from("input/11-demo.txt"))
    );
    // too high: 790_195_502_522
    println!(
        "{:?}",
        day_11_cosmic_expansion(&PathBuf::from("input/11.txt"))
    );
}
