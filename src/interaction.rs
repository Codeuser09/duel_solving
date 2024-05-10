use std::io;
use std::process::exit;
use dialoguer::Confirm;
use crate::cube::{make_move, MoveArray};
use crate::game::{Board, InfoMatrix, generate_startpos, generate_info_matrix};
use crate::display::{display_info, display_board, display_tops, display_ids, display_move_array, input_number, confirmation};
use crate::legal_move_iteration::{get_possible_moves, get_possible_boards};
use crate::libcube::{count_cubes};
use crate::evaluation::{evaluate_position, is_won};
use crate::minimax::{_mt_map_minimax, minimax};

pub fn play_sample_game() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("What sample game do you want to play?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
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
        move_array_array = vec![[17, -4, 3, 0], [0, 4, 3, 0], [16, 0, 2, 0], [0, 3, 1, 1]]; //Simply black taking the king
    }
    if example_game == 3 {
        move_array_array = vec![ //Black's king ending up on the winning square
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
        ];
    }
    if example_game == 4 {
        move_array_array = vec![ //White's king ending up on the winning square
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
        [13, 0, 1, 0],
        [4, 0, 0, 0],
        [13, 0, 3, 0],
        [4, 0, 0, 0],
        [13, 0, 3, 0],
        [4, 0, 0, 0],
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
            [13, -4, 0, 0],
            [3, 0, 0, 0]
        ];
    }

    for (i, move_array) in move_array_array.iter_mut().enumerate() {
        println!("Move number {i}");
        if make_move(
            &mut board,
            &mut info_matrix,
            &is_white_player,
            &move_array,
        ) == true
        {
            println!();
            println!();
            println!("Exited with code 1");
            println!();
            println!();
        }
        is_white_player = !is_white_player;
        display_info(&board, &info_matrix);
        println!("Cube counter: {}", count_cubes(&board));
        println!("Info matrix length: {}", info_matrix.len());
        println!();
        println!();
        // for legal in get_legal_moves(&board, &info_matrix, is_white_player) {
        //     display_move_array(&legal);
        // }
    }
}

fn get_input(board: &Board, info_matrix: &InfoMatrix, is_white: &bool) -> MoveArray {
    loop {
        // Display the board and info matrix
        display_tops(&board);
        display_ids(&info_matrix, *is_white);
        println!();

        let mut input = String::new();

        // Ask for cube ID

        let cube_id = input_number(String::from("Enter the cube ID (0-17): "));
        let legal_moves = get_possible_moves(&board, &info_matrix, *is_white);
        let mut legal_cube_moves = vec![];
        for legal_move in legal_moves {
            if legal_move[0] == cube_id as i32 {
                legal_cube_moves.push(legal_move)
            }
        }

        if info_matrix[cube_id as usize][3] != *is_white as i32 {
            println!("You cannot move the cube of the opponent");
            continue
        }

        for (i, legal_board) in get_possible_boards(&board, &info_matrix, is_white, &legal_cube_moves).iter().enumerate() {
            println!();
            println!("Move number: {i}");
            display_tops(&legal_board);
        }

        let move_index = input_number(String::from("Please enter the move you want to make"));

        // Return the move array
        let move_array = legal_cube_moves[move_index as usize];
        let mut board_clone = board.clone();
        let mut info_clone = info_matrix.clone();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        if make_move(&mut board_clone, &mut info_clone, &is_white, &move_array) == false {
            let mut board_clone = board.clone();
            let mut info_matrix_clone = info_matrix.clone();
            make_move(&mut board_clone, &mut info_matrix_clone, &is_white, &move_array);
            display_tops(&board_clone);
            if confirmation(String::from("This would be the board after your move, is it correct?"),
                            String::from("Great, next player make your move:"),
                            String::from("never mind then :(, please choose another move")) {
                return move_array
            }
        }
        else {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Illegal move, please choose another");
        }
    }
}


pub fn play_bvh_game() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    let mut input = String::new();
    println!("Please enter the amount of moves that the bot should calculate into the future:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let depth: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    while is_won(&info_matrix) == 0 {
        if is_white == true {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Bot is thinking");
            let bot_move = minimax(&board, &info_matrix, depth, f64::NEG_INFINITY, f64::INFINITY, true, true);
            make_move(&mut board, &mut info_matrix, &true, &bot_move.0);
        }
        else {
            let player_move = get_input(&board, &info_matrix, &false);
            make_move(&mut board, &mut info_matrix, &false, &player_move);
        }
        is_white = !is_white;
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", if is_won(&info_matrix) == 1 {"The bot won"} else {"You won"});
}

pub fn play_bvb_game() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    let mut input = String::new();
    println!("Please enter the amount of moves that the bot should calculate into the future:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let depth: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    while is_won(&info_matrix) == 0 {
        // println!("Evaluation: {}", evaluate_position(&board, &info_matrix));
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        if is_white == true {
            let bot_move = minimax(&board, &info_matrix, depth, f64::NEG_INFINITY, f64::INFINITY, true, true);
            make_move(&mut board, &mut info_matrix, &true, &bot_move.0);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            print!("Bot evaluation: {}, ", bot_move.1);
        } else {
            let bot_move = minimax(&board, &info_matrix, depth, f64::NEG_INFINITY, f64::INFINITY, false, true);
            make_move(&mut board, &mut info_matrix, &false, &bot_move.0);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            print!("Bot evaluation: {}, ", bot_move.1);
        }
        println!("Static evaluation: {}", evaluate_position(&board, &info_matrix));
        is_white = !is_white;
        display_board(&board);
        println!();
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", if is_won(&info_matrix) == 1 {"White won"} else {"Black won"});
}

pub fn play_hvh_game() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;


    while is_won(&info_matrix) == 0 {
        if is_white == true {
            let player_move = get_input(&board, &info_matrix, &true);
            make_move(&mut board, &mut info_matrix, &true, &player_move);
        }
        else {
            let player_move = get_input(&board, &info_matrix, &false);
            make_move(&mut board, &mut info_matrix, &false, &player_move);
        }
        // println!("Evaluation: {}", evaluate_position(&board, &info_matrix));
        is_white = !is_white;
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", if is_won(&info_matrix) == 1 {"White won"} else {"Black won"});
}
