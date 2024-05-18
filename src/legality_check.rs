use crate::cube::MoveArray;
use crate::game::{Board, InfoMatrix};
use crate::libcube::{change_direction, get_top};

fn get_maximum_value(is_sw: &i32) -> i32 {
    if *is_sw == 0 {
        7
    } else {
        8
    }
}
pub fn is_illegal_move(
    info_matrix: &mut InfoMatrix,
    new_position: &[i32; 2],
    available_moves: &i32,
    cube_id: &i32,
    is_white_cube: &i32,
) -> (bool, bool, usize) {
    //Returns (Is_illegal, removed_cube, removed_cube_position)
    for (legality_check_loop_id, cube) in info_matrix.iter().enumerate() {
        if cube[0] == new_position[0] && cube[1] == new_position[1] {
            if *cube_id == available_moves - 1 || *cube_id == -available_moves + 1 {
                if *is_white_cube == cube[3] {
                    return (true, false, 0);
                } else {
                    info_matrix.remove(legality_check_loop_id);
                    return (false, true, legality_check_loop_id);
                }
            } else {
                return (true, false, 0);
            }
        }
    }
    return (false, false, 0);
}

pub fn _is_illegal_move_var(
    info_matrix: &InfoMatrix,
    new_position: &[i32; 2],
    available_moves: &i32,
    cube_id: &i32,
    is_white_cube: &i32,
    is_white_player: &bool,
) -> bool {
    //Returns (Is_illegal)
    for cube in info_matrix {
        if cube[0] == new_position[0] && cube[1] == new_position[1] {
            if *cube_id == available_moves - 1 || *cube_id == -available_moves + 1 {
                if *is_white_cube == cube[3] {
                    return true;
                } else {
                    return false;
                }
            } else {
                return true;
            }
        }
        if *is_white_cube != *is_white_player as i32 {
            return true;
        }
    }
    return false;
}

pub fn is_oob(
    new_position: &[i32; 2],
    is_sw: &i32,
    forward_direction: &i32,
    forward_fields: &i32,
    available_moves: &i32,
) -> bool {
    let maximum_value = get_maximum_value(is_sw);
    if new_position[*is_sw as usize] + forward_direction > maximum_value
        || new_position[*is_sw as usize] + forward_direction < 0
    {
        return true;
    }
    if forward_fields.abs() > *available_moves {
        return true;
    }
    return false;
}

pub fn is_illegal_operation(move_array: &MoveArray, available_moves: &i32) -> bool {
    let [_, forward_fields, turn_direction, _] = move_array;
    if *available_moves == *forward_fields && *turn_direction == 1 {
        return true;
    }
    return false;
}

pub fn _check_legality(
    board: &Board,
    info_matrix: &InfoMatrix,
    is_white_player: &bool,
    move_array: &MoveArray,
) -> bool {
    let [cube_id, forward_fields, turn_direction, mut is_sw] = move_array;

    // display_move_array(move_array);

    let original_position = [
        info_matrix[*cube_id as usize][0] as usize,
        info_matrix[*cube_id as usize][1] as usize,
    ];

    let available_moves: i32 = get_top(&board[original_position[0]][original_position[1]]);
    if is_illegal_operation(&move_array, &available_moves) == true {
        return true;
    }

    let mut new_position: [i32; 2] = [original_position[0] as i32, original_position[1] as i32];

    let mut forward_direction = forward_fields.signum();
    let is_white_cube = info_matrix[*cube_id as usize][3];

    for i in 0..available_moves {
        if *turn_direction != 0 && i == *forward_fields
            || *turn_direction != 0 && i == -forward_fields
        {
            (is_sw, forward_direction) =
                change_direction(&turn_direction, &is_sw, &forward_direction);
        }

        if is_oob(
            &new_position,
            &is_sw,
            &forward_direction,
            &forward_fields,
            &available_moves,
        ) == true
        {}

        //Setting up the new position
        new_position[is_sw as usize] += forward_direction;

        if _is_illegal_move_var(
            info_matrix,
            &new_position,
            &available_moves,
            &i,
            &is_white_cube,
            is_white_player,
        ) == true
        {
            return true;
        }
    }

    return false;
}
