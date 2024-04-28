use crate::cube::{make_move, MoveArray};
use crate::game::{Board, InfoMatrix};
use crate::libcube::{get_top};
use std::collections::HashSet;

// move_array = [cube_id, forward_fields, turn_direction]

fn discard_legal_moves(
    board: &Board,
    info_matrix: &InfoMatrix,
    possible_moves: &mut Vec<MoveArray>,
    is_white: &bool,
) {
    let mut i = 0;
    while i < possible_moves.len() {
        let mut original_board = board.clone();
        let mut original_info_matrix: InfoMatrix = info_matrix.clone();
        if make_move(
            &mut original_board,
            &mut original_info_matrix,
            is_white,
            &possible_moves[i],
        ) == 1
        {
            possible_moves.remove(i);
        }
        else {
            i += 1;
        }
    }
}

pub fn get_legal_moves(board: &Board, info_matrix: &InfoMatrix, is_white: bool) -> Vec<MoveArray> {
    let mut possible_moves = vec![];
    let possible_turn_directions = [0, 1, 2, 3];
    let possible_is_sw = [0, 1];

    for (cube_id, cube) in info_matrix.iter().enumerate() {
        if cube[3] != is_white as i32 {continue;}
        let cube_position = [cube[0] as usize, cube[1] as usize];
        let available_moves = get_top(&board[cube_position[0]][cube_position[1]]);
        let mut possible_forward_fields = vec![];
        for i in 0..available_moves {
            possible_forward_fields.push(i);
            if i != 0 {
                possible_forward_fields.push(-i);
            }
        }
        for possible_turn_direction in possible_turn_directions {
            for possible_forward_field in &possible_forward_fields {
                for is_sw in possible_is_sw {
                    //Be careful with this, as we should probably have duplicates with 0 ff and td
                    possible_moves.push([
                        cube_id as i32,
                        *possible_forward_field,
                        possible_turn_direction,
                        is_sw,
                    ]);
                    // display_move_array(&[cube_id as i32, *possible_forward_field, possible_turn_direction, is_sw]);
                }
            }
        }
    }
    // println!("Possible legal moves: {}", possible_moves.len());
    discard_legal_moves(&board, &info_matrix, &mut possible_moves, &is_white);
    let non_duped_moves = filter_duplicates(possible_moves, &board, &info_matrix, &is_white);
    return non_duped_moves;
}
pub fn filter_duplicates (legal_moves: Vec<MoveArray>, board: &Board, info_matrix: &InfoMatrix, is_white: &bool) -> Vec<MoveArray> {
    let new_board_array: Vec<Board> = get_possible_boards(&board, &info_matrix, &is_white, &legal_moves);

    let mut set = HashSet::new();
    let mut non_duped_moves = Vec::new();

    for (index, item) in new_board_array.iter().enumerate() {
        if set.insert(item.clone()) {
            non_duped_moves.push(legal_moves[index]);
        }
        else {
            // println!("Filtered duplicate")
        }
    }
    return non_duped_moves;
}

pub fn get_possible_boards (board: &Board, info_matrix: &InfoMatrix, is_white: &bool, legal_moves: &Vec<MoveArray>) -> Vec<Board> {
    let mut possible_boards: Vec<Board> = vec![];
    for i in 0..legal_moves.len() {
        let mut board_clone = board.clone();
        let mut info_matrix_clone = info_matrix.clone();
        make_move(&mut board_clone, &mut info_matrix_clone, &is_white, &legal_moves[i]);
        possible_boards.push(board_clone);
    }
    return possible_boards;
}