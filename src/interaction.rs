use crate::cube::{make_move, MoveArray};
use crate::display::{
    confirmation, display_board, display_ids, display_move_array, display_tops, input_int,
};
use crate::evaluation::{evaluate_position, is_won};
use crate::game::{generate_info_matrix, generate_startpos, Board, InfoMatrix};
use crate::legal_move_iteration::{get_possible_boards, get_possible_moves};
use crate::minimax::minimax;
use crate::{
    CUBE_AMOUNT_WEIGHT, DISTANCE_TO_ENEMY_KING_WEIGHT, DISTANCE_TO_OWN_KING_WEIGHT,
    INTERESTING_MOVE_WEIGHT, LEGAL_MOVE_WEIGHT, TOP_VALUE_WEIGHT, WINNING_SQUARE_WEIGHT,
};

//These are just multipliers for the evaluation parameters and will be calculated by a genetic algo later

fn get_input(board: &Board, info_matrix: &InfoMatrix, is_white: &bool) -> MoveArray {
    loop {
        // Display the board and info matrix
        display_tops(&board);
        display_ids(&info_matrix, *is_white);
        println!();

        let cube_id = input_int(String::from("Enter the cube ID (0-17): "));
        let legal_moves = get_possible_moves(&board, &info_matrix, *is_white);
        let mut legal_cube_moves = vec![];
        for legal_move in legal_moves {
            if legal_move[0] == cube_id {
                legal_cube_moves.push(legal_move)
            }
        }

        if info_matrix[cube_id as usize][3] != *is_white as i32 {
            println!("You cannot move the cube of the opponent");
            continue;
        }

        for (i, legal_board) in
            get_possible_boards(&board, &info_matrix, is_white, &mut legal_cube_moves)
                .iter()
                .enumerate()
        {
            println!();
            println!("Move number: {i}");
            display_tops(&legal_board);
        }

        let move_index = input_int(String::from("Please enter the move you want to make"));

        // Return the move array
        let move_array = legal_cube_moves[move_index as usize];
        let mut board_clone = board.clone();
        let mut info_clone = info_matrix.clone();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        if make_move(&mut board_clone, &mut info_clone, &is_white, &move_array) == false {
            let mut board_clone = board.clone();
            let mut info_matrix_clone = info_matrix.clone();
            make_move(
                &mut board_clone,
                &mut info_matrix_clone,
                &is_white,
                &move_array,
            );
            display_tops(&board_clone);
            if confirmation(
                String::from("This would be the board after your move, is it correct?"),
                String::from("Great, next player make your move:"),
                String::from("never mind then :(, please choose another move"),
            ) {
                return move_array;
            }
        } else {
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

    let depth = input_int(String::from(
        "Please enter the amount of moves that the bot should calculate into the future:",
    ));

    while is_won(&info_matrix) == 0 {
        if is_white == true {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Bot is thinking");
            let bot_move = minimax(
                &board,
                &info_matrix,
                depth,
                depth,
                f64::NEG_INFINITY,
                f64::INFINITY,
                is_white,
                CUBE_AMOUNT_WEIGHT,
                WINNING_SQUARE_WEIGHT,
                LEGAL_MOVE_WEIGHT,
                TOP_VALUE_WEIGHT,
                DISTANCE_TO_OWN_KING_WEIGHT,
                DISTANCE_TO_ENEMY_KING_WEIGHT,
                INTERESTING_MOVE_WEIGHT,
            );
            make_move(&mut board, &mut info_matrix, &true, &bot_move.0);
        } else {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            let player_move = get_input(&board, &info_matrix, &false);
            make_move(&mut board, &mut info_matrix, &false, &player_move);
        }
        is_white = !is_white;
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!(
        "{}",
        if is_won(&info_matrix) == 1 {
            "The bot won"
        } else {
            "You won"
        }
    );
}

pub fn display_play_bvb_game(
    cube_amount_weight: f64,
    winning_square_weight: f64,
    legal_move_weight: f64,
    top_value_weight: f64,
    distance_to_own_king_weight: f64,
    distance_to_enemy_king_weight: f64,
    interesting_move_weight: f64,
) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    let depth = input_int(String::from(
        "Please enter the amount of moves that the bot should calculate into the future:",
    ));

    while is_won(&info_matrix) == 0 {
        // println!("Evaluation: {}", evaluate_position(&board, &info_matrix));
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let bot_move = minimax(
            &board,
            &info_matrix,
            depth,
            depth,
            f64::NEG_INFINITY,
            f64::INFINITY,
            is_white,
            CUBE_AMOUNT_WEIGHT,
            WINNING_SQUARE_WEIGHT,
            LEGAL_MOVE_WEIGHT,
            TOP_VALUE_WEIGHT,
            DISTANCE_TO_OWN_KING_WEIGHT,
            DISTANCE_TO_ENEMY_KING_WEIGHT,
            INTERESTING_MOVE_WEIGHT,
        );
        make_move(&mut board, &mut info_matrix, &is_white, &bot_move.0);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("Bot evaluation: {}", bot_move.1);
        print!(
            ", Static evaluation: {}",
            evaluate_position(
                &board,
                &info_matrix,
                cube_amount_weight,
                winning_square_weight,
                legal_move_weight,
                top_value_weight,
                distance_to_own_king_weight,
                distance_to_enemy_king_weight,
                interesting_move_weight
            )
        );
        print!(", Bot move: ");
        display_move_array(&bot_move.0);
        println!();
        is_white = !is_white;
        display_board(&board);
        println!();
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!(
        "{}",
        if is_won(&info_matrix) == 1 {
            "White won"
        } else {
            "Black won"
        }
    );
}

pub fn play_bvb_game(
    w_weight_array: (f64, f64, f64, f64, f64, f64, f64),
    b_weight_array: (f64, f64, f64, f64, f64, f64, f64),
    depth: i32,
) -> i32 {
    let (
        w_cube_amount_weight,
        w_winning_square_weight,
        w_legal_move_weight,
        w_top_value_weight,
        w_distance_to_own_king_weight,
        w_distance_to_enemy_king_weight,
        w_interesting_move_weight,
    ) = w_weight_array;

    let (
        b_cube_amount_weight,
        b_winning_square_weight,
        b_legal_move_weight,
        b_top_value_weight,
        b_distance_to_own_king_weight,
        b_distance_to_enemy_king_weight,
        b_interesting_move_weight,
    ) = b_weight_array;

    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    while is_won(&info_matrix) == 0 {
        let bot_move: (MoveArray, f64);
        if is_white {
            bot_move = minimax(
                &board,
                &info_matrix,
                depth,
                depth,
                f64::NEG_INFINITY,
                f64::INFINITY,
                is_white,
                w_cube_amount_weight,
                w_winning_square_weight,
                w_legal_move_weight,
                w_top_value_weight,
                w_distance_to_own_king_weight,
                w_distance_to_enemy_king_weight,
                w_interesting_move_weight,
            );
        } else {
            bot_move = minimax(
                &board,
                &info_matrix,
                depth,
                depth,
                f64::NEG_INFINITY,
                f64::INFINITY,
                is_white,
                b_cube_amount_weight,
                b_winning_square_weight,
                b_legal_move_weight,
                b_top_value_weight,
                b_distance_to_own_king_weight,
                b_distance_to_enemy_king_weight,
                b_interesting_move_weight,
            );
        }
        make_move(&mut board, &mut info_matrix, &is_white, &bot_move.0);
        is_white = !is_white;
    }
    is_won(&info_matrix)
}

pub fn play_hvh_game() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let mut board: Board = generate_startpos();
    let mut info_matrix: InfoMatrix = generate_info_matrix(board);
    let mut is_white = true;

    while is_won(&info_matrix) == 0 {
        println!(
            "Evaluation for position: {}",
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
        if is_white == true {
            let player_move = get_input(&board, &info_matrix, &true);
            make_move(&mut board, &mut info_matrix, &true, &player_move);
        } else {
            let player_move = get_input(&board, &info_matrix, &false);
            make_move(&mut board, &mut info_matrix, &false, &player_move);
        }
        // println!("Evaluation: {}", evaluate_position(&board, &info_matrix));
        is_white = !is_white;
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!(
        "{}",
        if is_won(&info_matrix) == 1 {
            "White won"
        } else {
            "Black won"
        }
    );
}
