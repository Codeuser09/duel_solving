use log::warn;
use crate::cube::roll;
use crate::display::display_cube;

use crate::cube::Cube;

pub type Board = [[[[i32; 4]; 2]; 9]; 8];
pub type InfoMatrix = Vec<[i32; 4]>;

fn generate_startcubes() -> (Cube, Cube, Cube, Cube, Cube, Cube) {
    let five: Cube = [[5, 3, 2, 4], [5, 1, 2, 6]];

    let mut six = five.clone();
    roll(-1, 1usize, &mut six);

    let mut two = six.clone();
    roll(-1, 1usize, &mut two);

    let mut one = two.clone();
    roll(-1, 1usize, &mut one);

    let king: Cube = [[1; 4]; 2];
    let zero: Cube = [[0; 4]; 2];

    (five, one, two, six, king, zero)
}

fn black_cube_row(cube_row: [Cube; 9]) -> [Cube; 9] {
    let mut cube_row_b = cube_row.clone();
    for (i, cube) in cube_row_b.iter_mut().enumerate() {
        let mut new_cube = cube.clone();
        roll(2, 1, &mut new_cube);
        roll(2, 0, &mut new_cube);
        *cube = new_cube;
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

