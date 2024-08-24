use std::error::Error;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

use crate::display::{input_float, input_int};
use crate::interaction::play_bvb_game;
use chrono::{DateTime, Utc};
use csv::{ReaderBuilder, WriterBuilder};
use encoding_rs;
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fs;
use std::fs::OpenOptions;

type Parameter = [f64; 3];
type Bot = [Parameter; 7];
type Generation = Vec<Bot>;
type Experiment = Vec<Generation>;
type ExperimentLog = Vec<Experiment>;

fn get_genetic_variables() -> (i32, i32, i32, f64, i32) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let depth = input_int(String::from(
        "Please enter the amount of moves that the bot should calculate into the future:",
    ));
    let pop_size = input_int(String::from(
        "Please enter the amount of bots each generation should have at the start",
    ));
    let reproduction_number = input_int(String::from(
        "Please enter how many bots of these survive and are allowed to reproduce",
    ));
    let mutation_rate = input_float(String::from(
        "Please enter the random mutation added to each parameter of a child",
    ));
    let generations = input_int(String::from(
        "Please enter the amount of generations the simulation should run for",
    ));

    return (
        depth,
        pop_size,
        generations,
        mutation_rate,
        reproduction_number,
    );
}

fn init_population(pop_size: i32) -> Vec<[[f64; 3]; 7]> {
    let mut bot_vector = vec![];
    let mut current_bot = [[0f64; 3]; 7];
    for _ in 0..pop_size {
        for w in 0..current_bot.len() {
            for gp in 0..current_bot[w].len() {
                current_bot[w][gp] = rand::thread_rng().gen_range(-100.0..=100.0);
            }
        }
        bot_vector.push(current_bot);
    }
    return bot_vector;
}

fn fight(
    depth: i32,
    reproduction_number: &i32,
    pop: &mut Vec<[[f64; 3]; 7]>,
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

fn elementwise_avg(array1: &[[f64; 3]; 7], array2: &[[f64; 3]; 7]) -> [[f64; 3]; 7] {
    let mut avg_array = [[0f64; 3]; 7];
    for i in 0..array1.len() {
        for e in 0..array1[i].len() {
            avg_array[i][e] = (array1[i][e] + array2[i][e]) / 2f64;
        }
    }
    avg_array
}

fn reproduce(pop: &mut Vec<[[f64; 3]; 7]>, pop_size: &i32) {
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

fn mutate(pop: &mut Vec<[[f64; 3]; 7]>, reproduction_number: &i32, mutation_rate: &f64) {
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

fn append_to_csv(file_path: &str, row: &[&str]) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().append(true).open(file_path)?;

    let mut wtr = WriterBuilder::new()
        .has_headers(false) // We don't want to write headers again
        .from_writer(file);

    wtr.write_record(row)?;
    wtr.flush()?;

    Ok(())
}

fn convert_vec_to_string_vec(data: Vec<[[f64; 3]; 7]>) -> Vec<String> {
    let mut string_data: Vec<String> = vec![];
    for bot in data {
        for gene in bot {
            for gp in gene {
                string_data.push(gp.to_string());
            }
        }
    }
    string_data
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

fn write_generation(pop: &mut Vec<[[f64; 3]; 7]>) {
    let string_data = convert_vec_to_string_vec(pop.clone());
    // Convert to Vec<&str> for the append_to_csv function
    let mut all_data_str: Vec<&str> = string_data.iter().map(AsRef::as_ref).collect();

    // Inserting empty string after each bot
    for i in (0..all_data_str.len()).step_by(22) {
        all_data_str.insert(i, &"");
    }
    all_data_str.remove(0);

    // Append all data to CSV as a single column
    let _ = append_to_csv("log/Generations.csv", &all_data_str);
}

pub fn read_generations() -> ExperimentLog {
    let file = fs::read("log/Generations.csv").unwrap();
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&file);

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(res.as_bytes());

    let mut experiment_log: ExperimentLog = vec![vec![vec![]]];
    let mut experiment: Experiment = vec![vec![]];
    let mut c_subtract;

    for result in reader.records() {
        let record = result.unwrap(); //One line of the csv every time

        //Redefining generation as an empty vector
        let mut generation: Generation = vec![];
        c_subtract = 1;
        let mut bot: Bot = [[0f64; 3]; 7];
        let mut parameter_index = 0;
        let mut finished_bot = false;
        let mut finished_generation = false;

        //If the experiment finished, push it to experiment log and ignore the empty space
        if record.get(0).unwrap() == "" {
            experiment_log.push(experiment);
            experiment = vec![vec![]];
            continue;
        }

        // if record.get(24).unwrap() == "" {
        //     continue;
        // }

        //Looping through a single generation, stepping by three to make one parameter (through three game phases) one step
        for mut c in (0..record.clone().len()).step_by(3) {
            //To prevent it erroring on reading the first column and then trying to subtract
            if c == 0 {
                continue;
            } else {
                //To prevent weirdness with the step size, because it needs to be bigger by one with 0 for some reason
                c -= c_subtract;
            }

            //Resetting the parameter
            let mut parameter: [f64; 3] = [0f64; 3];

            let _value_at_c = record.get(c).unwrap();

            //Getting values for parameter
            for i in 0..=2 {
                let parameter_phase_str = record.get(c - 2 + i).unwrap();
                if parameter_phase_str == "" {
                    let mut end_test = true;
                    for parameter in bot {
                        for gf in parameter {
                            if gf != 0f64 {
                                end_test = false
                            }
                        }
                    }
                    if end_test {
                        finished_generation = true;
                        break;
                    }

                    generation.push(bot);
                    bot = [[0f64; 3]; 7];
                    finished_bot = true;
                    parameter_index = 0;
                    break;
                }
                parameter[i] = FromStr::from_str(parameter_phase_str).unwrap();
            }

            if finished_bot && !finished_generation {
                finished_bot = false;
                c_subtract += 2;
                continue;
            }

            if finished_generation {
                break;
            }

            //Append it to the bot and increase param index
            if parameter_index == 7 {
                println!("Test");
            }
            bot[parameter_index] = parameter;
            parameter_index += 1;
        }
        experiment.push(generation);
    }
    return experiment_log;
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
            );
            let _ = append_to_csv("log/Generations.csv", &[""]);
            for pop_step in pop_hist.iter_mut() {
                write_generation(pop_step);
            }
            fight(depth, &1, &mut pop, &-1, &0f64, &generations, &pop_size);
            write_generation(&mut pop);
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}
