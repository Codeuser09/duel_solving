use std::io;
use std::io::stdin;
use crate::cube::{make_move, MoveArray, roll};
use crate::game::{display_info, Board, InfoMatrix, display_info_matrix, display_board, generate_startpos, generate_info_matrix};
use crate::legal_move_iteration::{get_legal_moves, get_possible_boards};
use crate::libcube::display_move_array;
use crate::libcube::display_cube;
use dialoguer::Confirm;
use crate::evaluation::{evaluate_position, is_won};
use crate::minimax::minimax;

pub fn print_legal_moves(board: &mut Board, info_matrix: &mut InfoMatrix, is_white: &bool) {
    let legal_moves = get_legal_moves(&board, &info_matrix, *is_white);
    println!();
    println!();
    println!();
    println!();
    println!("Legal moves: ");


    for legal_move in legal_moves {
        display_move_array(&legal_move);
    }
}

pub fn play_sample_game(board: &mut Board, info_matrix: &mut InfoMatrix, example_game: i32) {
    let is_white_player = true;
    let mut is_white_player = true;
    //move_array = [cube_id, forward_fields, turn_direction, is_sideways];

    let mut move_array_array = vec![];

    if example_game == 1 {
        move_array_array = vec![[17, -4, 3, 0], [0, 4, 3, 0], [17, -3, 1, 1]];
    }
    if example_game == 2 {
        move_array_array = vec![[17, -4, 3, 0], [0, 4, 3, 0], [16, 0, 2, 0], [0, 3, 1, 1]];
    }
    if example_game == 3 {
        move_array_array = vec![
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
            // [0, -1, 1, 0]
        ];
    } else {
        move_array_array = vec![
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
            [13, 0, 3, 0],
            [4, 0, 0, 0],
            [13, 0, 3, 0],
            [4, 0, 0, 0],
            [13, 0, 3, 0],
            [4, 0, 0, 0],
        ];
    }

    for mut move_array in move_array_array.iter_mut() {
        if make_move(
            &mut *board,
            &mut *info_matrix,
            &is_white_player,
            &move_array,
        ) != 0
        {
            println!();
            println!();
            println!("Exited with code 1");
            println!();
            println!();
        }
        is_white_player = !is_white_player;
    }

    display_info(&board, &info_matrix);
}

fn convert_input (human_input: &[[u32; 2];2], info_matrix: &InfoMatrix, board: &Board, is_white: &bool) -> MoveArray {
    let mut chosen_cube_id = 100;
    let legal_moves = &get_legal_moves(&board, &info_matrix, *is_white);
    let possible_boards = get_possible_boards(&board, &info_matrix, is_white, legal_moves);
    for (cube_id, cube) in info_matrix.iter().enumerate() {
        if cube[0] == human_input[0][0] as i32 && cube[1] == human_input[0][1] as i32 {
            chosen_cube_id = cube_id as i32;
        }
    }

    for is_sw in 0..2 {
        for i in 1..4 {
            let mut original_board = board.clone();
            let new_cube = roll(i, is_sw != 0, board[human_input[0][0] as usize][human_input[0][1] as usize]);
            original_board[human_input[1][0] as usize][human_input[1][1] as usize] = new_cube;
            original_board[human_input[0][0] as usize][human_input[0][1] as usize] = [[0; 4];2];
            for (index, possible_board) in possible_boards.iter().enumerate() {
                if original_board == *possible_board { return legal_moves[index]; }
            }
        }
    }
    return [100, 100, 100, 100];
}

pub fn get_input (board: &Board, info_matrix: &InfoMatrix, is_white: &bool) -> MoveArray{
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut coord_array: [[String; 2]; 2] = [[String::new(), String::new()], [String::new(), String::new()]];
    let mut coord_array_int: [[u32; 2];2] = [[10; 2]; 2];
    let mut is_input_correct = true;

    loop {
        loop {
            for coord in 0..coord_array.len() {
                for element in 0..coord_array[coord].len() {
                    display_board(board);

                    println!("Please input the {} that the {} (Starting with index 0 up to {})",
                             if element == 0 { "Row" } else { "Column" },
                             if coord == 0 { "Cube you want to move is in" } else { "Square you want to move that cube to to is in" },
                             if element == 0 { "7" } else { "8" }
                    );

                    io::stdin()
                        .read_line(&mut coord_array[coord][element])
                        .expect("Failed to read line");

                    coord_array_int[coord][element] = match coord_array[coord][element].trim().parse() {
                        Ok(num) => num,
                        Err(e) => continue,
                    };
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                }
            }
            for coord in coord_array_int {
                for element in coord {
                    if element == 10 {
                        is_input_correct == false;
                    }
                }
            }
            if is_input_correct == true { break }
        }

        let real_move = convert_input(&coord_array_int, &info_matrix, &board, &is_white);
        if real_move == [100, 100, 100, 100] {
            println!("Illegal choice, please choose another move");
            continue
        }

        let mut board_clone = board.clone();
        let mut info_matrix_clone = info_matrix.clone();
        make_move(&mut board_clone, &mut info_matrix_clone, &is_white, &real_move);
        display_board(&board_clone);
        println!("This would be the board after your move, is it correct (Y/N)");
        let confirmation = Confirm::new()
            .with_prompt("Do you want to continue?")
            .interact()
            .unwrap();

        if confirmation {
            println!("Looks like it's fine to you");
            return real_move;
        } else {
            println!("nevermind then :(");
        }
    }
}

pub fn play_bvh_game () {
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    while is_won(&info_matrix) == 0 {
        if is_white == true {
            let bot_move= minimax(&board, &info_matrix, -1000000000, 1000000000, 3, true).0;
            make_move(&mut board, &mut info_matrix, &true, &bot_move);
        }
        else {
            let player_move = get_input(&board, &info_matrix, &false);
            make_move(&mut board, &mut info_matrix, &false, &player_move);
        }
        is_white = !is_white;
    }
}

pub fn play_bvb_game () {
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    while is_won(&info_matrix) == 0 {
        println!("Evaluation: {}", evaluate_position(&board, &info_matrix));
        if is_white == true {
            let bot_move = minimax(&board, &info_matrix, -1000000000, 1000000000, 3, true).0;
            make_move(&mut board, &mut info_matrix, &true, &bot_move);
        } else {
            let bot_move = minimax(&board, &info_matrix, -1000000000, 1000000000, 3, false).0;
            make_move(&mut board, &mut info_matrix, &false, &bot_move);
        }
        is_white = !is_white;
        display_board(&board);
    }
}

pub fn play_hvh_game () {
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
        println!("Evaluation: {}", evaluate_position(&board, &info_matrix));
        is_white = !is_white;
    }
}