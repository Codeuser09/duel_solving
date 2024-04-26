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
) -> i32 {
    for legality_check_loop_id in 0..info_matrix.len() {
        let cube = info_matrix[legality_check_loop_id];

        if cube[0] == new_position[0] && cube[1] == new_position[1] {
            println!("Cube at this field");
            if *cube_id == available_moves - 1 || *cube_id == -available_moves + 1 {
                if *is_white_cube == cube[3] {
                    println!("Cube at this field is the same color");
                    return 1;
                } else {
                    println!("Removed cube from info matrix");
                    info_matrix.retain(|x| *x != cube);
                    return 0;
                }
            } else {
                println!("Not at maximum fields, cannot take cube");
                return 1;
            }
        }
        if *is_white_cube != *is_white_player as i32 {
            println!("Cannot move cube of the opponent");
            return 1;
        }
    }
    return 0;
}

pub fn is_oob(new_position: &[i32; 2], is_sw: &i32, forward_direction: &i32, forward_fields: &i32, available_moves: &i32, turn_direction: &i32) -> i32 {
    let maximum_value = get_maximum_value(is_sw);
    if new_position[*is_sw as usize] + forward_direction > maximum_value || new_position[*is_sw as usize] + forward_direction < 0 {
        println!("Out of bounds, forward_direction: {forward_direction}, is_sideways: {is_sw}");
        return 1;
    }
    if forward_fields.abs() >= *available_moves {
        println!("Too many forward fields or too little forward fields, ff: {}, am: {}", forward_fields, available_moves);
        return 1;
    }

    return 0;
}

pub fn is_legal_operation (move_array: &MoveArray) -> i32 {
    let [cube_id, mut forward_fields, turn_direction, mut is_sw] = move_array;
    if *turn_direction == 2 && forward_fields != 0 || *turn_direction > 3 || *turn_direction < 0 {
        println!("Illegal turn direction");
        return 1;
    }
    if *turn_direction == 2 && is_sw == 1 {
        println!("Illegal combo");
        return 1;
    }
    return 0;
}