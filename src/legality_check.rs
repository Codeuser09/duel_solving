use crate::game::InfoMatrix;

pub fn is_legal_move(
    info_matrix: &mut InfoMatrix,
    new_position: &[i32; 2],
    available_moves: &i32,
    cube_id: &i32,
    is_white: &i32,
    is_sw: &i32,
) -> i32 {
    for legality_check_loop_id in 0..info_matrix.len() {
        let cube = info_matrix[legality_check_loop_id];
        if cube[0] == new_position[0] && cube[1] == new_position[1] {
            if cube_id == available_moves - 1 || cube_id == -available_moves + 1 {
                if is_white == cube[3] {
                    return 1;
                } else {
                    info_matrix.retain(|x| *x != cube);
                }
            } else {
                return 1;
            }
        }
        let maximum_value = get_maximum_value(is_sw);
        if new_position[*is_sw as usize] + forward_direction > maximum_value {
            return 1;
        }
        if *forward_fields >= available_moves {
            return 1;
        }
    }
}
