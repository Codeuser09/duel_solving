use crate::cube::{make_move, MoveArray};
use crate::evaluation::{evaluate_position, is_won};
use crate::game::{Board, InfoMatrix};
use crate::legal_move_iteration::{get_possible_moves};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};

pub fn _mt_map_minimax(
    board: &Board,
    info_matrix: &InfoMatrix,
    depth: i32,
    alpha: f64,
    beta: f64,
    is_white: bool,
    is_st_move_gen: bool,
) -> (MoveArray, f64) {
    let is_game_won = is_won(&info_matrix);
    if depth == 0 || is_game_won != 0 {
        if is_game_won == 1 {
            return ([0, 0, 0, 0], f64::INFINITY);
        }
        if is_game_won == -1 {
            return ([0, 0, 0, 0], f64::NEG_INFINITY);
        }
        if depth == 0 {
            return ([0, 0, 0, 0], evaluate_position(&board, &info_matrix));
        }
    }

    let mut shared_alpha = Arc::new(Mutex::new(alpha));
    let mut shared_beta = Arc::new(Mutex::new(beta));

    let (best_move, best_eval) = if is_st_move_gen {
        get_possible_moves(&board, &info_matrix, is_white)
    } else {
        get_possible_moves(&board, &info_matrix, is_white)
    }
    .into_par_iter() // Parallel iterator
    .map(|legal_move| {
        let mut new_board = board.clone();
        let mut new_info_matrix = info_matrix.clone();
        make_move(&mut new_board, &mut new_info_matrix, &is_white, &legal_move);
        let eval = _mt_map_minimax(
            &new_board,
            &new_info_matrix,
            depth - 1,
            alpha,
            beta,
            !is_white,
            is_st_move_gen,
        )
        .1;

        let mut alpha = shared_alpha.lock().unwrap();
        let mut beta = shared_beta.lock().unwrap();

        if is_white {
            *alpha = (*alpha).max(eval);
        } else {
            *beta = (*beta).min(eval);
        }

        if *beta <= *alpha {
            return (legal_move, eval);
        }

        (legal_move, eval)
    })
    .reduce(
        || {
            (
                [0, 0, 0, 0],
                if is_white {
                    f64::NEG_INFINITY
                } else {
                    f64::INFINITY
                },
            )
        },
        |(best_move1, best_eval1), (best_move2, best_eval2)| {
            if best_eval1 > best_eval2 {
                (best_move1, best_eval1)
            } else {
                (best_move2, best_eval2)
            }
        },
    );

    (best_move, best_eval)
}

pub fn _mt_minimax_par_iter(
    board: &Board,
    info_matrix: &InfoMatrix,
    depth: i32,
    mut alpha: f64,
    mut beta: f64,
    is_white: bool,
    is_st_move_gen: bool,
) -> (MoveArray, f64) {
    let is_game_won = is_won(&info_matrix);
    if depth == 0 || is_game_won != 0 {
        if is_game_won == 1 {
            return ([0, 0, 0, 0], f64::INFINITY);
        }
        if is_game_won == -1 {
            return ([0, 0, 0, 0], f64::NEG_INFINITY);
        }
        if depth == 0 {
            return ([0, 0, 0, 0], evaluate_position(&board, &info_matrix));
        }
    }
    let best_eval = if is_white {
        f64::NEG_INFINITY
    } else {
        f64::INFINITY
    };
    let best_move = get_possible_moves(&board, &info_matrix, is_white)[0];
    let mut best_move_shared = Arc::new(Mutex::new(best_move));
    let mut best_eval_shared = Arc::new(Mutex::new(best_eval));
    let mut alpha_shared = Arc::new(Mutex::new(alpha));
    let mut beta_shared = Arc::new(Mutex::new(beta));

    if is_st_move_gen {
        get_possible_moves(&board, &info_matrix, is_white)
    } else {
        get_possible_moves(&board, &info_matrix, is_white)
    }
    .into_par_iter()
    .for_each(|legal_move| {
        let mut new_board = board.clone();
        let mut new_info_matrix = info_matrix.clone();
        let mut best_move = *best_move_shared.lock().unwrap();
        let mut best_eval = *best_eval_shared.lock().unwrap();
        let alpha = *alpha_shared.lock().unwrap();
        let beta = *beta_shared.lock().unwrap();

        make_move(&mut new_board, &mut new_info_matrix, &is_white, &legal_move);
        let eval = _mt_minimax_par_iter(
            &new_board,
            &new_info_matrix,
            depth - 1,
            alpha,
            beta,
            !is_white,
            is_st_move_gen,
        )
        .1;

        if eval > best_eval && is_white || eval < best_eval && !is_white {
            best_eval = eval;
            best_move = legal_move;
        }
        if is_white {
            *alpha_shared.lock().unwrap() = alpha.max(eval);
        } else {
            *beta_shared.lock().unwrap() = beta.min(eval);
        }
        if *beta_shared.lock().unwrap() <= *alpha_shared.lock().unwrap() {
            return;
        }
    });
    return (
        *best_move_shared.lock().unwrap(),
        *best_eval_shared.lock().unwrap(),
    );
}

pub fn minimax(
    board: &Board,
    info_matrix: &InfoMatrix,
    depth: i32,
    mut alpha: f64,
    mut beta: f64,
    is_white: bool,
    is_st_move_gen: bool,
) -> (MoveArray, f64) {
    let is_game_won = is_won(&info_matrix);
    if depth == 0 || is_game_won != 0 {
        if is_game_won == 1 {
            return ([0, 0, 0, 0], f64::INFINITY);
        }
        if is_game_won == -1 {
            return ([0, 0, 0, 0], f64::NEG_INFINITY);
        }
        if depth == 0 {
            return ([0, 0, 0, 0], evaluate_position(&board, &info_matrix));
        }
    }

    let mut best_eval = if is_white {
        f64::NEG_INFINITY
    } else {
        f64::INFINITY
    };
    let mut best_move = get_possible_moves(&board, &info_matrix, is_white)[0];

    for legal_move in get_possible_moves(&board, &info_matrix, is_white)
    {
        let mut new_board = board.clone();
        let mut new_info_matrix = info_matrix.clone();
        make_move(&mut new_board, &mut new_info_matrix, &is_white, &legal_move);

        let eval = minimax(
            &new_board,
            &new_info_matrix,
            depth - 1,
            alpha,
            beta,
            !is_white,
            is_st_move_gen,
        )
        .1;

        if eval > best_eval && is_white || eval < best_eval && !is_white {
            best_eval = eval;
            best_move = legal_move;
        }
        if is_white {
            alpha = alpha.max(eval);
        } else {
            beta = beta.min(eval);
        }
        if beta <= alpha {
            break;
        }
    }
    return (best_move, best_eval);
}
