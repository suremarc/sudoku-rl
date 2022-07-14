use std::mem::size_of;

use sudoku_rl::grid::Grid;

fn main() {
    let mut g: Grid = Grid::new_from_rows([
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 3, 0, 8, 5],
        [0, 0, 1, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 7, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 1, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 0, 0, 0, 7, 3],
        [0, 0, 2, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 9],
    ]);

    println!("{}", size_of::<Grid>());

    let now = std::time::Instant::now();
    let (s, num_iterations) = g.brute_force();
    println!("{} {} {}", s, num_iterations, now.elapsed().as_millis());

    println!("{}", g);
}
