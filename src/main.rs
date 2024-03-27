mod cube;
mod game;
mod move_cube;

fn main() {
    let board = game::generate_startpos();
    let mut index_matrix = game::generate_index_matrix(board);
}
