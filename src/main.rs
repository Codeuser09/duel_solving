use std::io::stdin;
use evaluation::evaluate_position;
use game::Board;
use game::InfoMatrix;
use game::{generate_info_matrix, generate_startpos};
use crate::minimax::minimax;
use interaction::play_sample_game;
use interaction::print_legal_moves;
use crate::cube::{make_move, MoveArray};
use crate::game::{display_info, display_info_matrix};
use crate::legal_move_iteration::{filter_duplicates, get_legal_moves};
use crate::evaluation::is_won;

mod cube;
mod evaluation;
mod game;
mod legal_move_iteration;
mod legality_check;
mod libcube;
mod interaction;
mod minimax;

fn main() {
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    // play_sample_game(&mut board, &mut info_matrix, 3);
    // println!("{}", evaluate_position(&board, &info_matrix));
    // println!();
    // println!("Evaluation after these moves: {}", evaluate_position(&mut board, &mut info_matrix));
    // display_info(&board, &info_matrix);
    // print_legal_moves(&mut board, &mut info_matrix, &false);
    // print_legal_moves(&mut board, &mut info_matrix, &true);
    // filter_duplicates(&mut vec![[9, -1, 1, 0], [9, -1, 1, 0]], &board, &info_matrix, &true);
    // let mut move_1 = [1, 0, 0, 0];
    // make_move(&mut board, &mut info_matrix, &false, &move_1);
    // display_info(&board, &info_matrix);

    while is_won(&info_matrix) == 0 {
        if is_white = true {
            let bot_move: MoveArray = minimax(&board, &info_matrix, -1000000000, 1000000000, 100, true)[1];
            make_move(&mut board, &mut info_matrix, &true, &bot_move);
        }
        else {

        }
    }
}
