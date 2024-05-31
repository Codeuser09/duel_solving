use crate::display::input_number;
use crate::interaction::play_bvb_game;
use crate::MUT_RATE;
use crate::POP_SIZE;
use rand::Rng;

pub fn init_population() -> Vec<[f64; 7]> {
    let mut bot_vector = vec![];
    let mut current_bot = [0f64; 7];
    for _ in 0..POP_SIZE {
        for w in 0..current_bot.len() {
            current_bot[w] = rand::thread_rng().gen_range(-100.0..=100.0);
        }
        bot_vector.push(current_bot);
    }
    return bot_vector;
}

pub fn fight() {
    let population = init_population();
    let depth = input_number(String::from(
        "Please enter the amount of moves that the bot should calculate into the future:",
    ));
    play_bvb_game(population[0].into(), depth);
}
