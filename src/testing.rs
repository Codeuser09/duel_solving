use std::io;
use std::process::exit;
use std::time::SystemTime;
use crate::display::{display_move_array, input_number};
use crate::game::{generate_info_matrix, generate_startpos};
use crate::interaction::play_sample_game;
use crate::legal_move_iteration::{get_possible_moves};
use crate::minimax::{_mt_map_minimax, _mt_minimax_par_iter, minimax};

pub fn test_different_bot_versions () {
    let board = generate_startpos();
    let info_matrix = generate_info_matrix(board);
    let depth = input_number(String::from("Please input the amount of moves that the bots should think ahead"));
    let iterations =  input_number(String::from("Please input how many times the bots should calculate"));

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    for minimax_version in 0..3 {
        for is_move_gen_st in 0..2 {
            let now = SystemTime::now();
            for i in 0..iterations {
                if minimax_version == 0 {
                    let bot_move = _mt_map_minimax(&board, &info_matrix, depth, f64::NEG_INFINITY, f64::INFINITY, true, is_move_gen_st != 0);
                    display_move_array(&bot_move.0);
                }
                if minimax_version == 1 {
                    let bot_move = _mt_minimax_par_iter(&board, &info_matrix, depth, f64::NEG_INFINITY, f64::INFINITY, true, is_move_gen_st != 0);
                    display_move_array(&bot_move.0);
                }
                if minimax_version == 2 {
                    let bot_move = minimax(&board, &info_matrix, depth, f64::NEG_INFINITY, f64::INFINITY, true, is_move_gen_st != 0);
                    display_move_array(&bot_move.0);
                }
            }
            match now.elapsed() {
                Ok(elapsed) => {
                    println!("Bot: minimax multithreading: {}, move generation multithreading {}, depth {}, time {}",
                             if minimax_version == 0 {"map"} else if minimax_version == 1 {"par_iter"} else {"none"},
                             if is_move_gen_st != 0 {"yes"} else {"no"},
                             depth,
                            elapsed.as_millis() / iterations as u128
                    );
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }
    }
}

pub fn display_legal_moves () {
    println!("Do you want to print white's (1) or black's legal moves (0)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let purpose: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    let board = generate_startpos();
    let info_matrix = generate_info_matrix(board);
    let mut i = 0;
    for legal_move in get_possible_moves(&board, &info_matrix, purpose != 0) {
        display_move_array(&legal_move);
        i += 1;
    }
    println!("And now the single threaded legal moves: ");
    let mut I = 0;
    for legal_move in get_possible_moves(&board, &info_matrix, purpose != 0) {
        display_move_array(&legal_move);
        I += 1;
    }
    println!("Total mt legal moves: {i}");
    println!("Total st legal moves: {I}");
}

pub fn dev_mode () {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("What do you want to do?");
    println!("1: Play a sample game");
    println!("2: Print all legal moves");
    println!("3: Test different bot versions");
    println!("3: Exit");
    let mut input= String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let purpose: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    if purpose == 1 {
        play_sample_game();
    }
    if purpose == 2 {
        display_legal_moves();
    }
    if purpose == 3 {
        test_different_bot_versions();
    }
    if purpose == 4 {
        exit(0);
    }
}
