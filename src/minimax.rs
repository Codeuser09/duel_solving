use std::collections::HashSet;
use crate::cube::{make_move, MoveArray};
use crate::evaluation::{evaluate_position, is_won};
use crate::game::{Board, InfoMatrix};
use crate::libcube::get_top;

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
    let mut best_move = [0, 0, 0, 0];

    let possible_turn_directions = [1, 3];
    let possible_is_sw = [0, 1];
    let mut is_board_dupe: HashSet<Board> = HashSet::new();

    for (cube_id, cube) in info_matrix.iter().enumerate() {
        if cube[3] != is_white as i32 { continue; }
        let cube_position = [cube[0] as usize, cube[1] as usize];
        let available_moves = get_top(&board[cube_position[0]][cube_position[1]]);
        let mut possible_forward_fields = vec![];
        for i in 1..=available_moves {
            possible_forward_fields.push(i);
            if i != 0 {
                possible_forward_fields.push(-i);
            }
        }
        for possible_turn_direction in possible_turn_directions {
            for possible_forward_field in &possible_forward_fields {
                for is_sw in possible_is_sw {
                    let possible_move = [
                        cube_id as i32,
                        *possible_forward_field,
                        possible_turn_direction,
                        is_sw,
                    ];
                    let mut new_board = board.clone();
                    let mut new_info_matrix = info_matrix.clone();
                    if !make_move(&mut new_board, &mut new_info_matrix, &is_white, &possible_move) && is_board_dupe.insert(new_board) {
                        let eval = minimax(
                            &new_board,
                            &new_info_matrix,
                            depth - 1,
                            alpha,
                            beta,
                            !is_white,
                            is_st_move_gen,
                        ).1;

                        if eval > best_eval && is_white || eval < best_eval && !is_white {
                            best_eval = eval;
                            best_move = possible_move;
                        }
                        if is_white {
                            alpha = alpha.max(eval);
                        } else {
                            beta = beta.min(eval);
                        }
                        if beta <= alpha {
                            return (best_move, best_eval);
                        }
                    }
                }
            }
        }
    }
    return (best_move, best_eval);
}
