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

pub fn roll(shift: i32, is_sideways: bool, original_matrix: Cube) -> Cube {
    /* Example usage:

    //This is turning the cube away from you starting with one on the top and two facing towards you and noting down the values
    let forward_ring: [i32; 4] = [1, 2, 6, 5];

    //This is turning the cube to the right starting with one on the top and two facing towards you and noting down the values
    let side_ring: [i32; 4] = [1, 4, 6, 3];

    //Defining a cube
    let mut cube: [[i32; 4]; 2] = [forward_ring, side_ring];

    //A positive value here indicates turning the cube either to the right or away from you
    let shift: i32 = -5;
    let is_sideways: bool = true;

    //Shifting the target axis to the left at a positive value
    cube = roll(shift, is_sideways, cube);

    display_cube(cube);
    */

    let is_sideways_: usize = is_sideways as usize;
    let mut ring_matrix: Cube = original_matrix.clone();
    let matrix_copy: Cube = ring_matrix.clone();

    if shift % 4 != 0 {
        for i in 0..ring_matrix[is_sideways_].len() {
            ring_matrix[is_sideways_][i] =
                matrix_copy[is_sideways as usize][get_index(i as i32 + shift)];
        }
        let other_axis = !is_sideways as usize;
        ring_matrix[other_axis][0] = ring_matrix[is_sideways_][0];
        ring_matrix[other_axis][2] = ring_matrix[is_sideways_][2];
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

fn change_direction(turn_direction: &i32, forward_fields: &i32) -> (i32, i32) {
    if *turn_direction == 0 {
        return (0, *forward_fields);
    }
    if *turn_direction == 1 {
        return (1, -*forward_fields);
    }
    if *turn_direction == 2 {
        return (0, *forward_fields);
    }
    if *turn_direction == 3 {
        return (1, -*forward_fields);
    } else {
        return (0, 0);
    }
}

fn get_smallest_unit(number: &i32) -> i32 {
    if *number < 0 {
        return number / (-1 * number);
    }
    if *number > 0 {
        return number / number;
    } else {
        return 0;
    }
}

pub fn make_move(board: &mut Board, info_matrix: &mut InfoMatrix, move_array: &MoveArray) {
    // move_array = [cube_id, is_forward, forward_fields, turn_direction]
    // Do you start moving forwards,
    // how many fields do you want to go forwards (or backwards) before turning,
    // in which direction do you turn after you went the forward fields (0 - 3)(fw, right, bw., left)
    //
    // info_matrix = [x_array, y_array, king_array, is_white_array]

    let (cube_id, is_sideways, forward_fields, turn_direction) = move_array;

    let is_bw = get_smallest_unit(forward_fields);

    let original_position = [
        info_matrix[*cube_id as usize][0] as usize,
        info_matrix[*cube_id as usize][1] as usize,
    ];

    let mut is_sideways = *is_sideways;
    let mut forward_fields = *forward_fields;

    let mut new_position = original_position.clone();

    let available_moves: i32 = get_top(&board[original_position[0]][original_position[1]]);

    let original_cube = board[original_position[0]][original_position[1]].clone();
    let new_cube = roll(forward_fields, is_sideways != 0, original_cube);

    /* working on sideways functionality
    let is_sideways_temp = change_direction(&turn_direction, &mut forward_fields).0;
    let forward_fields_temp =
        forward_fields - change_direction(&turn_direction, &mut forward_fields).1;

    if *turn_direction != 0 {
        new_cube = roll(
            available_moves - forward_fields,
            is_sideways_temp != 0,
            new_cube,
        );
    }
    */

    for i in 0..available_moves {
        //Info printing
        println!();

        //For changing direction if needed
        if *turn_direction != 0 && i == forward_fields {
            is_sideways = change_direction(&turn_direction, &mut forward_fields).0;
            forward_fields = change_direction(&turn_direction, &mut forward_fields).1;
        }

        //More info
        print!(
            "Current index: {}, is_sideways: {}, is_backwards: {}, forward: {}, available moves: {}",
            i,
            is_sideways,
            is_bw,
            get_smallest_unit(&forward_fields),
            available_moves
        );
        println!();

        let position_change = is_bw as usize * get_smallest_unit(&forward_fields) as usize;

        //Setting up the new position
        if is_legal_move() == true {
            new_position[is_sideways as usize] += position_change;
        }
    }

    //Setting up our original cube that will be placed on the board

    //Setting zero cube for convenience
    let zero_cube = [[0; 4]; 2];

    //Info
    for element in &new_position {
        println!("new position: {}", element);
    }
    display_cube(&new_cube);

    //TODO: Implement rolling for the cube
    // Actually placing new cube
    board[new_position[0]][new_position[1]] = new_cube;
    board[original_position[0]][original_position[1]] = zero_cube;

    info_matrix[*cube_id as usize][0] = new_position[0] as i32;
    info_matrix[*cube_id as usize][1] = new_position[1] as i32;
}
