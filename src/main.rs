use std::mem::size_of;

use sudoku_rl::grid::Grid;

fn main() {
    // 69175316 iterations
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
    let s = g.brute_force();
    println!("{} {}", s, now.elapsed().as_millis());

    println!("{}", g);
}
