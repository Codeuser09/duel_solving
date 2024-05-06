use std::io;
use std::process::exit;
use crate::interaction::{play_bvb_game, play_bvh_game, play_hvh_game, play_sample_game};

mod cube;
mod evaluation;
mod game;
mod legal_move_iteration;
mod legality_check;
mod libcube;
mod interaction;
mod minimax;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("What do you want to do?");
    println!("1: Play against a computer");
    println!("2: Play against another human");
    println!("3: Let two bots play against each other");
    println!("4: Play a sample game");
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
        play_sample_game();
    }
    if purpose == 5 {
        exit(0);
    }
}