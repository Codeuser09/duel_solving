use crate::game::Board;
use crate::game::InfoMatrix;
use crate::legality_check::{is_illegal_move, is_illegal_operation, is_oob};
use crate::libcube::{
    change_direction, get_top, place_cube, roll_after_dir_change, roll_before_dir_change,
};

pub type Cube = [[i32; 4]; 2];
pub type MoveArray = [i32; 4];

pub fn roll(shift: i32, is_sw: usize, ring_matrix: &mut Cube) {
    let actual_shift = shift % 4;
    if actual_shift != 0 {
        if shift < 0 {
            ring_matrix[is_sw].rotate_left(-actual_shift as usize);
        } else {
            ring_matrix[is_sw].rotate_right(actual_shift as usize);
        }

        let other_axis = if is_sw == 0 { 1 } else { 0 };
        ring_matrix[other_axis][0] = ring_matrix[is_sw][0];
        ring_matrix[other_axis][2] = ring_matrix[is_sw][2];
    }
}

pub fn make_move(
    board: &mut Board,
    info_matrix: &mut InfoMatrix,
    is_white_player: &bool,
    move_array: &MoveArray,
) -> bool {
    let [mut cube_id, forward_fields, turn_direction, mut is_sw] = move_array;
    if info_matrix[cube_id as usize][3] != *is_white_player as i32 {
        return true;
    }

    let original_position = [
        info_matrix[cube_id as usize][0] as usize,
        info_matrix[cube_id as usize][1] as usize,
    ];

    let mut new_position: [i32; 2] = [original_position[0] as i32, original_position[1] as i32];

    let available_moves: i32 = get_top(&board[original_position[0]][original_position[1]]);
    if is_illegal_operation(&move_array, &available_moves) == true {
        return true;
    }

    let mut new_cube = board[original_position[0]][original_position[1]];
    let mut forward_direction = forward_fields.signum();
    let is_white_cube = info_matrix[cube_id as usize][3];

    let board_before = board.clone();

    roll_before_dir_change(
        &is_sw,
        &forward_fields,
        turn_direction,
        available_moves,
        &mut new_cube,
        forward_direction,
    );

    for i in 0..available_moves {
        if *turn_direction != 0 && i == *forward_fields
            || *turn_direction != 0 && i == -forward_fields
        {
            (is_sw, forward_direction) =
                change_direction(&turn_direction, &is_sw, &forward_direction);
            roll_after_dir_change(
                &is_sw,
                &forward_fields,
                available_moves,
                &mut new_cube,
                forward_direction,
            );
        }

        if is_oob(
            &new_position,
            &is_sw,
            &forward_direction,
            &forward_fields,
            &available_moves,
        ) == true
        {
            return true;
        }

        //Setting up the new position
        new_position[is_sw as usize] += forward_direction;

        let is_illegal = is_illegal_move(
            info_matrix,
            &new_position,
            &available_moves,
            &i,
            &is_white_cube,
        );

        if is_illegal.0 == true {
            return true;
        }

        if is_illegal.1 == true && cube_id > is_illegal.2 as i32 {
            cube_id -= 1;
        }
    }
    place_cube(
        board,
        info_matrix,
        &cube_id,
        &original_position,
        &new_position,
        &new_cube,
    );

    if board_before == *board {
        return true;
    }

    return false;
}
