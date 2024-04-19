use crate::cube::{MoveArray};
use crate::libcube::get_top;
use crate::game::{Board, InfoMatrix};

// move_array = [cube_id, forward_fields, turn_direction]

pub fn get_legal_moves (board: &Board, info_matrix: &InfoMatrix) -> Vec<MoveArray> {
    let mut legal_moves = vec![];
    let mut possible_turn_directions = [0, 1, 2, 3];

    for (cube_id, cube) in info_matrix.iter().enumerate() {
        let cube_position = [cube[0] as usize, cube[1] as usize];
        let available_moves = get_top(&board[cube_position[0]][cube_position[1]]);
        let mut possible_forward_fields = vec![];
        for i in 0..available_moves {
            &possible_forward_fields.push(i);
        }
        for possible_turn_direction in possible_turn_directions {
            for possible_forward_field in &possible_forward_fields {
                legal_moves.push([cube_id as i32, *possible_forward_field, possible_turn_direction])
            }
        }
    }
    return legal_moves;
}