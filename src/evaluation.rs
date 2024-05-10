use crate::game::{Board, InfoMatrix};
use crate::legal_move_iteration::get_possible_moves;
use crate::libcube::{get_top};
fn cube_amount_evaluation (info_matrix: &InfoMatrix) -> f64 {
    let mut w_cube_amount = 0f64;
    let mut b_cube_amount = 0f64;

    //
    for cube in info_matrix {
        if cube[3] == 1 {
            w_cube_amount += 1f64;
        }
        else {
            b_cube_amount += 1f64;
        }
    }
    return w_cube_amount - b_cube_amount;
}

pub fn is_won (info_matrix: &InfoMatrix) -> i32 {
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

fn get_distance (vec1: [i32; 2], vec2: [i32; 2]) -> f64 {
    let difference : [f64; 2] = [vec1[0] as f64 - vec2[0] as f64, vec1[1] as f64 - vec2[1] as f64];
    let squared_added_difference: f64 = difference[0].powf(2f64) as f64 + difference[1].powf(2f64) as f64;
    return squared_added_difference.sqrt();
}

fn winning_square_distance (info_matrix: &InfoMatrix) -> f64 {
    let w_winning_square = [0, 4] ;
    let b_winning_square = [7, 4];

    let mut w_distance= 7.0f64;
    let mut b_distance= 7.0f64;

        for cube in info_matrix {
        if cube[2] == 1 {
            if cube[3] == 1 {
                let w_king_square = [cube[0], cube[1]];
                w_distance = get_distance(w_king_square, w_winning_square);
            }
            else {
                let b_king_square = [cube[0], cube[1]];
                b_distance = get_distance(b_king_square, b_winning_square);
            }
        }
    }
    // println!("w_distance: {}, b_distance: {}, Winning square distance eval: {}", w_distance, b_distance, w_distance - b_distance);
    let winning_square_distance = b_distance - w_distance;
    return winning_square_distance.powf(winning_square_distance) * winning_square_distance.signum() //Inverted because a smaller distance is good
}

pub fn top_value_total(board: &Board, info_matrix: &InfoMatrix) -> f64 {
    let mut white_total = 0;
    let mut black_total = 0;

    for cube in info_matrix {
        let cube_pos_x: usize = cube[0] as usize;
        let cube_pos_y: usize = cube[1] as usize;
        if cube[3] == 1 {
            white_total += get_top(&board[cube_pos_x][cube_pos_y]);
        }
        else {
            black_total += get_top(&board[cube_pos_x][cube_pos_y]);
        }
    }
    return white_total as f64 - black_total as f64;
}

pub fn legal_move_total (board: &Board, info_matrix: &InfoMatrix) -> f64 {
    return (get_possible_moves(&board, &info_matrix, true).len() as f64 - get_possible_moves(&board, &info_matrix, false).len() as f64) / 100f64;
}

pub fn evaluate_position (board: &Board, info_matrix: &InfoMatrix) -> f64 {

    // The evaluation is always from white's perspective, so a positive score indicates a white advantage

    let mut evaluation= 0f64;
    evaluation += cube_amount_evaluation(info_matrix);
    evaluation += winning_square_distance(&info_matrix);
    evaluation += legal_move_total(&board, &info_matrix);
    // evaluation += top_value_total(&board, &info_matrix);

    return evaluation
}