use std::time::SystemTime;

use crate::libgenetics::{
    append_to_csv, get_genetic_variables, retrieve_experiment_log, write_generation,
    write_hyperparams,
};
use crate::{interaction::play_bvb_game, libgenetics::elementwise_avg};
use chrono::{DateTime, Utc};
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

type Parameter = [f64; 3];
type Bot = [Parameter; 7];
type Generation = Vec<Bot>;
type _Experiment = Vec<Generation>;
type _ExperimentLog = Vec<_Experiment>;

fn init_population(
    pop_size: i32,
    train_from_experiment: i32,
    train_from_generation: i32,
) -> Generation {
    let mut bot_vector = vec![];
    let mut current_bot = [[0f64; 3]; 7];
    let mut experiment_no = train_from_experiment;
    let mut generation_no = train_from_generation;

    if train_from_experiment == -2 {
        for _ in 0..pop_size {
            for w in 0..current_bot.len() {
                for gp in 0..current_bot[w].len() {
                    current_bot[w][gp] = rand::thread_rng().gen_range(-100.0..=100.0);
                }
            }
            bot_vector.push(current_bot);
        }
        return bot_vector;
    } else {
        let experiment_log = retrieve_experiment_log();
        if train_from_experiment == -1 {
            experiment_no = (experiment_log.len() - 1) as i32;
        }

        let experiment = experiment_log[experiment_no as usize].clone();
        if train_from_generation == -1 {
            generation_no = (experiment.len() - 2) as i32; //-2, because last gen is just 1 bot
        }
        return experiment[generation_no as usize].clone();
    }
}

fn fight(
    depth: i32,
    reproduction_number: &i32,
    pop: &mut Generation,
    generation: &i32,
    secs: &f64,
    generations: &i32,
    pop_size: &i32,
) {
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    let datetime_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let mut eta = 0f64;
    if *generation != 0 {
        eta = (*secs / *generation as f64 * *generations as f64) - secs;
    }

    println!("");
    println!(
        "{} Starting generation {}/{} with depth {} and {} bots, {} bots will survive. ETA: {}s",
        datetime_str,
        generation,
        generations,
        depth,
        pop.len(),
        reproduction_number,
        eta
    );
    while pop.len() as i32 > *reproduction_number {
        let new_pop: Vec<[[f64; 3]; 7]> = (0..pop.len())
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
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = now.into();
        let datetime_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        eta = eta - (secs / *generation as f64 * (1f64 - (pop.len() as f64 / *pop_size as f64)));

        println!(
            "{} {} bots left on generation {}/{}, ETA: {}s",
            datetime_str,
            pop.len(),
            generation,
            generations,
            eta
        );
    }
}

fn reproduce(pop: &mut Generation, pop_size: &i32) {
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

fn mutate(pop: &mut Generation, reproduction_number: &i32, mutation_rate: &f64) {
    for (i, bot) in pop.iter_mut().enumerate() {
        if i >= *reproduction_number as usize {
            for gene in bot.iter_mut() {
                for gp in gene {
                    *gp += rand::thread_rng().gen_range(-*mutation_rate..=*mutation_rate);
                }
            }
        }
    }
}

pub fn evolve() {
    let (
        depth,
        pop_size,
        generations,
        mutation_rate,
        reproduction_number,
        train_from_experiment,
        train_from_generation,
    ) = get_genetic_variables();

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
    let mut pop = init_population(pop_size, train_from_experiment, train_from_generation);

    let mut pop_hist: Vec<Vec<[[f64; 3]; 7]>> = vec![];
    let mut secs = 0.0;

    pop_hist.push(pop.clone());
    for i in 0..generations {
        match start.elapsed() {
            Ok(elapsed) => {
                secs = elapsed.as_secs_f64();
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }

        fight(
            depth,
            &reproduction_number,
            &mut pop,
            &i,
            &secs,
            &generations,
            &pop_size,
        );
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
                &train_from_experiment,
                &train_from_generation,
            );
            let _ = append_to_csv("log/Generations.csv", &[""]);
            for pop_step in pop_hist.iter_mut() {
                write_generation(pop_step);
            }
            fight(depth, &1, &mut pop, &-1, &0f64, &generations, &pop_size);
            write_generation(&mut pop);
            let _ = append_to_csv("log/Generations.csv", &[""]);
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}
