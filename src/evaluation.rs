use crate::game::{Board, InfoMatrix};

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

fn winning_score (info_matrix: &InfoMatrix) -> i32 {
    let mut is_w_king = false;
    let mut is_b_king = false;
    for cube in info_matrix {
        if cube[2] == 1 {
            if cube[3] == 1 {
                if cube[0..1] == [0, 4] {
                    return 1000000000;
                }
                is_w_king = true;
            }
            if cube[3] == 0 {
                if cube[0..1] == [7, 4] {
                    return -1000000000;
                }
                is_b_king = true;
            }
        }
    }
    if !is_w_king {
        return -1000000000;
    }
    if !is_b_king {
        return 1000000000;
    }
    return 0;
}

pub fn evaluate_position (board: &Board, info_matrix: &InfoMatrix) -> i32 {

    // The evaluation is always from white's perspective, so a positive score indicates a white advantage

    let mut evaluation = 0;
    evaluation += cube_amount_evaluation(info_matrix);
    evaluation += winning_score(&info_matrix);

    return evaluation
}