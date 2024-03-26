fn get_index(index: i32) -> usize {
    let index_wrapped: i32 = index % 4;
    let index_final: usize = if index_wrapped >= 0 {
        index_wrapped as usize
    } else {
        (index_wrapped + 4) as usize
    };
    index_final
}

fn roll(shift: i32, is_forward: bool, ring_matrix: &mut [[i32; 4]; 2]) -> &mut [[i32; 4]; 2] {
    let is_forward: usize = is_forward as usize;
    let matrix_copy = ring_matrix.clone();
    if shift % 4 != 0 {
        for i in 0..ring_matrix[is_forward].len() {
            ring_matrix[is_forward][i] =
                matrix_copy[is_forward as usize][get_index(i as i32 + shift)];
        }
    }
    ring_matrix
}

fn display_cube(cube_matrix: [[i32; 4]; 2]) {
    for axis in cube_matrix {
        print!("[");
        for element in axis {
            print!("{element}");
        }
        print!("]");
    }
}

fn main() {
    let game_matrix = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let side_ring: [i32; 4] = [6, 4, 1, 3];
    let forward_ring: [i32; 4] = [6, 2, 1, 5];
    let mut cube: [[i32; 4]; 2] = [side_ring, forward_ring];
    let shift: i32 = -1;
    let is_forward: bool = true;

    roll(shift, is_forward, &mut cube);
    display_cube(cube);
}
