use crate::cube::{make_move, MoveArray};
use crate::game::{Board, InfoMatrix};
use crate::libcube::{get_top};
use crate::legality_check::check_legality;


fn discard_legal_moves(
    board: &Board,
    info_matrix: &InfoMatrix,
    possible_moves: &mut Vec<MoveArray>,
    is_white: &bool,
) {
    let mut i = 0;
    while i < possible_moves.len() {
        if check_legality(
            &board,
            &info_matrix,
            is_white,
            &possible_moves[i],
        ) == true {
            possible_moves.remove(i);
        }
        else {
            i += 1;
        }
    }
}

// pub fn calculate_position (board: &Board, info_matrix: &InfoMatrix, legal_move: &MoveArray) -> [i32; 2]{
//     let [cube_id, forward_fields, turn_direction, is_sw] = legal_move;
//     let forward_direction = forward_fields.signum();
//     let mut new_position = [info_matrix[*cube_id as usize][0], info_matrix[*cube_id as usize][1]];
//     let available_move =  get_top(&board[new_position[0] as usize][new_position[1] as usize]);
//     new_position[*is_sw as usize] += forward_fields;
//     let (is_sw, forward_direction) = change_direction(&turn_direction, &is_sw, &forward_direction);
//     new_position[is_sw as usize] += forward_direction * available_move - forward_fields;
//
//     return new_position;
// }

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
    discard_legal_moves(&board, &info_matrix, &mut possible_moves, &is_white);
    return possible_moves;
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