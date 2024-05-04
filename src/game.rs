use crate::libcube::_display_cube;
use crate::libcube::get_top;
use crate::cube::roll;

use crate::cube::Cube;
use crate::evaluation::is_won;

pub type Board = [[[[i32; 4]; 2]; 9]; 8];
pub type InfoMatrix = Vec<[i32; 4]>;

fn generate_startcubes() -> (Cube, Cube, Cube, Cube, Cube, Cube) {
    let five: Cube = [[5, 3, 2, 4], [5, 1, 2, 6]];
    let six: Cube = roll(1, true, five);
    let two: Cube = roll(1, true, six);
    let one: Cube = roll(1, true, two);
    let king: Cube = [[1; 4]; 2];
    let zero: Cube = [[0; 4]; 2];

    (five, one, two, six, king, zero)
}

fn black_cube_row(cube_row: [Cube; 9]) -> [Cube; 9] {
    let mut cube_row_b = cube_row.clone();
    for (_i, cube) in cube_row_b.iter_mut().enumerate() {
        *cube = roll(2, false, *cube);
        *cube = roll(2, true, *cube);
    }
    cube_row_b
}

pub fn generate_startpos() -> Board {
    let (five, one, two, six, king, zero) = generate_startcubes();

    let cube_row_w = [five, one, two, six, king, six, two, one, five];
    let cube_row_b = black_cube_row(cube_row_w);
    let zero_row = [zero; 9];
    [
        cube_row_b, zero_row, zero_row, zero_row, zero_row, zero_row, zero_row, cube_row_w,
    ]
}

pub fn generate_info_matrix(board: Board) -> InfoMatrix {
    let mut king_array = [0; 18];
    let mut x_array = [0; 18];
    let mut y_array = [0; 18];
    let mut is_white_array = [0; 18];

    let mut info_matrix = vec![[0, 0, 0, 0]; 18];
    let mut index = 0;

    for i in 0..8 {
        for e in 0..9 {
            if board[i][e] != [[0; 4]; 2] {
                x_array[index] = i;
                y_array[index] = e;

                if board[i][e] == [[1; 4]; 2] {
                    king_array[index] = 1;
                }

                if i == 7 {
                    is_white_array[index] = 1;
                }

                index += 1;
            }
        }
    }

    for i in 0..18 {
        info_matrix[i] = [
            x_array[i] as i32,
            y_array[i] as i32,
            king_array[i],
            is_white_array[i],
        ];
    }

    info_matrix
}

pub fn _display_info(board: &Board, info_matrix: &InfoMatrix) {
    _display_board(board);
    println!();
    _display_tops(board);
    println!();
    _display_info_matrix(info_matrix);
    println!();
    println!("Is won: {}", is_won(&info_matrix));
}

pub fn _display_board(board: &Board) {
    println!("Board:");
    for row in board {
        print!("[");
        for cube in row {
            _display_cube(cube);
        }
        print!("]");
        println!();
    }
}

pub fn _display_tops(board: &Board) {
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

pub fn _display_info_matrix(index_matrix: &InfoMatrix) {
    println!("Info matrix:");
    for element in index_matrix {
        print!("[");
        for coordinate in element {
            print!("{}", coordinate);
        }
        print!("]");
    }
}
