use std::collections::{HashSet, VecDeque};

use crate::game::{self, GameState};


pub fn bfs(g: &mut game::Game, print: bool) -> Path {
    let mut queue = VecDeque::new();
    let mut visited_states = HashSet::new();
    visited_states.insert(GameState::new(g));
    queue.push_back((g.clone(), Path::new()));

    let mut depth = 0;
    let mut states = 0;
    let mut prev_states = 0;
    
    while let Some((g, pth)) = queue.pop_front() {
        states += 1;
        let curr_depth = pth.pth.len();

        if curr_depth != depth {
            depth = curr_depth;
            if print {
                for _ in 0..100 {
                    print!("\x08 \x08");
                }
                print!("Current Depth {depth}, Total states {states}, States last depth {}, length of queue {}",states - prev_states, queue.len())
            }
            prev_states = states;
        }
        for mv in g.get_all_valid_moves() {
            let mut g_cpy = g.clone();
            let _ = g_cpy.make_move(&mv);
            let new_state = GameState::new(&g_cpy);
            if visited_states.contains(&new_state) {
                continue;
            }
            let new_path = pth.add(mv);
            if g_cpy.is_win() {
                return new_path;
            }
            visited_states.insert(new_state);
            queue.push_back((g_cpy, new_path));
        }
    }
    Path { pth: vec![] }
}

pub struct Path {
    pth: Vec<game::Move>
}

impl Path {
    fn new() -> Self {
        let v = Vec::new();
        Self { pth: v }
    }
    fn add(&self, mv: game::Move) -> Self {
        let mut v = self.pth.clone();
        v.push(mv);
        Self { pth: v }
    }
    pub fn print(&self) {
        for mv in &self.pth {
            println!("{}",mv.str())
        }
    }
}

