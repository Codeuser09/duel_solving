use crate::cube::{make_move, MoveArray};
use crate::game::{Board, InfoMatrix};
use crate::evaluation::{evaluate_position, is_won};
use crate::legal_move_iteration::get_legal_moves;

pub fn minimax(
    board: &Board,
    info_matrix: &InfoMatrix,
    depth: i32,
    mut alpha: f64,
    mut beta: f64,
    is_white: bool,
) -> (MoveArray, f64) {
    if depth == 0 || is_won(&info_matrix) != 0 {
        if is_won(&info_matrix) == 1 {
            return ([0, 0, 0, 0], 1000000000f64);
        }
        if is_won(&info_matrix) == -1 {
            return ([0, 0, 0, 0], -1000000000f64);
        }
        if depth == 0 {
            return ([0, 0, 0, 0], evaluate_position(&board, &info_matrix));
        }
    }

    return if is_white {
        let mut max_eval = f64::NEG_INFINITY;
        let mut best_move = get_legal_moves(&board, &info_matrix, is_white)[0];

        for legal_move in get_legal_moves(&board, &info_matrix, is_white) {
            let mut new_board = board.clone();
            let mut new_info_matrix = info_matrix.clone();
            make_move(&mut new_board, &mut new_info_matrix, &is_white, &legal_move);
            let eval = minimax(&new_board, &new_info_matrix, depth - 1, alpha, beta, false).1;

            if eval > max_eval {
                max_eval = eval;
                best_move = legal_move;
            }
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        (best_move, max_eval)
    } else {
        let mut min_eval = f64::INFINITY;
        let mut best_move = get_legal_moves(&board, &info_matrix, is_white)[0];

        for legal_move in get_legal_moves(&board, &info_matrix, is_white) {
            let mut new_board = board.clone();
            let mut new_info_matrix = info_matrix.clone();
            make_move(&mut new_board, &mut new_info_matrix, &is_white, &legal_move);
            let eval = minimax(&new_board, &new_info_matrix, depth - 1, alpha, beta, true).1;

            if eval < min_eval {
                min_eval = eval;
                best_move = legal_move;
            }
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        (best_move, min_eval)
    }
}