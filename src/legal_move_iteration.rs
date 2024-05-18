use crate::cube::{make_move, MoveArray};
use crate::game::{Board, InfoMatrix};
use crate::libcube::get_top;
use std::collections::HashSet;

pub fn discard_legal_moves(
    board: &Board,
    info_matrix: &InfoMatrix,
    possible_moves: &mut Vec<MoveArray>,
    is_white: &bool,
) {
    let mut i = 0;
    let mut is_dupe = HashSet::new();
    while i < possible_moves.len() {
        let mut board_dupe = board.clone();
        let mut info_dupe = info_matrix.clone();
        if make_move(
            &mut board_dupe,
            &mut info_dupe,
            is_white,
            &possible_moves[i],
        ) == true
            || !is_dupe.insert(board_dupe)
        {
            possible_moves.remove(i);
        } else {
            i += 1;
        }
    }
}

pub fn get_possible_moves(
    board: &Board,
    info_matrix: &InfoMatrix,
    is_white: bool,
) -> Vec<MoveArray> {
    let mut possible_moves = vec![];
    let possible_turn_directions = [1, 3];
    let possible_is_sw = [0, 1];

    for (cube_id, cube) in info_matrix.iter().enumerate() {
        if cube[3] != is_white as i32 {
            continue;
        }
        let cube_position = [cube[0] as usize, cube[1] as usize];
        let available_moves = get_top(&board[cube_position[0]][cube_position[1]]);
        let mut possible_forward_fields = vec![];
        for i in 1..=available_moves {
            possible_forward_fields.push(i);
            if i != 0 {
                possible_forward_fields.push(-i);
            }
        }
        for possible_turn_direction in possible_turn_directions {
            for possible_forward_field in &possible_forward_fields {
                for is_sw in possible_is_sw {
                    let possible_move = [
                        cube_id as i32,
                        *possible_forward_field,
                        possible_turn_direction,
                        is_sw,
                    ];
                    possible_moves.push(possible_move);
                }
            }
        }
    }
    // println!("Possible legal moves: {}", possible_moves.len());
    return possible_moves;
}

pub fn get_possible_boards(
    board: &Board,
    info_matrix: &InfoMatrix,
    is_white: &bool,
    legal_moves: &mut Vec<MoveArray>,
) -> Vec<Board> {
    let mut possible_boards: Vec<Board> = vec![];
    discard_legal_moves(&board, &info_matrix, legal_moves, is_white);
    for i in 0..legal_moves.len() {
        let mut board_clone = board.clone();
        let mut info_matrix_clone = info_matrix.clone();
        make_move(
            &mut board_clone,
            &mut info_matrix_clone,
            &is_white,
            &legal_moves[i],
        );
        possible_boards.push(board_clone);
    }
    return possible_boards;
}
