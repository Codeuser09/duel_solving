use crate::display::input_number;
use crate::interaction::play_bvb_game;
use rand::Rng;

pub fn get_genetic_variables() -> (i32, i32, i32, i32, i32) {
    let depth = input_number(String::from(
        "Please enter the amount of moves that the bot should calculate into the future:",
    ));
    let pop_size = input_number(String::from(
        "Please enter the amount of bots each generation should have",
    ));
    let generations = input_number(String::from(
        "Please enter the amount of generations the simulation should run for",
    ));
    let mutation_rate = input_number(String::from(
        "Please enter the random mutation added to each parameter of a child",
    ));
    let reproduction_number = input_number(String::from(
        "Please enter how many bots are allowed to reproduce",
    ));
    return (
        depth,
        pop_size,
        generations,
        mutation_rate,
        reproduction_number,
    );
}

pub fn init_population(pop_size: i32) -> Vec<[f64; 7]> {
    let mut bot_vector = vec![];
    let mut current_bot = [0f64; 7];
    for _ in 0..pop_size {
        for w in 0..current_bot.len() {
            current_bot[w] = rand::thread_rng().gen_range(-100.0..=100.0);
        }
        bot_vector.push(current_bot);
    }
    return bot_vector;
}

pub fn fight(
    depth: i32,
    pop_size: i32,
    generations: i32,
    mutation_rate: i32,
    reproduction_number: i32,
    population: &mut Vec<[f64; 7]>,
) {
    let mut bot_id: usize = 0;
    for _ in 0..generations {
        while population.len() != 10 {
            while bot_id as i32 != population.len() as i32 {
                if play_bvb_game(
                    population[bot_id as usize].into(),
                    population[bot_id as usize + 1].into(),
                    depth,
                ) == 1
                {
                    population.remove(bot_id);
                }
                bot_id += 1;
            }
        }
    }
}

pub fn evolve() {
    let (depth, pop_size, generations, mutation_rate, reproduction_number) =
        get_genetic_variables();

    if pop_size % 2 != 0 {
        panic!("Pop_size must be divisible by 2 for the tournament elimination to work");
    }

    let mut population = init_population(pop_size);

    fight(
        depth,
        pop_size,
        generations,
        mutation_rate,
        reproduction_number,
        &mut population,
    );
    println!("{:?}", population);
}
