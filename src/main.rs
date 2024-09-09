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
mod libgenetics;
mod minimax;
mod testing;

//Values are obviously only used for bvh and display_bvb games
// const CUBE_AMOUNT_WEIGHT: [f64; 3] = [
//     67.4259109894776f64, //Value for opening
//     67.4259109894776f64, //Value for middlegame
//     67.4259109894776f64, //Value for endgame
// ];
// const WINNING_SQUARE_WEIGHT: [f64; 3] = [
//     18.2168822548758f64,
//     18.2168822548758f64,
//     18.2168822548758f64,
// ];
// const LEGAL_MOVE_WEIGHT: [f64; 3] = [
//     59.6632650085869f64,
//     59.6632650085869f64,
//     59.6632650085869f64,
// ];
// const TOP_VALUE_WEIGHT: [f64; 3] = [
//     83.1413071732032f64,
//     83.1413071732032f64,
//     83.1413071732032f64,
// ];
// const DISTANCE_TO_OWN_KING_WEIGHT: [f64; 3] = [
//     -61.9738412116521f64,
//     -61.9738412116521f64,
//     -61.9738412116521f64,
// ];
// const DISTANCE_TO_ENEMY_KING_WEIGHT: [f64; 3] = [
//     19.30592161527491f64,
//     19.30592161527491f64,
//     19.30592161527491f64,
// ];
// const INTERESTING_MOVE_WEIGHT: [f64; 3] = [
//     -15.5835110143151f64,
//     -15.5835110143151f64,
//     -15.5835110143151f64,
// ];

//The values are noted down in the csv file in this manner:
// Param 1 endgame, param 1 midgame, param 1 opening, param 2 endgame...
//Same with the values below, so just copy csv over to below

const CUBE_AMOUNT_WEIGHT: [f64; 3] = [
    -29.8003260403288f64,
    -38.0478803745276f64,
    33.746154376666f64,
];
const WINNING_SQUARE_WEIGHT: [f64; 3] = [
    35.0453316809402f64,
    52.5566024439559f64,
    2.26847612755076f64,
];
const LEGAL_MOVE_WEIGHT: [f64; 3] = [7.7625717884876f64, 26.8205897884494f64, 45.1310922372689f64];
const TOP_VALUE_WEIGHT: [f64; 3] = [21.588063002618f64, 46.8705743627489f64, 51.2811082048522f64];
const DISTANCE_TO_OWN_KING_WEIGHT: [f64; 3] = [
    0.491299383788078f64,
    -27.4134795087902f64,
    -27.4816564067347f64,
];
const DISTANCE_TO_ENEMY_KING_WEIGHT: [f64; 3] = [
    11.7672634836059f64,
    6.01819012836402f64,
    2.45817788688824f64,
];
const INTERESTING_MOVE_WEIGHT: [f64; 3] = [
    4.28620908642498f64,
    41.0770984457881f64,
    -14.6659853406584f64,
];

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
