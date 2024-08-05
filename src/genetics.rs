use std::error::Error;
use std::time::{Duration, SystemTime};

use crate::display::{input_float, input_int, textless_confirmation};
use crate::interaction::play_bvb_game;
use csv::WriterBuilder;
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
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

fn fight(depth: i32, reproduction_number: &i32, pop: &mut Vec<[f64; 7]>, generation: &i32) {
    println!("");
    println!(
        "Starting generation {} with {} bots, {} bots will survive",
        generation,
        pop.len(),
        reproduction_number
    );
    while pop.len() as i32 > *reproduction_number {
        let new_pop: Vec<[f64; 7]> = (0..pop.len())
            .into_par_iter()
            .step_by(2)
            .map(|bot_id| {
                if play_bvb_game(pop[bot_id].into(), pop[bot_id + 1].into(), depth) == 1 {
                    pop[bot_id]
                } else {
                    pop[bot_id + 1]
                }
            })
            .collect();

        *pop = new_pop;
        println!("{} bots left on generation {}", pop.len(), generation);
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

fn write_hyperparams(
    elapsed: &Duration,
    depth: &i32,
    pop_size: &i32,
    generations: &i32,
    mutation_rate: &f64,
    reproduction_number: &i32,
) {
    let all_data = vec![
        elapsed.as_secs_f64().to_string(),
        depth.to_string(),
        pop_size.to_string(),
        generations.to_string(),
        mutation_rate.to_string(),
        reproduction_number.to_string(),
    ];

    let all_data_str: Vec<&str> = all_data.iter().map(AsRef::as_ref).collect();

    let _ = append_to_csv("log/Hyperparams.csv", &all_data_str);
}

fn write_generation(pop: &mut Vec<[f64; 7]>) {
    let string_data = convert_vec_to_string_vec(pop.clone());
    // Convert to Vec<&str> for the append_to_csv function
    let mut all_data_str: Vec<&str> = string_data.iter().map(AsRef::as_ref).collect();

    // Inserting empty string after each bot
    for i in (0..all_data_str.len()).step_by(8) {
        all_data_str.insert(i, &"");
    }
    all_data_str.remove(0);

    // Append all data to CSV as a single column
    let _ = append_to_csv("log/Generations.csv", &all_data_str);
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
    let mut pop_hist: Vec<Vec<[f64; 7]>> = vec![];

    pop_hist.push(pop.clone());
    for i in 0..generations {
        fight(depth, &reproduction_number, &mut pop, &i);
        reproduce(&mut pop, &pop_size);
        pop_hist.push(pop.clone());
        mutate(&mut pop, &reproduction_number, &mutation_rate);
    }

    match start.elapsed() {
        Ok(elapsed) => {
            write_hyperparams(
                &elapsed,
                &depth,
                &pop_size,
                &generations,
                &mutation_rate,
                &reproduction_number,
            );
            let _ = append_to_csv("log/Generations.csv", &[""]);
            for pop_step in pop_hist.iter_mut() {
                write_generation(pop_step);
            }
            fight(depth, &1, &mut pop, &-1);
            write_generation(&mut pop);
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}
