use std::error::Error;
use std::time::{Duration, SystemTime};

use crate::display::{input_float, input_int};
use crate::interaction::play_bvb_game;
use csv::WriterBuilder;
use rand::Rng;
use std::fs::OpenOptions;

fn get_genetic_variables() -> (i32, i32, i32, f64, i32) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let depth = input_int(String::from(
        "Please enter the amount of moves that the bot should calculate into the future:",
    ));
    let pop_size = input_int(String::from(
        "Please enter the amount of bots each generation should have",
    ));
    let generations = input_int(String::from(
        "Please enter the amount of generations the simulation should run for",
    ));
    let mutation_rate = input_float(String::from(
        "Please enter the random mutation added to each parameter of a child",
    ));
    let reproduction_number = input_int(String::from(
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
    }
}

fn elementwise_avg(array1: &[f64; 7], array2: &[f64; 7]) -> [f64; 7] {
    let mut avg_array = [0.0f64; 7];
    for i in 0..array1.len() {
        avg_array[i] = (array1[i] + array2[i]) / 2f64;
    }
    avg_array
}

fn reproduce(pop: &mut Vec<[f64; 7]>, pop_size: &i32) {
    while pop.len() < *pop_size as usize {
        for bot_id in 0..pop.len() {
            if pop.len() == *pop_size as usize {
                break;
            }
            pop.push(elementwise_avg(
                &pop[bot_id as usize],
                &pop[(bot_id + 1) as usize],
            ));
        }
    }
}

fn mutate(pop: &mut Vec<[f64; 7]>, reproduction_number: &i32, mutation_rate: &f64) {
    for (i, bot) in pop.iter_mut().enumerate() {
        if i >= *reproduction_number as usize {
            for gene in bot.iter_mut() {
                *gene += rand::thread_rng().gen_range(-*mutation_rate..=*mutation_rate);
            }
        }
    }
}

fn print_winners(pop: &mut Vec<[f64; 7]>) {
    for bot in pop {
        println!("This bot values: Cube amount: {}, Winning square dist.: {}, Legal move amount: {}, top value sum: {}, dist. to own king sum: {}, dist. to enemy king sum: {}, Interesting move amount: {}", bot[0], bot[1], bot[2], bot[3], bot[4], bot[5], bot[6]);
    }
}

fn append_to_csv(file_path: &str, row: &[&str]) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().append(true).open(file_path)?;

    let mut wtr = WriterBuilder::new()
        .has_headers(false) // We don't want to write headers again
        .from_writer(file);

    wtr.write_record(row)?;
    wtr.flush()?;

    Ok(())
}

fn convert_vec_to_string_vec(data: Vec<[f64; 7]>) -> Vec<String> {
    data.into_iter()
        .flat_map(|row| {
            row.iter()
                .map(|&num| num.to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

fn write_results(
    elapsed: &Duration,
    depth: &i32,
    pop_size: &i32,
    generations: &i32,
    mutation_rate: &f64,
    reproduction_number: &i32,
    pop: &mut Vec<[f64; 7]>,
) {
    println!("");
    println!("");
    println!("The experiment took {} Seconds", elapsed.as_secs_f64());
    println!("The hyperparameters were: Depth: {depth}, pop_size: {pop_size}, generations: {generations}, mutation_rate: {mutation_rate}, reproduction_number: {reproduction_number}");

    let string_data = convert_vec_to_string_vec(pop.clone());

    let mut all_data = vec![
        elapsed.as_secs_f64().to_string(),
        depth.to_string(),
        pop_size.to_string(),
        generations.to_string(),
        mutation_rate.to_string(),
        reproduction_number.to_string(),
    ];

    all_data.extend(string_data);

    let all_data_str: Vec<&str> = all_data.iter().map(AsRef::as_ref).collect();

    let _ = append_to_csv("Output.csv", &all_data_str);
}

fn write_generation(pop: &mut Vec<[f64; 7]>) {
    let string_data = convert_vec_to_string_vec(pop.clone());
    // Convert to Vec<&str> for the append_to_csv function
    let all_data_str: Vec<&str> = string_data.iter().map(AsRef::as_ref).collect();
    // Append all data to CSV as a single column
    let _ = append_to_csv("Generations.csv", &all_data_str);
}

pub fn evolve() {
    let (depth, pop_size, generations, mutation_rate, reproduction_number) =
        get_genetic_variables();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

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

    let start = SystemTime::now();
    let mut pop = init_population(pop_size);

    let _ = append_to_csv("Generations.csv", &[""]);
    write_generation(&mut pop);
    for i in 0..generations {
        fight(depth, &reproduction_number, &mut pop);
        println!("");
        println!("");
        println!("Winners of generation: {i}");
        print_winners(&mut pop);
        reproduce(&mut pop, &pop_size);
        write_generation(&mut pop);
        mutate(&mut pop, &reproduction_number, &mutation_rate);
    }

    match start.elapsed() {
        Ok(elapsed) => {
            write_results(
                &elapsed,
                &depth,
                &pop_size,
                &generations,
                &mutation_rate,
                &reproduction_number,
                &mut pop,
            );
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}
