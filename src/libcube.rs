use crate::cube::{Cube, MoveArray, roll};
use crate::game::{Board, InfoMatrix};

pub fn display_move_array(move_array: &MoveArray) {
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

pub fn roll_after_dir_change(
    is_sw: &i32,
    forward_fields: &i32,
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
    is_sw: &i32,
    forward_fields: &i32,
    turn_direction: &i32,
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

pub fn display_ids(info_matrix: &InfoMatrix, is_white: bool) {
    let mut pseudo_board = [[100; 9];8];
    for (i, cube) in info_matrix.iter().enumerate() {
        if cube[3] == is_white as i32 {
            pseudo_board[cube[0] as usize][cube[1] as usize] = i;
        }
    }
    println!();
    println!("IDs of your cubes:");
    for row in pseudo_board {
        print!("[");
        for element in row {
            if element == 100 {
                print!("■■,");
            }
            if element <= 9 {
                print!("0{},", element);
            }
            if element != 100 && element > 9 {
                print!("{},", element);
            }
        }
        print!("]");
        println!();
    }
}