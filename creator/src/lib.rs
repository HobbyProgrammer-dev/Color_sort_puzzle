
use std::fs::File;
use std::iter;

use csv::Writer;
use game::Game;
use game::GameRules;
use rand::Rng;
use rand::RngExt;
use serde::Serialize;

pub fn generate<Random: Rng>(
    no_bottles: usize,
    height: u16,
    rng: &mut Random,
    wrtr: &mut Writer<File>,
) -> Game {
    let mut v = Vec::new();
    for _i in 0..no_bottles {
        v.push(Vec::new());
    }
    let mut available: Vec<_> = iter::repeat(height).take(no_bottles).collect();
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
        let mut g = Game::new(height, &v);
        let path = sorter::dfs::dfs(&mut g, false);
        if path.is_some() {
            is_solvable = true;
            let path_unwrapped = path.unwrap();
            path_length = path_unwrapped.get_depth();
        } else {
            v.push(Vec::new());
            no_empty += 1;
        }
    }

    let game_solvable = Game::new(height, &v);
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
