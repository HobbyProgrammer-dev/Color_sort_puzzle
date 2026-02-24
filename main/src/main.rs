// use std::rc::Rc;

// use game::Game;
// use rand::SeedableRng;


// fn main() {
//     let v = game::read_csv::read("data/games.csv");
//     let mut g = Game::new(
//         4,
//         v
//     );
//     let path = sorter::bfs::bfs(&mut g, true);
//     if path.is_none() {
//         println!("No solution found!");
//     } else {
//         Rc::new(path.unwrap()).print();
//     }
// }

fn main() {
    // let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let mut rng = rand::rng();
    let mut wrtr = csv::Writer::from_path("./data/games.csv").expect("No games.csv");
    for _x in 0..50{
        let _g = creator::generate(40, 4, &mut rng, &mut wrtr);
        println!("{_x}");
    }
    wrtr.flush().expect("Error flushing value");
    // dbg!(g);
}
