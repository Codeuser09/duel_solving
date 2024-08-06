use crate::cube::MoveArray;
use crate::evaluation::is_won;
use crate::game::{Board, InfoMatrix};
use crate::libcube::get_top;
use dialoguer::Confirm;
use std::io;

pub fn display_move_array(move_array: &MoveArray) {
    print!("[");
    for element in move_array {
        print!("{element},");
    }
    print!("]");
}
pub fn display_cube(cube_matrix: &[[i32; 4]; 2]) {
    for axis in cube_matrix {
        print!("[");
        for element in axis {
            print!("{}", element);
        }
        print!("]");
    }
}

pub fn display_board(board: &Board) {
    println!("Board:");
    for row in board {
        print!("[");
        for cube in row {
            display_cube(cube);
        }
        print!("]");
        println!();
    }
}

pub fn display_tops(board: &Board) {
    println!("Tops");
    for row in board {
        print!("[");
        for cube in row {
            print!("{}", get_top(cube));
        }
        print!("]");
        println!();
    }
}
pub fn display_info(board: &Board, info_matrix: &InfoMatrix) {
    display_board(board);
    println!();
    display_tops(board);
    println!();
    display_info_matrix(info_matrix);
    println!();
    println!("Is won: {}", is_won(&info_matrix));
}

pub fn display_info_matrix(index_matrix: &InfoMatrix) {
    println!("Info matrix:");
    for element in index_matrix {
        print!("[");
        for coordinate in element {
            print!("{}", coordinate);
        }
        print!("]");
    }
}

pub fn display_ids(info_matrix: &InfoMatrix, is_white: bool) {
    let mut pseudo_board = [[100; 9]; 8];
    for (i, cube) in info_matrix.iter().enumerate() {
        if cube[3] == is_white as i32 {
            pseudo_board[cube[0] as usize][cube[1] as usize] = i;
        }
    }
    println!();
    println!("IDs of your cubes:");
    for row in pseudo_board {
        print!("[");
        for element in row {
            if element == 100 {
                print!("■■,");
            }
            if element <= 9 {
                print!("0{},", element);
            }
            if element != 100 && element > 9 {
                print!("{},", element);
            }
        }
        print!("]");
        println!();
    }
}

pub fn input_int(input_string: String) -> i32 {
    let mut input_int = String::new();
    println!("{}", input_string);
    io::stdin()
        .read_line(&mut input_int)
        .expect("Failed to read line");
    let number: i32 = match input_int.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    number
}

pub fn input_float(input_string: String) -> f64 {
    let mut input = String::new();
    println!("{}", input_string);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    number
}

pub fn confirmation(confirmation_text: String, yes_text: String, no_text: String) -> bool {
    let confirmation = Confirm::new()
        .with_prompt(confirmation_text)
        .interact()
        .unwrap();

    if confirmation {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", yes_text);
        return confirmation;
    } else {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", no_text);
        return confirmation;
    }
}
