use crate::cube;
use cube::Cube;

pub type Board = [[[[i32; 4]; 2]; 9]; 8];
pub type IndexMatrix = Vec<[i32; 2]>;

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

pub fn generate_index_matrix(board: Board) -> IndexMatrix {
    let mut index_matrix = vec![[0, 0]; 18];
    let mut index = 0;
    for i in 0..8 {
        for e in 0..9 {
            if board[i][e] != [[0; 4]; 2] {
                index_matrix[index] = [i as i32, e as i32];
                index += 1;
            }
        }
    }
    index_matrix
}
