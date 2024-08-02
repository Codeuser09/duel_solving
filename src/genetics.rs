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
    while population.len() as i32 > reproduction_number {
        // while population.len() as i32 != reproduction_number {
        let mut new_population: Vec<[f64; 7]> = vec![];
        let mut bot_id = 0;
        while bot_id < population.len() {
            if play_bvb_game(
                population[bot_id].into(),
                population[bot_id + 1].into(),
                depth,
            ) == 1
            {
                // population.remove(bot_id);
                new_population.push(population[bot_id]);
            } else {
                // population.remove(bot_id + 1);
                new_population.push(population[bot_id + 1]);
            }
            bot_id += 2;
        }
        *population = new_population;
        println!("Round 1 finished");
    }
}

pub fn evolve() {
    let (depth, pop_size, generations, mutation_rate, reproduction_number) =
        get_genetic_variables();

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
