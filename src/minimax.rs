use crate::cube::{make_move, MoveArray};
use crate::game::{Board, InfoMatrix};
use crate::evaluation::{evaluate_position, is_won};
use crate::legal_move_iteration::get_legal_moves;
use std::cmp::{max, min};

pub fn minimax (board: &Board, info_matrix: &InfoMatrix, depth: i32,mut alpha: i32, mut beta: i32, is_white: bool) -> (MoveArray, i32) {
    if depth == 0 || is_won(&info_matrix) != 0 {
        if is_won(&info_matrix) == 1 {
            return ([0, 0, 0, 0], 1000000000);
        }
        if is_won(&info_matrix) == -1 {
            return ([0, 0, 0, 0], -1000000000);
        }
        if depth == 0 {
            return ([0, 0, 0, 0], evaluate_position(&board, &info_matrix));
        }
    }
    if is_white == true {
        let mut max_eval = -1000000000;
        let mut best_move = get_legal_moves(&board, &info_matrix, is_white)[0];
        for legal_move in get_legal_moves(&board, &info_matrix, is_white) {
            let mut new_board = board.clone();
            let mut new_info_matrix = info_matrix.clone();
            make_move(&mut new_board, &mut new_info_matrix, &is_white, &legal_move);
            let eval = minimax(&new_board, &new_info_matrix, depth - 1, alpha, beta,false).1;
            if eval > max_eval {
                max_eval = eval;
                best_move = legal_move;
            }
            alpha = max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        return (best_move, max_eval);
    } else {
        let mut min_eval = 1000000000;
        let mut best_move = get_legal_moves(&board, &info_matrix, is_white)[0];
        for legal_move in get_legal_moves(&board, &info_matrix, is_white) {
            let mut new_board = board.clone();
            let mut new_info_matrix = info_matrix.clone();
            make_move(&mut new_board, &mut new_info_matrix, &is_white, &legal_move);
            let eval = minimax(&new_board, &new_info_matrix, depth -1, alpha, beta,  true).1;
            if eval < min_eval {
                min_eval = eval;
                best_move = legal_move;
            }
            beta = min(beta, eval);
            if alpha <= beta {
                break;
            }
        }
        return (best_move, min_eval);
    }
}