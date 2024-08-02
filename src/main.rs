use crate::genetics::evolve;
use crate::interaction::{display_play_bvb_game, play_bvh_game, play_hvh_game};
use crate::testing::dev_mode;
use std::io;
use std::process::exit;

mod cube;
mod display;
mod evaluation;
mod game;
mod genetics;
mod interaction;
mod legal_move_iteration;
mod legality_check;
mod libcube;
mod minimax;
mod testing;

const CUBE_AMOUNT_WEIGHT: f64 = 1f64;
const WINNING_SQUARE_WEIGHT: f64 = 1f64;
const LEGAL_MOVE_WEIGHT: f64 = 1f64;
const TOP_VALUE_WEIGHT: f64 = 1f64;
const DISTANCE_TO_OWN_KING_WEIGHT: f64 = 1f64;
const DISTANCE_TO_ENEMY_KING_WEIGHT: f64 = 1f64;
const INTERESTING_MOVE_WEIGHT: f64 = 1f64;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("What do you want to do?");
    println!("1: Play against a computer");
    println!("2: Play against another human");
    println!("3: Let two bots play against each other");
    println!("4: Experiment with the genetic algorithm");
    println!("5: Enter dev mode");
    println!("6: Exit");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
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
        display_play_bvb_game(
            CUBE_AMOUNT_WEIGHT,
            DISTANCE_TO_ENEMY_KING_WEIGHT,
            DISTANCE_TO_OWN_KING_WEIGHT,
            INTERESTING_MOVE_WEIGHT,
            LEGAL_MOVE_WEIGHT,
            TOP_VALUE_WEIGHT,
            WINNING_SQUARE_WEIGHT,
        );
    }
    if purpose == 4 {
        evolve();
    }
    if purpose == 5 {
        dev_mode();
    }
    if purpose == 6 {
        exit(0);
    }
}
