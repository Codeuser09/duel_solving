use std::collections::HashSet;
use std::io;
use crate::cube::{Cube, make_move, MoveArray, roll};
use crate::game::{_display_info, Board, InfoMatrix, _display_board, generate_startpos, generate_info_matrix, _display_tops};
use crate::legal_move_iteration::{get_legal_moves, get_possible_boards};
use crate::libcube::{_display_move_array};
use dialoguer::Confirm;
use crate::evaluation::{evaluate_position, is_won};
use crate::minimax::minimax;

pub fn _play_sample_game(example_game: i32) {
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);

    let mut is_white_player = true;
    //move_array = [cube_id, forward_fields, turn_direction, is_sideways];

    let mut move_array_array ;

    if example_game == 1 {
        move_array_array = vec![[17, -4, 3, 0], [0, 4, 3, 0], [17, -3, 1, 1]];
    }
    else {
        move_array_array = vec![[17, -4, 3, 0], [0, 4, 3, 0], [16, 0, 2, 0], [0, 3, 1, 1]];
    }

    for move_array in move_array_array.iter_mut() {
        if make_move(
            &mut board,
            &mut info_matrix,
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
        _display_info(&board, &info_matrix);
        println!();
        println!("Eval: {}", evaluate_position(&board, &info_matrix));
    }

    _display_info(&board, &info_matrix);
}

fn _generate_possible_rollings (cube_matrix: &Cube) -> Vec<Cube> {
    let mut possible_rollings : Vec<Cube>= vec![];
    let mut cubes_already_in = HashSet::new();
    for starts_sw in 0..2 {
        for starts_with_ff in -3..4 {
            for turns_sw in 0..2 {
                for turns_ff in -3..4 {
                    let cube_copy = cube_matrix.clone();
                    let new_cube = roll(starts_with_ff, starts_sw != 0, cube_copy);
                    if cubes_already_in.insert(new_cube) {
                        possible_rollings.push(new_cube);
                    }
                    if turns_sw == 1 {
                        let new_cube = roll(turns_ff, turns_sw != 0, new_cube);
                        if cubes_already_in.insert(new_cube) {
                            possible_rollings.push(new_cube);
                        }
                    }
                }
            }
        }
    }
    return possible_rollings
}

fn _convert_input (human_input: &[[u32; 2];2], info_matrix: &InfoMatrix, board: &Board, is_white: &bool) -> MoveArray {
    let legal_moves = &get_legal_moves(&board, &info_matrix, *is_white);
    let possible_boards = get_possible_boards(&board, &info_matrix, is_white, legal_moves);

    for possible_cube in _generate_possible_rollings(&board[human_input[0][0] as usize][human_input[0][1] as usize]) {
        for (i, possible_board) in possible_boards.iter().enumerate() {
            let mut board_copy = board.clone();
            board_copy[human_input[1][0] as usize][human_input[1][1] as usize] = possible_cube;
            board_copy[human_input[0][0] as usize][human_input[0][1] as usize] = [[0; 4]; 2];
            if board_copy == *possible_board {
                return legal_moves[i];
            }
        }
    }
    return [100, 100, 100, 100];
}

pub fn _get_input (board: &Board, info_matrix: &InfoMatrix, is_white: &bool) -> MoveArray{
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let mut coord_array: [[String; 2]; 2] = [[String::new(), String::new()], [String::new(), String::new()]];
        let mut coord_array_int: [[u32; 2];2] = [[10; 2]; 2];
        let mut is_input_correct = true;
        loop {
            for coord in 0..coord_array.len() {
                for element in 0..coord_array[coord].len() {
                    _display_tops(&board);

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
                        Err(_) => continue,
                    };
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                }
            }
            for coord in coord_array_int {
                for element in coord {
                    if element == 10 {
                        is_input_correct = false;
                    }
                }
            }
            if is_input_correct == true { break }
        }

        let real_move = _convert_input(&coord_array_int, &info_matrix, &board, &is_white);
        if real_move == [100, 100, 100, 100] {
            println!("Illegal choice, please choose another move");
            continue
        }

        let mut board_clone = board.clone();
        let mut info_matrix_clone = info_matrix.clone();
        make_move(&mut board_clone, &mut info_matrix_clone, &is_white, &real_move);
        _display_tops(&board_clone);
        let confirmation = Confirm::new()
            .with_prompt("This would be the board after your move, is it correct?")
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

pub fn _play_bvh_game () {
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    while is_won(&info_matrix) == 0 {
        if is_white == true {
            let bot_move = minimax(&board, &info_matrix, 3, f64::NEG_INFINITY, f64::INFINITY, true);
            make_move(&mut board, &mut info_matrix, &true, &bot_move.0);
        }
        else {
            let player_move = _get_input(&board, &info_matrix, &false);
            make_move(&mut board, &mut info_matrix, &false, &player_move);
        }
        is_white = !is_white;
    }
}

pub fn _play_bvb_game () {
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    while is_won(&info_matrix) == 0 {
        // println!("Evaluation: {}", evaluate_position(&board, &info_matrix));
        print!("Is it white to move?: {}, ", is_white);
        if is_white == true {
            let bot_move = minimax(&board, &info_matrix, 3, f64::NEG_INFINITY, f64::INFINITY, true);
            make_move(&mut board, &mut info_matrix, &true, &bot_move.0);
            print!("Bot evaluation: {}, ", bot_move.1);
            _display_move_array(&bot_move.0);
        } else {
            let bot_move = minimax(&board, &info_matrix, 3, f64::NEG_INFINITY, f64::INFINITY, false);
            make_move(&mut board, &mut info_matrix, &false, &bot_move.0);
            print!("Bot evaluation: {}, ", bot_move.1);
            _display_move_array(&bot_move.0);
        }
        println!("Static evaluation: {}", evaluate_position(&board, &info_matrix));
        is_white = !is_white;
        _display_board(&board);
        println!();
    }
}

pub fn _play_hvh_game () {
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    while is_won(&info_matrix) == 0 {
        if is_white == true {
            let player_move = _get_input(&board, &info_matrix, &true);
            make_move(&mut board, &mut info_matrix, &true, &player_move);
        }
        else {
            let player_move = _get_input(&board, &info_matrix, &false);
            make_move(&mut board, &mut info_matrix, &false, &player_move);
        }
        // println!("Evaluation: {}", evaluate_position(&board, &info_matrix));
        is_white = !is_white;
        println!("Evaluation: {}", evaluate_position(&board, &info_matrix));
    }
}