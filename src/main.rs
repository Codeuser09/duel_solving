use evaluation::evaluate_position;
use game::Board;
use game::InfoMatrix;
use game::{generate_info_matrix, generate_startpos};
use testing::play_sample_game;
use testing::print_legal_moves;
use crate::cube::make_move;
use crate::game::display_info;
use crate::legal_move_iteration::filter_duplicates;

mod cube;
mod evaluation;
mod game;
mod legal_move_iteration;
mod legality_check;
mod libcube;
mod testing;

fn main() {
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);

    play_sample_game(&mut board, &mut info_matrix, 3);
    // println!("{}", evaluate_position(&board, &info_matrix));
    // println!();
    // println!("Evaluation after these moves: {}", evaluation::evaluate_position(&mut board, &mut info_matrix));
    // display_info(&board, &info_matrix);
    print_legal_moves(board, info_matrix, &false);
    // filter_duplicates(&mut vec![[9, -1, 1, 0], [9, -1, 1, 0]], &board, &info_matrix, &true);
}
