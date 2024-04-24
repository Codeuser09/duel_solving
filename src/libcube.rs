use crate::cube::{Cube, MoveArray, roll};
use crate::game::{Board, InfoMatrix};

pub fn display_move_array (move_array: &MoveArray) {
    println!();
    print!("[");
    for element in move_array {
        print!("{element},");
    }
    print!("]");
    println!();
}

pub fn get_index(index: i32) -> usize {
    let index_wrapped: i32 = index % 4;
    let index_final: usize = if index_wrapped >= 0 {
        index_wrapped as usize
    } else {
        (index_wrapped + 4) as usize
    };
    index_final
}

pub fn get_abs_direction_unit(is_sw: &i32, forward_direction: &i32) -> i32 {
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

pub fn get_direction(direction_unit: &i32) -> (i32, i32) {
    // Returns is_sw, forward_dir
    if *direction_unit == 1 {
        return (1, -1);
    }
    if *direction_unit == 2 {
        return (0, -1);
    }
    return if *direction_unit == 3 { (1, 1) } else { (0, 1) };
}

pub fn change_direction(turn_direction: &i32, is_sw: &i32, forward_direction: &i32) -> (i32, i32) {
    //Returns (is_sw, forward_direction)
    // 0 = no change
    let mut abs_direction_unit = get_abs_direction_unit(&is_sw, &forward_direction);

    abs_direction_unit = (abs_direction_unit + *turn_direction) % 4;

    let new_is_sw = get_direction(&abs_direction_unit).0;
    let new_forward_direction = get_direction(&abs_direction_unit).1;

    return (new_is_sw, new_forward_direction);
}

pub fn get_smallest_unit(number: &i32) -> i32 {
    if *number < 0 {
        return -1;
    }
    return if *number > 0 { 1 } else { 0 };
}

pub fn place_cube(
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

pub fn roll_after_dir_change(
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

pub fn roll_before_dir_change(
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


pub fn display_cube(cube_matrix: &[[i32; 4]; 2]) {
    for axis in cube_matrix {
        print!("[");
        for element in axis {
            print!("{}", element);
        }
        print!("]");
    }
}

pub fn get_top(cube_matrix: &[[i32; 4]; 2]) -> i32 {
    cube_matrix[0][0]
}