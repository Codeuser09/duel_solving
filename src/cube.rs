use crate::game::Board;
use crate::game::InfoMatrix;

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

pub fn display_cube(cube_matrix: [[i32; 4]; 2]) {
    for axis in cube_matrix {
        print!("[");
        for element in axis {
            print!("{element}");
        }
        print!("]");
    }
}

pub fn get_top(cube_matrix: [[i32; 4]; 2]) -> i32 {
    cube_matrix[0][0]
}

pub fn make_move(board: &mut Board, index_matrix: &mut InfoMatrix, move_array: MoveArray) {
    // move_array = [cube_id, is_forward, forward_fields, turn_direction]
    // Do you start moving forwards,
    // how many fields do you want to go forwards (or backwards) before turning,
    // in which direction do you turn after you went the forward fields (0 - 3)(fw, right, bw., left)

    let (cube_id, is_sideways, forward_fields, turn_direction) = move_array;

    let current_cube_x = index_matrix[cube_id as usize][0] as usize;
    let current_cube_y = index_matrix[cube_id as usize][1] as usize;

    let available_moves: i32 = get_top(board[current_cube_x][current_cube_y]);

    for i in 0..available_moves {
        if i < forward_fields {
            print!("{i}");
            index_matrix[cube_id as usize][is_sideways as usize] += 1;
        }
    }

    board[current_cube_x][current_cube_y] = [[0; 4]; 2];
}
