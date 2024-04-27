use crate::cube::make_move;
use crate::game::{Board, InfoMatrix};
use crate::evaluation::{evaluate_position, is_won};
use crate::legal_move_iteration::get_legal_moves;
use std::cmp::{max, min};

pub fn minimax (board: &Board, info_matrix: &InfoMatrix, alpha: i32, beta: i32, depth: i32, is_white: bool) -> i32 {
    if depth == 0 || is_won(&info_matrix) != 0 {
        return evaluate_position(&board, &info_matrix);
    }
    return if is_white == true {
        let mut max_eval = -1000000000;
        for legal_move in get_legal_moves(&board, &info_matrix, is_white) {
            let mut new_board = board.clone();
            let mut new_info_matrix = info_matrix.clone();
            make_move(&mut new_board, &mut new_info_matrix, &is_white, &legal_move, );
            let eval = minimax(&new_board, &new_info_matrix, alpha, beta, depth - 1,false);
            max_eval = max(max_eval, eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = 1000000000;
        for legal_move in get_legal_moves(&board, &info_matrix, is_white) {
            let mut new_board = board.clone();
            let mut new_info_matrix = info_matrix.clone();
            make_move(&mut new_board, &mut new_info_matrix, &is_white, &legal_move, );
            let eval = minimax(&new_board, &new_info_matrix, alpha, beta, depth -1, true);
            min_eval = min(min_eval, eval);
            if alpha <= beta {
                break;
            }
        }
        min_eval
    }
}