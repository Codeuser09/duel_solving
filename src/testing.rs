use crate::cube::make_move;
use crate::display::{display_info, display_move_array};
use crate::evaluation::evaluate_position;
use crate::game::{generate_info_matrix, generate_startpos, Board, InfoMatrix};
use crate::genetics::init_population;
use crate::legal_move_iteration::get_possible_moves;
use std::io;
use std::process::exit;

use crate::{
    CUBE_AMOUNT_WEIGHT, DISTANCE_TO_ENEMY_KING_WEIGHT, DISTANCE_TO_OWN_KING_WEIGHT,
    INTERESTING_MOVE_WEIGHT, LEGAL_MOVE_WEIGHT, TOP_VALUE_WEIGHT, WINNING_SQUARE_WEIGHT,
};

pub fn display_legal_moves() {
    println!("Do you want to print white's (1) or black's legal moves (0)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
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
    let mut e = 0;
    for legal_move in get_possible_moves(&board, &info_matrix, purpose != 0) {
        display_move_array(&legal_move);
        e += 1;
    }
    println!("Total mt legal moves: {i}");
    println!("Total st legal moves: {e}");
}

pub fn play_sample_game() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("What sample game do you want to play?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let example_game: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);

    let mut is_white_player = true;
    //move_array = [cube_id, forward_fields, turn_direction, is_sideways];

    let mut move_array_array = vec![];

    if example_game == 1 {
        move_array_array = vec![[17, -4, 3, 0], [0, 4, 3, 0], [17, -3, 1, 1]]; //Simply white taking the king
    }
    if example_game == 2 {
        move_array_array = vec![[17, -4, 3, 0], [0, 4, 3, 0], [16, 0, 2, 0], [0, 3, 1, 1]];
        //Simply black taking the king
    }
    if example_game == 3 {
        move_array_array = vec![
            //Black's king ending up on the winning square
            [13, 0, 2, 0],
            [0, 4, 0, 0],
            [10, 0, 2, 0],
            [5, 1, 1, 0],
            [16, 0, 2, 0],
            [3, 1, 3, 0],
            [13, 0, 2, 0],
            [4, 1, 3, 0],
            [13, 0, 2, 0],
            [4, 1, 3, 0],
            [13, 0, 2, 0],
            [4, 1, 3, 0],
            [13, 0, 2, 0],
            [4, 1, 3, 0],
            [13, 0, 2, 0],
            [4, 1, 3, 0],
        ];
    }
    if example_game == 4 {
        move_array_array = vec![
            //White's king ending up on the winning square
            [13, 0, 2, 0],
            [0, 4, 0, 0],
            [10, 0, 2, 0],
            [5, 1, 1, 0],
            [16, 0, 2, 0],
            [3, 1, 3, 0],
            [13, 0, 2, 0],
            [4, 1, 3, 0],
            [13, 0, 2, 0],
            [4, 1, 3, 0],
            [13, 0, 3, 0],
            [4, 1, 3, 0],
            [13, 0, 3, 0],
            [4, 1, 3, 0],
            [13, 0, 1, 0],
            [4, 1, 3, 0],
            [13, 1, 3, 0],
            [4, 1, 3, 0],
            [13, 1, 3, 0],
            [4, 1, 3, 0],
        ]
    }
    if example_game == 5 {
        move_array_array = vec![
            [12, -5, 1, 0],
            [5, 5, 1, 0],
            [12, 3, 3, 1],
            [5, -3, 3, 1],
            [8, -3, 1, 0],
            [5, -3, 1, 0],
            [11, -3, 3, 0],
            [3, 5, 3, 0],
            [13, -4, 1, 0],
            [3, 1, 0, 0],
        ];
    }

    println!(
        "Static evaluation at start pos: {}",
        evaluate_position(
            &board,
            &info_matrix,
            CUBE_AMOUNT_WEIGHT,
            WINNING_SQUARE_WEIGHT,
            LEGAL_MOVE_WEIGHT,
            TOP_VALUE_WEIGHT,
            DISTANCE_TO_OWN_KING_WEIGHT,
            DISTANCE_TO_ENEMY_KING_WEIGHT,
            INTERESTING_MOVE_WEIGHT
        )
    );
    println!();
    for (i, move_array) in move_array_array.iter_mut().enumerate() {
        println!("Move number {i}");

        if make_move(&mut board, &mut info_matrix, &is_white_player, &move_array) == true {
            println!();
            println!();
            println!("Exited with code 1");
            panic!("");
        }
        is_white_player = !is_white_player;
        display_info(&board, &info_matrix);
        println!(
            "Static evaluation: {}",
            evaluate_position(
                &board,
                &info_matrix,
                CUBE_AMOUNT_WEIGHT,
                WINNING_SQUARE_WEIGHT,
                LEGAL_MOVE_WEIGHT,
                TOP_VALUE_WEIGHT,
                DISTANCE_TO_OWN_KING_WEIGHT,
                DISTANCE_TO_ENEMY_KING_WEIGHT,
                INTERESTING_MOVE_WEIGHT
            )
        );
        println!();
        println!();
    }
}
pub fn dev_mode() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("What do you want to do?");
    println!("1: Play a sample game");
    println!("2: Print all legal moves");
    println!("3: Test the init_population function");
    println!("4: Exit");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
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
        for bot in init_population() {
            println!("{:?}", bot);
        }
    }
    if purpose == 4 {
        exit(0);
    }
}
