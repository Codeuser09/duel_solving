use std::ptr::write;
use cube::MoveArray;
use game::Board;
use game::InfoMatrix;
mod cube;
mod game;
mod legality_check;
mod legal_move_iteration;

use cube::make_move;
use crate::legal_move_iteration::get_legal_moves;

fn main() {
    let mut board: Board = game::generate_startpos();
    let mut info_matrix: InfoMatrix = game::generate_info_matrix(board);
    let mut is_white_player = true;

    // let move_array: MoveArray = (0, 0, 3, 0);
    // cube::make_move(&mut board, &mut index_matrix, move_array);

    // move_array = [cube_id, forward_fields, turn_direction]

    // let mut move_array_array = [
    //     (1, 5, 3),
    //     (11, -1, 0),
    //     (7, 4, 1),
    //     (11, -4, 0),
    //     (1, 0, 1),
    //     (8, 0, 2),
    //     (1, 1, 1),
    //     (7, -3, 1) //Weiß
    // ];
    //
    // for mut move_array in move_array_array.iter_mut() {
    //     is_white_player  = !is_white_player;
    //     if make_move(&mut board, &mut info_matrix, &is_white_player, &mut move_array) != 0 {
    //         println!();
    //         println!();
    //         println!("Exited with code 1");
    //         println!();
    //         println!();
    //     }
    // }
    //
    // game::display_info(&board, &info_matrix);

    let legal_moves = get_legal_moves(&board, &info_matrix);
    for legal_move in legal_moves {
        for element in legal_move {
            print!("{}", element);
        }
        println!();
    }

}
