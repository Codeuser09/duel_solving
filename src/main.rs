use std::io::stdin;
use evaluation::evaluate_position;
use game::Board;
use game::InfoMatrix;
use game::{generate_info_matrix, generate_startpos};
use crate::minimax::minimax;
use interaction::play_sample_game;
use interaction::print_legal_moves;
use crate::cube::{make_move, MoveArray};
use crate::game::{display_info, display_info_matrix};
use crate::legal_move_iteration::{filter_duplicates, get_legal_moves};
use crate::evaluation::is_won;
use crate::interaction::{get_input, play_bvh_game, play_bvb_game, play_hvh_game};

mod cube;
mod evaluation;
mod game;
mod legal_move_iteration;
mod legality_check;
mod libcube;
mod interaction;
mod minimax;

fn main() {
    play_bvb_game();
}
