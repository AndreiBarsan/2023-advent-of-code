use std::path::PathBuf;

struct Round {
    n_red: usize,
    n_green: usize,
    n_blue: usize,
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

fn parse_round(chunk: &str) -> Round {
    let mut n_red = 0usize;
    let mut n_green = 0usize;
    let mut n_blue = 0usize;

    let subs: Vec<&str> = chunk.split(",").map(|chunk| chunk.trim()).collect();
    for sub in subs {
        let name_val: Vec<&str> = sub.split(" ").collect();
        let name = name_val[1];
        let val = usize::from_str_radix(name_val[0], 10).expect("");

        if name == "red" {
            n_red = val;
        } else if name == "green" {
            n_green = val;
        } else if name == "blue" {
            n_blue = val;
        } else {
            panic!("Unexpected color: {}", name);
        }
    }

    return Round {
        n_red,
        n_green,
        n_blue,
    };
}

fn parse_game_line(line: &str) -> Game {
    let coarse_chunks: Vec<&str> = line
        .split_terminator(":")
        .map(|chunk| chunk.trim())
        .collect();
    let meta_chunk = coarse_chunks[0];
    let game_id = usize::from_str_radix(meta_chunk.split(" ").collect::<Vec<&str>>()[1], 10)
        .expect("parse game ID");

    let rounds: Vec<Round> = coarse_chunks[1]
        .split_terminator(";")
        .map(parse_round)
        .collect();
    Game {
        id: game_id,
        rounds,
    }
}

fn is_valid_game_part_one(game: &Game) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    game.rounds
        .iter()
        .all(|r| r.n_red <= max_red && r.n_green <= max_green && r.n_blue <= max_blue)
}

/// The power score of a game is the minimum possible R * G * B given the observed rounds.
fn get_game_power(game: &Game) -> usize {
    let mins = game.rounds.iter().fold(
        Round {
            n_red: 0,
            n_green: 0,
            n_blue: 0,
        },
        |r1, r2| Round {
            n_red: r1.n_red.max(r2.n_red),
            n_green: r1.n_green.max(r2.n_green),
            n_blue: r1.n_blue.max(r2.n_blue),
        },
    );

    mins.n_red * mins.n_green * mins.n_blue
}

fn day_02_cube_conundrum(fpath: &PathBuf) -> (usize, usize) {
    let lines =
        std::fs::read_to_string(fpath).expect(format!("Read input from {:?}", fpath).as_str());

    let games: Vec<Game> = lines.split_terminator("\n").map(parse_game_line).collect();
    let valid_games_part_one = games.iter().filter(|g| is_valid_game_part_one(&g));
    // The Part 1 solution is just the sum of the valid game IDs.
    let part_one_code: usize = valid_games_part_one.map(|g| g.id).sum::<usize>();

    // The Part 2 solution is the sum of all games' power scores.
    let game_power_total_part_two = games.iter().map(get_game_power).sum();
    (part_one_code, game_power_total_part_two)
}

fn main() {
    println!(
        "{:?}",
        day_02_cube_conundrum(&PathBuf::from("input/02-demo.txt"))
    );
    println!(
        "{:?}",
        day_02_cube_conundrum(&PathBuf::from("input/02.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_problem() {
        let (part_one, part_two) = day_02_cube_conundrum(&PathBuf::from("input/02.txt"));
        assert_eq!(part_one, 2176);
        assert_eq!(part_two, 63700);
    }
}
