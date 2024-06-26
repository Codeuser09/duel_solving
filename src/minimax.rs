use crate::cube::{make_move, MoveArray};
use crate::evaluation::{evaluate_position, get_distance, is_won};
use crate::game::{Board, InfoMatrix};
use crate::libcube::{calculate_position, get_top};
use std::collections::HashSet;

pub fn is_interesting(
    board: &Board,
    info_matrix: &InfoMatrix,
    move_array: &MoveArray,
    is_white: bool,
) -> bool {
    let cube_id = move_array[0];
    let old_position = [
        info_matrix[cube_id as usize][0],
        info_matrix[cube_id as usize][1],
    ];
    let new_position = calculate_position(&board, &info_matrix, move_array);
    for cube in info_matrix {
        if new_position == [cube[0], cube[1]] && cube[3] != is_white as i32 {
            return true;
        }
    }
    let winning_square = if is_white { [0, 4] } else { [7, 4] };
    if info_matrix[cube_id as usize][2] == 1
        && get_distance(winning_square, new_position) < get_distance(winning_square, old_position)
    {
        return true;
    }
    return false;
}

pub fn minimax(
    board: &Board,
    info_matrix: &InfoMatrix,
    depth: i32,
    start_depth: i32,
    mut alpha: f64,
    mut beta: f64,
    is_white: bool,
    cube_amount_weight: f64,
    winning_square_weight: f64,
    legal_move_weight: f64,
    top_value_weight: f64,
    distance_to_own_king_weight: f64,
    distance_to_enemy_king_weight: f64,
    interesting_move_weight: f64,
) -> (MoveArray, f64) {
    let is_game_won = is_won(&info_matrix);
    if depth == 0 || is_game_won != 0 {
        if is_game_won == 1 {
            return ([50, 50, 50, 50], f64::INFINITY);
        }
        if is_game_won == -1 {
            return ([50, 50, 50, 50], f64::NEG_INFINITY);
        }
        if depth == 0 {
            return (
                [50, 50, 50, 50],
                evaluate_position(
                    &board,
                    &info_matrix,
                    cube_amount_weight,
                    winning_square_weight,
                    legal_move_weight,
                    top_value_weight,
                    distance_to_own_king_weight,
                    distance_to_enemy_king_weight,
                    interesting_move_weight,
                ),
            );
        }
    }

    let mut best_eval = if is_white {
        -f64::INFINITY
    } else {
        f64::INFINITY
    };
    let mut best_move = [100, 100, 100, 100];

    let possible_turn_directions = [1, 3];
    let possible_is_sw = [0, 1];
    let mut is_board_dupe: HashSet<Board> = HashSet::new();
    let consider_uninteresting = if depth <= start_depth / 2 {
        false
    } else {
        true
    };
    let mut eval;
    let mut bot_move;

    for (cube_id, cube) in info_matrix.iter().enumerate() {
        if cube[3] != is_white as i32 {
            continue;
        }
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
                    if (consider_uninteresting
                        || is_interesting(&board, &info_matrix, &possible_move, is_white))
                        && !make_move(
                            &mut new_board,
                            &mut new_info_matrix,
                            &is_white,
                            &possible_move,
                        )
                        && is_board_dupe.insert(new_board)
                    {
                        (bot_move, eval) = minimax(
                            &new_board,
                            &new_info_matrix,
                            depth - 1,
                            start_depth,
                            alpha,
                            beta,
                            !is_white,
                            cube_amount_weight,
                            winning_square_weight,
                            legal_move_weight,
                            top_value_weight,
                            distance_to_own_king_weight,
                            distance_to_enemy_king_weight,
                            interesting_move_weight,
                        );
                        if bot_move == [100, 100, 100, 100] {
                            eval = evaluate_position(
                                &new_board,
                                &new_info_matrix,
                                cube_amount_weight,
                                winning_square_weight,
                                legal_move_weight,
                                top_value_weight,
                                distance_to_own_king_weight,
                                distance_to_enemy_king_weight,
                                interesting_move_weight,
                            );
                        }
                        if eval == f64::INFINITY && is_white
                            || eval == f64::NEG_INFINITY && !is_white
                        {
                            return (possible_move, eval);
                        }
                        if eval > best_eval && is_white || eval < best_eval && !is_white {
                            best_eval = eval;
                            best_move = possible_move;
                        }

                        if is_white {
                            alpha = alpha.max(eval);
                        } else {
                            beta = beta.min(eval);
                        }

                        if beta <= alpha
                            || eval == f64::INFINITY && is_white
                            || eval == f64::NEG_INFINITY && !is_white
                        {
                            return (best_move, best_eval);
                        }
                    }
                }
            }
        }
    }
    return (best_move, best_eval);
}
