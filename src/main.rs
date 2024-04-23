use cube::MoveArray;
use game::Board;
use game::InfoMatrix;
use crate::evaluation::evaluate_position;
use crate::game::display_info;
use crate::testing::play_sample_game;

mod cube;
mod game;
mod legality_check;
mod legal_move_iteration;
mod libcube;
mod evaluation;
mod testing;

fn main() {
    let mut board: Board = game::generate_startpos();
    let mut info_matrix: InfoMatrix = game::generate_info_matrix(board);

    play_sample_game(&mut board, &mut info_matrix, 4);
    println!("{}", evaluate_position(&board, &info_matrix));
    // println!();
    // println!("Evaluation after these moves: {}", evaluation::evaluate_position(&mut board, &mut info_matrix));
    // display_info(&board, &info_matrix);
}
