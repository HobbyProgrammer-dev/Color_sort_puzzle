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

use rand::SeedableRng;
use std::env;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// fn main() {
//     let mut args = env::args();
//     args.next();
//     match args.next() {
//         None => println!("No arguments found!"),
//         Some(arg) => check_args(&arg, & mut args),
//     }
//     println!("{:#?}", args.collect::<Vec<_>>());
// }
fn main() {
    gen_puzz_mult();
}

fn check_args(arg: &str, args: & mut impl Iterator<Item = String>) {
    match arg {
        "-h" | "--help" => help(),
        _ => println!("Invalid argumen! Try --help for command list.")
    }
}

fn help() {
    println!("help will be added...")
}

fn gen_puzz_mult() {
    let no_bottles = 40;
    let height = 4;
    let color_size = 6;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);
    // let mut rng = rand::rng();
    let mut wrtr = csv::Writer::from_path("./data/games.csv").expect("No games.csv");
    for _x in 0..100{
        let _g: game::GameMemEff<u32> = creator::generate(no_bottles, height, color_size, &mut rng, &mut wrtr);
        println!("{_x}");
        wrtr.flush().expect("Error flushing value");
    }
}
