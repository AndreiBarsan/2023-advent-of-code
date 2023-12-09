use std::path::PathBuf;

fn parse_seq(line: &str) -> Vec<i64> {
    line.split(" ")
        .map(|n_str| {
            i64::from_str_radix(n_str, 10).unwrap_or_else(|_| panic!("Invalid number: {}", n_str))
        })
        .collect()
}

fn derivative(seq: &Vec<i64>) -> Vec<i64> {
    seq.windows(2).map(|w| w[1] - w[0]).collect()
}

fn day_09_mirage_maintenance(input_fpath: &PathBuf) -> (i64, i64) {
    let in_txt = std::fs::read_to_string(input_fpath)
        .expect(format!("Read input from {:?}", input_fpath).as_str());
    let sequences: Vec<Vec<i64>> = in_txt.split_terminator('\n').map(parse_seq).collect();

    let mut predictions = vec![];
    let mut predictions_f = vec![];
    for seq in sequences {
        let mut derivatives: Vec<Vec<i64>> = Vec::new();
        derivatives.push(seq);

        loop {
            let last = derivatives.last().unwrap();
            let all_zeros = last
                .iter()
                .filter(|x| **x != 0i64)
                .collect::<Vec<&i64>>()
                .len()
                == 0;

            if all_zeros {
                break;
            }

            let new_der = derivative(last);
            derivatives.push(new_der);
        }

        let prediction: i64 = derivatives.iter().map(|seq| seq.last().expect("")).sum();
        predictions.push(prediction);

        let mut prediction_front: i64 = 0;
        for der in derivatives.iter().rev() {
            prediction_front = der.first().expect("") - prediction_front;
        }
        predictions_f.push(prediction_front);
    }

    let part_one_answer: i64 = predictions.iter().sum();
    let part_two_answer: i64 = predictions_f.iter().sum();

    (part_one_answer, part_two_answer)
}

fn main() {
    println!(
        "{:?}",
        day_09_mirage_maintenance(&PathBuf::from("input/09-demo.txt"))
    );
    println!(
        "{:?}",
        day_09_mirage_maintenance(&PathBuf::from("input/09.txt"))
    );
}
