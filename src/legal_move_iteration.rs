use crate::cube::{make_move, MoveArray};
use crate::game::{Board, display_board, display_info, InfoMatrix};
use crate::libcube::{display_move_array, get_top};

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

pub fn get_legal_moves(board: &Board, info_matrix: &InfoMatrix, is_white: &bool) -> Vec<MoveArray> {
    let mut possible_moves = vec![];
    let possible_turn_directions = [1, 2, 3];
    let possible_is_sw = [0, 1];

    for (cube_id, cube) in info_matrix.iter().enumerate() {
        if cube[3] != *is_white as i32 {continue;}
        let cube_position = [cube[0] as usize, cube[1] as usize];
        let available_moves = get_top(&board[cube_position[0]][cube_position[1]]);
        let mut possible_forward_fields = vec![];
        for i in 0..available_moves {
            &possible_forward_fields.push(i);
            if i != 0 {
                &possible_forward_fields.push(-i);
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
                    display_move_array(&[cube_id as i32, *possible_forward_field, possible_turn_direction, is_sw]);
                }
            }
        }
    }
    discard_legal_moves(&board, &info_matrix, &mut possible_moves, &is_white);
    filter_duplicates(&mut possible_moves, &board, &info_matrix, &is_white);
    return possible_moves;
}

pub fn filter_duplicates (move_arrays: &mut Vec<MoveArray>, board: &Board, info_matrix: &InfoMatrix, is_white: &bool) {
    for (index, legal_move) in move_arrays.iter().enumerate() {
        let mut original_board = board.clone();
        let mut original_info_matrix = info_matrix.clone();
        make_move(&mut original_board, &mut original_info_matrix, is_white, legal_move);
        for (sub_index, possible_duplicate) in move_arrays.iter().enumerate() {
            if index == sub_index {
                continue;
            }
            let mut duplicate_board = board.clone();
            let mut duplicate_info_matrix = info_matrix.clone();
            make_move(&mut duplicate_board, &mut duplicate_info_matrix, is_white, possible_duplicate);
            if original_board == duplicate_board && original_info_matrix == duplicate_info_matrix {
                println!();
                println!();
                println!("Found duplicate");
                println!("Original: ");
                display_info(&original_board, &original_info_matrix);
                println!();
                println!("Move array: ");
                display_move_array(legal_move);
                println!();
                println!();
                println!("Duplicate: ");
                display_info(&duplicate_board, &duplicate_info_matrix);
                println!();
                println!("Move array: ");
                display_move_array(possible_duplicate);
                println!();
                println!();
                move_arrays.remove(index);
                return;
            }
        }
    }
    println!("Found no duplicates");
    return;
}