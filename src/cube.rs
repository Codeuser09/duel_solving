use crate::game::Board;
use crate::game::InfoMatrix;
use crate::legality_check::{is_illegal_move, is_oob};
use crate::libcube::{get_top, get_index, get_smallest_unit, get_direction, get_abs_direction_unit, roll_after_dir_change, roll_before_dir_change, change_direction, place_cube};

pub type Cube = [[i32; 4]; 2];
pub type MoveArray = [i32; 4];



pub fn roll(shift: i32, is_sw: bool, original_matrix: Cube) -> Cube {
    /* Example usage:

    //This is turning the cube away from you starting with one on the top and two facing towards you and noting down the values
    let forward_ring: [i32; 4] = [1, 2, 6, 5];

    //This is turning the cube to the right starting with one on the top and two facing towards you and noting down the values
    let side_ring: [i32; 4] = [1, 4, 6, 3];

    //Defining a cube
    let mut cube: [[i32; 4]; 2] = [forward_ring, side_ring];

    //A positive value here indicates turning the cube either to the left or towards you
    let shift: i32 = 5;
    let is_sw: bool = true;

    //Shifting the target axis to the left at a positive value
    cube = roll(shift, is_sw, cube);

    display_cube(cube);
    */

    let is_sw_: usize = is_sw as usize;
    let mut ring_matrix: Cube = original_matrix.clone();
    let matrix_copy: Cube = ring_matrix.clone();

    if shift % 4 != 0 {
        for i in 0..ring_matrix[is_sw_].len() {
            ring_matrix[is_sw_][i] = matrix_copy[is_sw as usize][get_index(i as i32 - shift)];
        }
        let other_axis = !is_sw as usize;
        ring_matrix[other_axis][0] = ring_matrix[is_sw_][0];
        ring_matrix[other_axis][2] = ring_matrix[is_sw_][2];
    }
    ring_matrix
}


// A positive forward_fields indicates a movement towards white (bottom)
// Turn_direction 0 indicates not to turn at all, while 1 turns the cube to the right of where it's going
// This continues clockwise

// abs_direction_units work like turn_directions, but the ff are always positive

//Exit code 1 here means that the move is illegal, while exit code 0 means that it was legal and the board was successfully changed
pub fn make_move(
    board: &mut Board,
    info_matrix: &mut InfoMatrix,
    is_white_player: &bool,
    move_array: &mut MoveArray,
) -> i32 {
    let [cube_id, forward_fields, turn_direction, mut is_sw] = move_array;

    let original_position = [
        info_matrix[*cube_id as usize][0] as usize,
        info_matrix[*cube_id as usize][1] as usize,
    ];

    let mut new_position: [i32; 2] = [original_position[0] as i32, original_position[1] as i32];

    let available_moves: i32 = get_top(&board[original_position[0]][original_position[1]]);
    let original_cube = board[original_position[0]][original_position[1]].clone();
    let mut forward_direction = get_smallest_unit(&forward_fields);
    let is_white_cube = info_matrix[*cube_id as usize][3];
    if *turn_direction == 0 && *forward_fields == 0 {
        forward_direction = 1;
    }

    let mut new_cube = roll_before_dir_change(
        &mut is_sw,
        forward_fields,
        turn_direction,
        available_moves,
        original_cube,
        forward_direction,
    );

    for i in 0..available_moves {
        if *turn_direction != 0 && i == *forward_fields
            || *turn_direction != 0 && i == -*forward_fields
        {
            let new_direction = change_direction(turn_direction, &mut is_sw, &forward_direction);
            is_sw = new_direction.0;
            forward_direction = new_direction.1;
            new_cube = roll_after_dir_change(
                &mut is_sw,
                forward_fields,
                available_moves,
                new_cube,
                forward_direction,
            );
        }

        let is_cube_oob = is_oob(&new_position, &is_sw, &forward_direction, &forward_fields, &available_moves, &turn_direction);
        if is_cube_oob == 1 {
            return 1;
        }

        //Setting up the new position
        new_position[is_sw as usize] += forward_direction;

        let (is_illegal_move, info_matrix_changed) = is_illegal_move(
            info_matrix,
            &new_position,
            &available_moves,
            &i,
            &is_white_cube,
            is_white_player
        );

        if is_illegal_move == 1 {
            return 1;
        }
    }

    // We need to do this in case cube id is 17 and it's taking a cube in which case it tries to place the cube in an oob spot in the info matrix
    let mut new_cube_id: i32 = *cube_id;

    if *cube_id == info_matrix.len() as i32 {
        new_cube_id -= 1;
    }

    place_cube(
        board,
        info_matrix,
        &mut new_cube_id,
        original_position,
        &mut new_position,
        &mut new_cube,
    );
    return 0;
}
