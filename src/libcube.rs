use crate::cube::{Cube, MoveArray, roll};
use crate::game::{Board, InfoMatrix};



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
    return get_direction(&abs_direction_unit);
}

pub fn place_cube(
    board: &mut Board,
    info_matrix: &mut InfoMatrix,
    cube_id: &i32,
    original_position: &[usize; 2],
    new_position: &[i32; 2],
    new_cube: &Cube,
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

pub fn count_cubes(board: &Board) -> i32 {
    let mut counter = 0;
    for row in board {
        for cube in row {
            if *cube != [[0; 4];2] {
                counter += 1;
            }
        }
    }
    return counter
}

pub fn get_top(cube_matrix: &[[i32; 4]; 2]) -> i32 {
    cube_matrix[0][0]
}


pub fn calculate_position (board: &Board, info_matrix: &InfoMatrix, legal_move: &MoveArray) -> [i32; 2]{
    let [cube_id, forward_fields, turn_direction, is_sw] = legal_move;
    let forward_direction = forward_fields.signum();
    let mut new_position = [info_matrix[*cube_id as usize][0], info_matrix[*cube_id as usize][1]];
    let available_moves = get_top(&board[new_position[0] as usize][new_position[1] as usize]);
    new_position[*is_sw as usize] += forward_fields;
    let (is_sw, forward_direction) = change_direction(&turn_direction, &is_sw, &forward_direction);
    println!("Is sideways: {is_sw}");

    let available_moves = (available_moves.abs() - forward_fields.abs()) * forward_direction;
    new_position[is_sw as usize] += available_moves;

    return new_position;
}
