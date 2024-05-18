use crate::cube::MoveArray;
use crate::game::{Board, InfoMatrix};
use crate::legal_move_iteration::{discard_legal_moves, get_possible_moves};
use crate::libcube::get_top;
use crate::minimax::is_interesting;

fn cube_amount_evaluation(info_matrix: &InfoMatrix) -> f64 {
    let mut w_cube_amount = 0f64;
    let mut b_cube_amount = 0f64;

    //
    for cube in info_matrix {
        if cube[3] == 1 {
            w_cube_amount += 1f64;
        } else {
            b_cube_amount += 1f64;
        }
    }
    return w_cube_amount - b_cube_amount;
}

pub fn is_won(info_matrix: &InfoMatrix) -> i32 {
    let mut is_w_king = false;
    let mut is_b_king = false;
    for cube in info_matrix {
        if cube[2] == 1 {
            if cube[3] == 1 {
                if cube[0..2] == [0, 4] {
                    return 1;
                }
                is_w_king = true;
            }
            if cube[3] == 0 {
                if cube[0..2] == [7, 4] {
                    return -1;
                }
                is_b_king = true;
            }
        }
    }
    if !is_w_king {
        return -1;
    }
    if !is_b_king {
        return 1;
    }
    return 0;
}

pub fn get_distance(vec1: [i32; 2], vec2: [i32; 2]) -> f64 {
    let difference: [f64; 2] = [
        vec1[0] as f64 - vec2[0] as f64,
        vec1[1] as f64 - vec2[1] as f64,
    ];
    return difference[0].abs() + difference[1].abs();
}

pub fn winning_square_distance(info_matrix: &InfoMatrix) -> f64 {
    let w_winning_square = [0, 4];
    let b_winning_square = [7, 4];

    let mut w_distance = 7.0f64;
    let mut b_distance = 7.0f64;

    for cube in info_matrix {
        if cube[2] == 1 {
            if cube[3] == 1 {
                let w_king_square = [cube[0], cube[1]];
                w_distance = get_distance(w_king_square, w_winning_square);
            } else {
                let b_king_square = [cube[0], cube[1]];
                b_distance = get_distance(b_king_square, b_winning_square);
            }
        }
    }
    // println!("w_distance: {}, b_distance: {}, Winning square distance eval: {}", w_distance, b_distance, w_distance - b_distance);
    let winning_square_distance = b_distance - w_distance;
    return winning_square_distance;
    //Inverted because a smaller distance is good
}

pub fn top_value_total(board: &Board, info_matrix: &InfoMatrix) -> f64 {
    let mut top_value_eval = 0f64;

    for cube in info_matrix {
        let cube_pos_x: usize = cube[0] as usize;
        let cube_pos_y: usize = cube[1] as usize;
        if cube[3] == 1 {
            top_value_eval += get_top(&board[cube_pos_x][cube_pos_y]) as f64;
        } else {
            top_value_eval -= get_top(&board[cube_pos_x][cube_pos_y]) as f64;
        }
    }
    return top_value_eval;
}
pub fn king_distance(info_matrix: &InfoMatrix) -> (f64, f64) {
    let mut distance_to_your_king = 0f64;
    let mut distance_to_enemy_king = 0f64;
    let mut w_king_pos = [0, 4];
    let mut b_king_pos = [7, 4];

    for cube in info_matrix {
        if cube[2] == 1 {
            if cube[3] == 1 {
                w_king_pos = [cube[0], cube[1]];
            } else {
                b_king_pos = [cube[0], cube[1]];
            }
        }
    }

    for cube in info_matrix {
        if cube[3] == 1 {
            distance_to_enemy_king -= get_distance([cube[0], cube[1]], b_king_pos).abs();
            distance_to_your_king -= get_distance([cube[0], cube[1]], w_king_pos).abs();
        } else {
            distance_to_enemy_king += get_distance([cube[0], cube[1]], w_king_pos).abs();
            distance_to_your_king += get_distance([cube[0], cube[1]], b_king_pos).abs();
        }
    }
    return (distance_to_your_king, distance_to_enemy_king);
}

fn interesting_move_amount(
    board: &Board,
    info_matrix: &InfoMatrix,
    w_legal_moves: &Vec<MoveArray>,
    b_legal_moves: &Vec<MoveArray>,
) -> f64 {
    let mut interesting_move_total = 0f64;

    for legal_move in w_legal_moves {
        if is_interesting(&board, &info_matrix, &legal_move, true) == true {
            interesting_move_total += 1f64;
        }
    }
    for legal_move in b_legal_moves {
        if is_interesting(&board, &info_matrix, &legal_move, false) == true {
            interesting_move_total -= 1f64;
        }
    }
    return interesting_move_total;
}

pub fn evaluate_position(
    board: &Board,
    info_matrix: &InfoMatrix,
    cube_amount_weight: f64,
    winning_square_weight: f64,
    legal_move_weight: f64,
    top_value_weight: f64,
    distance_to_own_king_weight: f64,
    distance_to_enemy_king_weight: f64,
    interesting_move_weight: f64,
) -> f64 {
    let mut evaluation = 0f64;

    let mut w_legal_moves = get_possible_moves(&board, &info_matrix, true);
    let mut b_legal_moves = get_possible_moves(&board, &info_matrix, false);
    discard_legal_moves(&board, &info_matrix, &mut w_legal_moves, &true);
    discard_legal_moves(&board, &info_matrix, &mut b_legal_moves, &false);

    let (distance_to_your_king, distance_to_enemy_king) = king_distance(&info_matrix);

    evaluation += cube_amount_evaluation(info_matrix) * cube_amount_weight;
    evaluation += winning_square_distance(&info_matrix) * winning_square_weight;
    evaluation += w_legal_moves.len() as f64 - b_legal_moves.len() as f64 * legal_move_weight;
    evaluation += (top_value_total(&board, &info_matrix)) * top_value_weight;
    evaluation += distance_to_your_king * distance_to_own_king_weight;
    evaluation += distance_to_enemy_king * distance_to_enemy_king_weight;
    evaluation += interesting_move_amount(&board, &info_matrix, &w_legal_moves, &b_legal_moves)
        * interesting_move_weight;

    return evaluation;
}
