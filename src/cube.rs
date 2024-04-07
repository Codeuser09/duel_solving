use crate::game::Board;
use crate::game::InfoMatrix;
use crate::legality_check::is_legal_move;

pub type Cube = [[i32; 4]; 2];
pub type MoveArray = (i32, i32, i32, i32);

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

fn get_direction_unit(is_sw: &i32, is_bw: &i32) -> i32 {
    if *is_sw == 1 && *is_bw == -1 {
        return 1;
    }
    if *is_sw == 1 && *is_bw == 1 {
        return 2;
    }
    if *is_sw == 0 && *is_bw == -1 {
        return 3;
    } else {
        return 0;
    }
}

fn get_direction(direction_unit: &i32) -> (i32, i32) {
    if *direction_unit == 1 {
        return (1, 1);
    }
    if *direction_unit == 2 {
        return (1, -1);
    }
    if *direction_unit == 3 {
        return (1, 1);
    } else {
        return (-1, 1);
    }
}

fn change_direction(turn_direction: &i32, is_sw: &i32, is_bw: &i32) -> (i32, i32) {
    //Returns (is_sw, is_bw)
    // 0 = no change

    if *turn_direction > 2 {
        panic!("Turn direction can not be larger than 2");
    }

    println!();
    println!("Changed direction");
    //
    let mut direction_unit = get_direction_unit(&is_sw, &is_bw);

    println!();
    println!("direction_unit: {}", direction_unit);

    direction_unit = (direction_unit + *turn_direction) % 3;

    println!();
    println!("new direction_unit: {}", direction_unit);

    let new_is_sw = get_direction(&direction_unit).0;
    let new_is_bw = get_direction(&direction_unit).1;

    return (new_is_sw, new_is_bw);
}

fn get_smallest_unit(number: &i32) -> i32 {
    if *number < 0 {
        return -1;
    }
    if *number > 0 {
        return 1;
    } else {
        return 0;
    }
}

pub fn make_move(board: &mut Board, info_matrix: &mut InfoMatrix, move_array: &mut MoveArray) {
    // move_array = [cube_id, is_sw, forward_fields, turn_direction]
    // Do you start moving forwards,
    // how many fields do you want to go forwards (or backwards) before turning,
    // in which direction do you turn after you went the forward fields (0 - 3)(fw, right, bw., left)
    //
    // info_matrix = [x_array, y_array, king_array, is_white_array]

    let (cube_id, is_sw, forward_fields, turn_direction) = move_array;

    let original_position = [
        info_matrix[*cube_id as usize][0] as usize,
        info_matrix[*cube_id as usize][1] as usize,
    ];

    let mut new_position: [i32; 2] =
        [original_position[0] as i32, original_position[1] as i32] as [i32; 2];

    let available_moves: i32 = get_top(&board[original_position[0]][original_position[1]]);

    let original_cube = board[original_position[0]][original_position[1]].clone();

    let mut new_cube;

    if *turn_direction != 0 {
        if *is_sw == 1 {
            new_cube = roll(-*forward_fields, *is_sw != 0, original_cube);
        } else {
            new_cube = roll(*forward_fields, *is_sw != 0, original_cube);
        }
    } else {
        if *is_sw == 1 {
            new_cube = roll(-available_moves, *is_sw != 0, original_cube);
        } else {
            new_cube = roll(available_moves, *is_sw != 0, original_cube);
        }
    }

    let mut is_bw = get_smallest_unit(&forward_fields);

    /* working on sideways functionality
    let is_sw_temp = change_direction(&turn_direction, &mut forward_fields).0;
    let forward_fields_temp =
        forward_fields - change_direction(&turn_direction, &mut forward_fields).1;

    if *turn_direction != 0 {
        new_cube = roll(
            available_moves - forward_fields,
            is_sw_temp != 0,
            new_cube,
        );
    }
    */

    for i in 0..available_moves {
        //Info printing

        //For changing direction if needed
        if *turn_direction != 0 && i == *forward_fields
            || *turn_direction != 0 && i == -*forward_fields
        {
            let new_direction = change_direction(&turn_direction, &is_sw, &is_sw);
            *is_sw = new_direction.0;
            is_bw = new_direction.1;

            if *is_sw == 1 {
                new_cube = roll(
                    -available_moves + *forward_fields,
                    *is_sw != 0,
                    original_cube,
                );
            } else {
                new_cube = roll(
                    available_moves - *forward_fields,
                    *is_sw != 0,
                    original_cube,
                );
            }
        }

        //is_bw = get_smallest_unit(&forward_fields);

        println!();

        //More info
        print!(
            "Current index: {}, is_sideways: {}, is_backwards: {}, available moves: {}, turn_direction: {}",
            i,
            is_sw,
            is_bw,
            available_moves,
            turn_direction
        );
        println!();

        //Setting up the new position
        if is_legal_move() == true {
            new_position[*is_sw as usize] += is_bw;
        }

        println!("New position for our cube is: ");
        for element in &new_position {
            print!("{}", element);
        }
        println!();
    }

    //Setting up our original cube that will be placed on the board

    //Setting zero cube for convenience
    let zero_cube = [[0; 4]; 2];

    //Info
    println!();
    println!();
    print!("The cube is ");
    display_cube(&new_cube);
    print!(" After rolling");
    println!();
    println!();

    //TODO: Implement rolling for the cube
    // Actually placing new cube
    board[new_position[0] as usize][new_position[1] as usize] = new_cube;
    board[original_position[0]][original_position[1]] = zero_cube;

    info_matrix[*cube_id as usize][0] = new_position[0] as i32;
    info_matrix[*cube_id as usize][1] = new_position[1] as i32;
}
