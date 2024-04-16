use crate::game::Board;
use crate::game::InfoMatrix;
use crate::legality_check::{is_illegal_move, is_oob};

pub type Cube = [[i32; 4]; 2];
pub type MoveArray = (i32, i32, i32);

fn get_index(index: i32) -> usize {
    let index_wrapped: i32 = index % 4;
    let index_final: usize = if index_wrapped >= 0 {
        index_wrapped as usize
    } else {
        (index_wrapped + 4) as usize
    };
    index_final
}

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

pub fn display_cube(cube_matrix: &[[i32; 4]; 2]) {
    for axis in cube_matrix {
        print!("[");
        for element in axis {
            print!("{element}");
        }
        print!("]");
    }
}

pub fn get_top(cube_matrix: &[[i32; 4]; 2]) -> i32 {
    cube_matrix[0][0]
}

fn get_abs_direction_unit(is_sw: &i32, forward_direction: &i32) -> i32 {
    if *is_sw == 1 && *forward_direction == 1 {
        return 3;
    }
    if *is_sw == 0 && *forward_direction == -1 {
        return 2;
    }
    return if *is_sw == 1 && *forward_direction == -1 {
        1
    } else {
        0
    };
}

fn get_direction(direction_unit: &i32) -> (i32, i32) {
    // Returns is_sw, forward_dir
    if *direction_unit == 1 {
        return (1, -1);
    }
    if *direction_unit == 2 {
        return (0, -1);
    }
    return if *direction_unit == 3 { (1, 1) } else { (0, 1) };
}

fn change_direction(turn_direction: &i32, is_sw: &i32, forward_direction: &i32) -> (i32, i32) {
    //Returns (is_sw, forward_direction)
    // 0 = no change
    let mut abs_direction_unit = get_abs_direction_unit(&is_sw, &forward_direction);

    abs_direction_unit = (abs_direction_unit + *turn_direction) % 4;

    let new_is_sw = get_direction(&abs_direction_unit).0;
    let new_forward_direction = get_direction(&abs_direction_unit).1;

    return (new_is_sw, new_forward_direction);
}

fn get_smallest_unit(number: &i32) -> i32 {
    if *number < 0 {
        return -1;
    }
    return if *number > 0 { 1 } else { 0 };
}

// A positive forward_fields indicates a movement towards white (bottom)
// Turn_direction 0 indicates not to turn at all, while 1 turns the cube to the right of where it's going
// This continues clockwise

// abs_direction_units work like turn_directions, but the ff are always positive

fn place_cube(
    board: &mut Board,
    info_matrix: &mut InfoMatrix,
    cube_id: &mut i32,
    original_position: [usize; 2],
    new_position: &mut [i32; 2],
    new_cube: &mut Cube,
) {
    //Setting up our original cube that will be placed on the board

    //Setting zero cube for convenience
    let zero_cube = [[0; 4]; 2];

    // Actually placing new cube
    board[new_position[0] as usize][new_position[1] as usize] = *new_cube;
    board[original_position[0]][original_position[1]] = zero_cube;

    info_matrix[*cube_id as usize][0] = new_position[0];
    info_matrix[*cube_id as usize][1] = new_position[1];
}

fn roll_after_dir_change(
    is_sw: &mut i32,
    forward_fields: &mut i32,
    available_moves: i32,
    mut new_cube: Cube,
    forward_direction: i32,
) -> Cube {
    if *is_sw == 1 {
        new_cube = roll(
            -(available_moves - forward_fields.abs()) * forward_direction,
            true,
            new_cube,
        );
    } else {
        new_cube = roll(
            (available_moves - forward_fields.abs()) * forward_direction,
            false,
            new_cube,
        );
    }
    return new_cube;
}

fn roll_before_dir_change(
    is_sw: &mut i32,
    forward_fields: &mut i32,
    turn_direction: &mut i32,
    available_moves: i32,
    original_cube: [[i32; 4]; 2],
    forward_direction: i32,
) -> Cube {
    let new_cube;
    if *turn_direction != 0 {
        if *is_sw == 1 {
            new_cube = roll(-*forward_fields, *is_sw != 0, original_cube);
        } else {
            new_cube = roll(*forward_fields, *is_sw != 0, original_cube);
        }
    } else {
        if *is_sw == 1 {
            new_cube = roll(
                -available_moves * forward_direction,
                *is_sw != 0,
                original_cube,
            );
        } else {
            new_cube = roll(
                available_moves * forward_direction,
                *is_sw != 0,
                original_cube,
            );
        }
    }
    return new_cube;
}
//Exit code 1 here means that the move is illegal, while exit code 0 means that it was legal and the board was successfully changed
pub fn make_move(
    board: &mut Board,
    info_matrix: &mut InfoMatrix,
    is_white_player: &bool,
    move_array: &mut MoveArray,
) -> i32 {
    let (cube_id, forward_fields, turn_direction) = move_array;

    let original_position = [
        info_matrix[*cube_id as usize][0] as usize,
        info_matrix[*cube_id as usize][1] as usize,
    ];

    let mut new_position: [i32; 2] = [original_position[0] as i32, original_position[1] as i32];

    let available_moves: i32 = get_top(&board[original_position[0]][original_position[1]]);
    let original_cube = board[original_position[0]][original_position[1]].clone();
    let mut forward_direction = get_smallest_unit(&forward_fields);
    let is_white_cube = info_matrix[*cube_id as usize][3];
    let mut is_sw = 0;
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

        let is_illegal_move = is_illegal_move(
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

    place_cube(
        board,
        info_matrix,
        cube_id,
        original_position,
        &mut new_position,
        &mut new_cube,
    );
    return 0;
}
