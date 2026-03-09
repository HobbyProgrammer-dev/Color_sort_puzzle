
use std::fs::File;
use std::iter;

use csv::Writer;
use game::GameMemEff;
use game::GameRules;
use game::UnsignedInt;
use rand::Rng;
use rand::RngExt;
use serde::Serialize;

pub fn generate<Random: Rng, T: UnsignedInt>(
    no_bottles: usize,
    height: u8,
    color_size: u8,
    rng: &mut Random,
    wrtr: &mut Writer<File>,
) -> GameMemEff<T> {
    let mut v = Vec::new();
    for _i in 0..no_bottles {
        v.push(Vec::new());
    }
    let mut available: Vec<_> = iter::repeat_n(height, no_bottles).collect();
    for _i in 0..(no_bottles*height as usize) {
        let mut marble_placed = false;
        while ! marble_placed {
            let pos = rng.random_range(0..no_bottles);
            let marble = rng.random_range(1..=no_bottles);
            if v[pos].len() == height.into() {
                continue;
            }
            if available[marble - 1] == 0 {
                continue;
            }
            v[pos].push(marble as u8);
            available[marble - 1] -= 1;
            marble_placed = true;
        }
    }
    v.push(Vec::new());
    let mut is_solvable = false;
    let mut no_empty = 1;
    let mut path_length = 0;
    while ! is_solvable {
        let mut g = GameMemEff::<T>::new(height, &v, color_size);
        let path = sorter::dfs::dfs(&mut g, false);
        if let Some(path_unwrapped) = path {
            is_solvable = true;
            path_length = path_unwrapped.get_depth();
        } else {
            v.push(Vec::new());
            no_empty += 1;
        }
    }

    let game_solvable = GameMemEff::new(height, &v, color_size);
    let val = LogCsv{ no_of_filled_bottles: no_bottles, no_of_bottles_added: no_empty, path_length };
    wrtr.serialize(val).expect("Error writing value");

    game_solvable
}

#[derive(Debug, Serialize)]
struct LogCsv {
    no_of_filled_bottles: usize,
    no_of_bottles_added: usize,
    path_length: usize,
}
