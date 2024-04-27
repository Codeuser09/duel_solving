use crate::cube::make_move;
use crate::game::{display_info, Board, InfoMatrix};
use crate::legal_move_iteration::get_legal_moves;
use crate::libcube::display_move_array;

pub fn print_legal_moves(board: &mut Board, info_matrix: &mut InfoMatrix, is_white: &bool) {
    let legal_moves = get_legal_moves(&board, &info_matrix, *is_white);
    println!();
    println!();
    println!();
    println!();
    println!("Legal moves: ");


    for legal_move in legal_moves {
        display_move_array(&legal_move);
    }
}

pub fn play_sample_game(board: &mut Board, info_matrix: &mut InfoMatrix, example_game: i32) {
    let is_white_player = true;
    let mut is_white_player = true;
    //move_array = [cube_id, forward_fields, turn_direction, is_sideways];

    let mut move_array_array = vec![];

    if example_game == 1 {
        move_array_array = vec![[17, -4, 3, 0], [0, 4, 3, 0], [17, -3, 1, 1]];
    }
    if example_game == 2 {
        move_array_array = vec![[17, -4, 3, 0], [0, 4, 3, 0], [16, 0, 2, 0], [0, 3, 1, 1]];
    }
    if example_game == 3 {
        move_array_array = vec![
            [13, 0, 2, 0],
            [0, 4, 0, 0],
            [10, 0, 2, 0],
            [5, 1, 1, 0],
            [16, 0, 2, 0],
            [3, 1, 3, 0],
            [13, 0, 2, 0],
            [4, 0, 0, 0],
            [13, 0, 2, 0],
            [4, 0, 0, 0],
            [13, 0, 2, 0],
            [4, 0, 1, 0],
            [13, 0, 2, 0],
            [4, 0, 1, 0],
            [13, 0, 2, 0],
            [4, 0, 1, 0],
            [13, 0, 2, 0],
            // [0, -1, 1, 0]
        ];
    } else {
        move_array_array = vec![
            [13, 0, 2, 0],
            [0, 4, 0, 0],
            [10, 0, 2, 0],
            [5, 1, 1, 0],
            [16, 0, 2, 0],
            [3, 1, 3, 0],
            [13, 0, 2, 0],
            [4, 0, 0, 0],
            [13, 0, 2, 0],
            [4, 0, 0, 0],
            [13, 0, 3, 0],
            [4, 0, 0, 0],
            [13, 0, 3, 0],
            [4, 0, 0, 0],
            [13, 0, 3, 0],
            [4, 0, 0, 0],
            [13, 0, 3, 0],
            [4, 0, 0, 0],
            [13, 0, 3, 0],
            [4, 0, 0, 0],
        ];
    }

    for mut move_array in move_array_array.iter_mut() {
        if make_move(
            &mut *board,
            &mut *info_matrix,
            &is_white_player,
            &move_array,
        ) != 0
        {
            println!();
            println!();
            println!("Exited with code 1");
            println!();
            println!();
        }
        is_white_player = !is_white_player;
    }

    crate::game::display_info(&board, &info_matrix);
}
