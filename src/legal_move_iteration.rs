use crate::cube::{make_move, MoveArray};
use crate::game::{Board, InfoMatrix};
use crate::libcube::get_top;

// move_array = [cube_id, forward_fields, turn_direction]

fn discard_legal_moves(
    board: &Board,
    info_matrix: InfoMatrix,
    possible_moves: &mut Vec<MoveArray>,
    is_white: &bool,
) {
    for (i, mut possible_move) in possible_moves.iter_mut().enumerate() {
        let mut original_board = board.clone();
        let mut original_info_matrix: InfoMatrix = info_matrix.clone();
        if make_move(
            &mut original_board,
            &mut original_info_matrix,
            is_white,
            &mut possible_move,
        ) == 1
        {
            possible_moves.remove(i);
        }
    }
}

pub fn get_legal_moves(board: &Board, info_matrix: &InfoMatrix, is_white: &bool) -> Vec<MoveArray> {
    let mut legal_moves = vec![];
    let mut possible_turn_directions = [0, 1, 2, 3];
    let mut possible_is_sw = [0, 1];

    for (cube_id, cube) in info_matrix.iter().enumerate() {
        let cube_position = [cube[0] as usize, cube[1] as usize];
        let available_moves = get_top(&board[cube_position[0]][cube_position[1]]);
        let mut possible_forward_fields = vec![];
        for i in 0..available_moves {
            &possible_forward_fields.push(i);
        }
        for possible_turn_direction in possible_turn_directions {
            for possible_forward_field in &possible_forward_fields {
                for is_sw in possible_is_sw {
                    //Be careful with this, as we should probably have duplicates with 0 ff and td
                    legal_moves.push([
                        cube_id as i32,
                        *possible_forward_field,
                        possible_turn_direction,
                        is_sw,
                    ])
                }
            }
        }
    }
    return legal_moves;
}
