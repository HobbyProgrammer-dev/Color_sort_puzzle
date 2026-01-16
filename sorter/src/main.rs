use crate::game::Game;

mod read_csv;
mod game;
mod bfs;

fn main() {

    let v = read_csv::read();
    //     vec![
    //     vec![1, 2, 3, 4],
    //     vec![4, 5, 6, 6],
    //     vec![5, 3, 7, 8],
    //     vec![4, 8, 5, 9],
    //     vec![3, 1, 7, 2],
    //     vec![5, 10, 7, 9],
    //     vec![2, 7, 10, 2],
    //     vec![11, 6, 11, 3],
    //     vec![1, 10, 9, 6],
    //     vec![9, 8, 10, 11],
    //     vec![8, 4, 11, 1],
    //     vec![],
    //     vec![],
    //     vec![]
    // ];
    let mut g = Game::new(
        4,
        v
    );
    let path = bfs::bfs(&mut g, true);
    path.print();

}
