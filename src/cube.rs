use crate::game::Board;
use crate::game::InfoMatrix;
use crate::legality_check::{check_legality};
use crate::libcube::{calculate_position, change_direction, get_top, place_cube};

pub type Cube = [[i32; 4]; 2];
pub type MoveArray = [i32; 4];

    /* Example usage:

    //This is turning the cube away from you starting with one on the top and two facing towards you and noting down the values
    let forward_ring: [i32; 4] = [1, 2, 6, 5];

    //This is turning the cube to the right starting with one on the top and two facing towards you and noting down the values
    let side_ring: [i32; 4] = [1, 4, 6, 3];

    //Defining a cube
    let mut cube: [[i32; 4]; 2] = [forward_ring, side_ring];

    //A positive value here indicates turning the cube either to the right or towards you
    let shift: i32 = 5;
    let is_sw: bool = true;

    //Shifting the target axis to the left at a positive value
    cube = roll(shift, is_sw, cube);

    display_cube(cube);
    */

pub fn roll (shift: i32, is_sw: usize, ring_matrix: &mut Cube) {
    let actual_shift = shift % 4;
    if actual_shift != 0 {
        if shift < 0 {
            ring_matrix[is_sw].rotate_right(-actual_shift as usize);
        } else {
            ring_matrix[is_sw].rotate_left(actual_shift as usize);
        }

        let other_axis = if is_sw == 0 { 1 } else { 0 };
        ring_matrix[other_axis][0] = ring_matrix[is_sw][0];
        ring_matrix[other_axis][2] = ring_matrix[is_sw][2];
    }
}

pub fn calculate_rolling (move_array: &MoveArray, info_matrix: &InfoMatrix, cube: &mut Cube) -> Cube {
    let [mut cube_id, forward_fields, turn_direction, mut is_sw] = move_array;
    let mut forward_direction = forward_fields.signum();
    let available_moves = get_top(cube);
    roll(*forward_fields, is_sw as usize, cube);
    (is_sw, forward_direction) = change_direction(&turn_direction, &is_sw, &forward_direction);

    let remaining_fields = (available_moves - forward_fields.signum()) * forward_direction;
    roll(remaining_fields, is_sw as usize, cube);
    return *cube;
}

// A positive forward_fields indicates a movement towards white (bottom)
// Turn_direction 0 indicates not to turn at all, while 1 turns the cube to the right of where it's going
// This continues clockwise

// abs_direction_units work like turn_directions, but the ff are always positive

//Returns is_illegal
pub fn make_move(
    board: &mut Board,
    info_matrix: &mut InfoMatrix,
    is_white_player: &bool,
    move_array: &MoveArray,
) -> bool {
    if check_legality(&board, &info_matrix, &is_white_player, &move_array) == true {
        return true;
    }
    let [mut cube_id, forward_fields, turn_direction, mut is_sw] = move_array;

    let old_position: [usize; 2] = [info_matrix[cube_id as usize][0] as usize, info_matrix[cube_id as usize][1] as usize];
    let new_position: [i32; 2] = calculate_position(&board, &info_matrix, &move_array);
    let new_cube = calculate_rolling(move_array, info_matrix, &mut board[old_position[0]][old_position[1]]);
    place_cube(board, info_matrix, &cube_id, &old_position, &new_position, &new_cube);

    return false;
}
