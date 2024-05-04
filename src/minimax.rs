use crate::cube::{make_move, MoveArray};
use crate::game::{Board, InfoMatrix};
use crate::evaluation::{evaluate_position, is_won};
use crate::legal_move_iteration::get_legal_moves;
use rayon::prelude::*;

pub fn minimax(
    board: &Board,
    info_matrix: &InfoMatrix,
    depth: i32,
    alpha: f64,
    beta: f64,
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

    let (best_move, best_eval) = get_legal_moves(&board, &info_matrix, is_white)
        .into_par_iter() // Parallel iterator
        .map(|legal_move| {
            let mut new_board = board.clone();
            let mut new_info_matrix = info_matrix.clone();
            make_move(&mut new_board, &mut new_info_matrix, &is_white, &legal_move);
            let eval =
                minimax(&new_board, &new_info_matrix, depth - 1, alpha, beta, !is_white).1;

            (legal_move, eval)
        })
        .reduce(
            || ([0, 0, 0, 0], if is_white {f64::NEG_INFINITY} else {f64::INFINITY}),
            |(best_move1, best_eval1), (best_move2, best_eval2)| {
                if best_eval1 > best_eval2 && is_white || best_eval1 < best_eval2 && !is_white{
                    (best_move1, best_eval1)
                } else {
                    (best_move2, best_eval2)
                }
            },
        );

    (best_move, best_eval)
}