mod cube;
mod game;
mod start_cubes;

fn main() {
    let start_cubes = start_cubes::StartCubes::new();
    let board = game::init_game(&start_cubes);
    game::display_board_cubes(board);
    game::display_board_tops(board);
}
