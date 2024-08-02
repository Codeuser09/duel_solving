use crate::display::input_number;
use crate::interaction::play_bvb_game;
use rand::Rng;

fn get_genetic_variables() -> (i32, i32, i32, i32, i32) {
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

fn init_population(pop_size: i32) -> Vec<[f64; 7]> {
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

fn fight(depth: i32, reproduction_number: &i32, pop: &mut Vec<[f64; 7]>) {
    while pop.len() as i32 > *reproduction_number {
        // while population.len() as i32 != reproduction_number {
        let mut new_pop: Vec<[f64; 7]> = vec![];
        let mut bot_id = 0;
        while bot_id < pop.len() {
            if play_bvb_game(pop[bot_id].into(), pop[bot_id + 1].into(), depth) == 1 {
                // population.remove(bot_id);
                new_pop.push(pop[bot_id]);
            } else {
                // population.remove(bot_id + 1);
                new_pop.push(pop[bot_id + 1]);
            }
            bot_id += 2;
        }
        *pop = new_pop;
        println!("Round 1 finished");
    }
}

fn elementwise_avg(array1: &[f64; 7], array2: &[f64; 7]) -> [f64; 7] {
    let mut avg_array = [0.0f64; 7];
    for i in 0..array1.len() {
        avg_array[i] = (array1[i] + array2[i]) / 2f64;
    }
    avg_array
}

fn reproduce(pop: &mut Vec<[f64; 7]>, pop_size: &i32, reproduction_number: &i32) {
    for bot_id in 0..pop.len() {
        if pop.len() == *pop_size as usize {
            break;
        }
        let new_bot = elementwise_avg(&pop[bot_id as usize], &pop[(bot_id + 1) as usize]);
        pop.push(new_bot);
    }
}

pub fn evolve() {
    let (depth, pop_size, generations, mutation_rate, reproduction_number) =
        get_genetic_variables();

    let mut half_number = pop_size as f64;
    let mut is_allowed = false;
    while half_number >= reproduction_number as f64 {
        half_number = half_number / 2f64;
        if half_number == reproduction_number as f64 {
            is_allowed = true;
        }
    }
    if is_allowed == false {
        panic!(
            "Please input a population size that can be halved until it is the reproduction number"
        );
    }

    let mut pop = init_population(pop_size);

    for _ in 0..generations {
        fight(depth, &reproduction_number, &mut pop);
        reproduce(&mut pop, &pop_size, &reproduction_number);
    }
}
