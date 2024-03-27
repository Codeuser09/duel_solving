use crate::cube;
use crate::start_cubes::StartCubes;

pub fn init_game(cubes: &StartCubes) -> [[[[i32; 4]; 2]; 9]; 8] {
    let cube_row = [
        cubes.five, cubes.one, cubes.two, cubes.six, cubes.king, cubes.six, cubes.two, cubes.one,
        cubes.five,
    ];
    let zero_row = [cubes.zero; 9];
    let board = [
        cube_row, zero_row, zero_row, zero_row, zero_row, zero_row, zero_row, cube_row,
    ];
    board
}

pub fn display_board_cubes(board: [[[[i32; 4]; 2]; 9]; 8]) {
    for row in board {
        for cube in row {
            cube::display_cube(cube);
            print!(", ");
        }
        println!();
    }
}

pub fn display_board_tops(board: [[[[i32; 4]; 2]; 9]; 8]) {
    for row in board {
        for cube in row {
            print!("{}", cube::get_top(cube));
            print!(", ");
        }
        println!();
    }
}
