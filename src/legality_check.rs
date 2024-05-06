use crate::cube::MoveArray;
use crate::game::InfoMatrix;

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
    is_white_player: &bool
) -> (bool, bool, usize) { //Returns (Is_illegal, removed_cube, removed_cube_position)
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
        if *is_white_cube != *is_white_player as i32 {
            return (true, false, 0);
        }
    }
    return (false, false, 0);
}

pub fn is_oob(new_position: &[i32; 2], is_sw: &i32, forward_direction: &i32, forward_fields: &i32, available_moves: &i32) -> bool {
    let maximum_value = get_maximum_value(is_sw);
    if new_position[*is_sw as usize] + forward_direction > maximum_value || new_position[*is_sw as usize] + forward_direction < 0 {
        // println!("Out of bounds, forward_direction: {forward_direction}, is_sideways: {is_sw}");
        return true;
    }
    if forward_fields.abs() >= *available_moves {
        // println!("Too many forward fields or too little forward fields, ff: {}, am: {}", forward_fields, available_moves);
        return true;
    }
    return false;
}

pub fn is_illegal_operation (move_array: &MoveArray) -> bool {
    let [_cube_id, forward_fields, turn_direction, is_sw] = move_array;
    if *turn_direction == 2 && *forward_fields != 0 || *turn_direction > 3 || *turn_direction < 0 {
        // println!("Illegal turn direction");
        return true;
    }
    if *turn_direction == 2 && *is_sw == 1 {
        // println!("Illegal combo");
        return true;
    }
    return false;
}