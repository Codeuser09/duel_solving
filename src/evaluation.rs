use log::info;
use crate::game::{Board, InfoMatrix};
use crate::legal_move_iteration::get_legal_moves;

fn cube_amount_evaluation (info_matrix: &InfoMatrix) -> i32 {
    let mut w_cube_amount = 0;
    let mut b_cube_amount = 0;

    //
    for cube in info_matrix {
        if cube[3] == 1 {
            w_cube_amount += 1;
        }
        else {
            b_cube_amount += 1;
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

fn get_distance (vec1: [i32; 2], vec2: [i32; 2]) -> f32 {
    let difference = [vec1[0] - vec2[0], vec1[1] - vec2[1]];
    let squared_added_difference: f32 = difference[0].pow(2) as f32 + difference[1].pow(2) as f32;
    return squared_added_difference.sqrt();
}

fn winning_square_distance (info_matrix: &InfoMatrix) -> f32 {
    let w_winning_square = [0, 4] ;
    let b_winning_square = [7, 4];

    let mut w_distance: f32= 7.0;
    let mut b_distance: f32= 7.0;

        for (i, cube) in info_matrix.iter().enumerate() {
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
    println!("w_distance: {}, b_distance: {}, Winning square distance eval: {}", w_distance, b_distance, w_distance - b_distance);
    return w_distance - b_distance;
}

pub fn evaluate_position (board: &Board, info_matrix: &InfoMatrix) -> i32 {

    // The evaluation is always from white's perspective, so a positive score indicates a white advantage

    let mut evaluation = 0;
    evaluation += cube_amount_evaluation(info_matrix);
    evaluation += is_won(&info_matrix) * 1000000000;
    evaluation = evaluation + get_legal_moves(&board, &info_matrix, true).len() as i32 - get_legal_moves(&board, &info_matrix, false).len() as i32;
    println!("White legal moves: {}, black legal moves: {}, Legal move eval: {}",  get_legal_moves(&board, &info_matrix, true).len() as i32, get_legal_moves(&board, &info_matrix, false).len() as i32, get_legal_moves(&board, &info_matrix, true).len() as i32 - get_legal_moves(&board, &info_matrix, false).len() as i32);
    evaluation += winning_square_distance(&info_matrix) as i32;

    return evaluation
}