use cube::MoveArray;
use game::Board;
use game::InfoMatrix;

mod cube;
mod game;
mod legality_check;

use cube::make_move;

fn main() {
    let mut board: Board = game::generate_startpos();
    let mut info_matrix: InfoMatrix = game::generate_info_matrix(board);

    //let move_array: MoveArray = (0, 0, 3, 0);
    //cube::make_move(&mut board, &mut index_matrix, move_array);

    game::display_info(&board, &info_matrix);

    let move_array: MoveArray = (0, 0, 2, 0);

    println!();
    println!("New board");

    make_move(&mut board, &mut info_matrix, &move_array);

    game::display_info(&board, &info_matrix);
}
