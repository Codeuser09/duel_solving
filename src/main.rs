use std::io;
use std::process::exit;
use crate::cube::make_move;
use crate::display::display_board;
use crate::game::{generate_info_matrix, generate_startpos};
use crate::interaction::{dev_mode, play_bvb_game, play_bvh_game, play_hvh_game, play_sample_game};
use crate::libcube::calculate_position;

mod cube;
mod evaluation;
mod game;
mod legal_move_iteration;
mod legality_check;
mod libcube;
mod interaction;
mod minimax;
mod display;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("What do you want to do?");
    println!("1: Play against a computer");
    println!("2: Play against another human");
    println!("3: Let two bots play against each other");
    println!("4: Enter dev mode");
    println!("5: Exit");
    let mut input= String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let purpose: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };
    if purpose == 1 {
        play_bvh_game();
    }
    if purpose == 2 {
        play_hvh_game();
    }
    if purpose == 3 {
        play_bvb_game();
    }
    if purpose == 4 {
        dev_mode();
    }
    if purpose == 5 {
        exit(0);
    }
    // let mut board = generate_startpos();
    // let mut info_matrix = generate_info_matrix(board);
    // let my_move= [12, -5, 0, 0];
    // for element in calculate_position(&board, &info_matrix, &my_move) {
    //     println!("{}", element);
    // }
}