use crate::cube;
use cube::Cube;

pub type Board = [[[[i32; 4]; 2]; 9]; 8];
pub type InfoMatrix = Vec<[i32; 4]>;

pub fn generate_startcubes() -> (Cube, Cube, Cube, Cube, Cube, Cube) {
    let five: Cube = [[5, 3, 2, 4], [5, 1, 2, 6]];
    let one: Cube = cube::roll(1, true, five);
    let two: Cube = cube::roll(1, true, one);
    let six: Cube = cube::roll(1, true, two);
    let king: Cube = [[1; 4]; 2];
    let zero: Cube = [[0; 4]; 2];

    (five, one, two, six, king, zero)
}

pub fn generate_startpos() -> Board {
    let (five, one, two, six, king, zero) = generate_startcubes();

    let cube_row = [five, one, two, six, king, six, two, one, five];
    let zero_row = [zero; 9];
    let board = [
        cube_row, zero_row, zero_row, zero_row, zero_row, zero_row, zero_row, cube_row,
    ];
    board
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

pub fn display_info(board: Board, index_matrix: InfoMatrix) {
    display_board(board);
    println!();
    display_tops(board);
    println!();
    display_index_matrix(index_matrix);
}

pub fn display_board(board: Board) {
    for row in board {
        print!("[");
        for cube in row {
            cube::display_cube(cube);
            print!(", ")
        }
        print!("]");
        println!();
    }
}

pub fn display_tops(board: Board) {
    for row in board {
        print!("[");
        for cube in row {
            print!("{}, ", cube::get_top(cube));
        }
        print!("]");
        println!();
    }
}

pub fn display_index_matrix(index_matrix: InfoMatrix) {
    for element in index_matrix {
        print!("[");
        for coordinate in element {
            print!("{}", coordinate);
        }
        print!("], ");
    }
}
