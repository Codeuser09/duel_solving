use crate::cube;

pub fn move_cube(
    board: &mut [[[[i32; 4]; 2]; 9]; 8],
    index_matrix: Vec<[i32; 2]>,
    move_array: [i32; 3],
    cube_id: i32,
) {
    let cube_id: usize = cube_id as usize;

    let mut row_index: usize = index_matrix[cube_id][0] as usize;
    let mut column_index: usize = index_matrix[cube_id][1] as usize;

    let current_top: i32 = board[row_index][column_index][0][0];

    for i in 0..current_top {}

    board[row_index][column_index] = [[0; 4]; 2];
}
