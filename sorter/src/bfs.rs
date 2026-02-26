use std::{collections::{HashSet, VecDeque}, rc::Rc};

use game::{GameRules, Path, UnsignedInt};


pub fn bfs<T: UnsignedInt, G: GameRules<T> + Clone>(g: &mut G, print: bool) -> Option<Path> {
    let mut queue = VecDeque::new();
    let mut visited_states = HashSet::new();
    visited_states.insert(g.get_gamestate());
    queue.push_back((g.clone(), Rc::new(Path::new())));

    let mut depth = 0;
    let mut states = 0;
    let mut prev_states = 0;
    
    while let Some((g, pth)) = queue.pop_front() {
        states += 1;
        let curr_depth = pth.get_depth();

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
            let new_state = g_cpy.get_gamestate();
            if visited_states.contains(&new_state) {
                continue;
            }
            let new_path = pth.add(mv);
            if g_cpy.is_win() {
                return Some(new_path);
            }
            visited_states.insert(new_state);
            queue.push_back((g_cpy, Rc::new(new_path)));
        }
    }
    None
}

