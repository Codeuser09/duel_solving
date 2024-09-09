use std::{
    error::Error,
    fs::{self, OpenOptions},
    str::FromStr,
    time::Duration,
};

use csv::{ReaderBuilder, WriterBuilder};

use crate::display::{input_float, input_int};

type Parameter = [f64; 3];
type Bot = [Parameter; 7];
type Generation = Vec<Bot>;
type Experiment = Vec<Generation>;
type ExperimentLog = Vec<Experiment>;

pub fn get_genetic_variables() -> (i32, i32, i32, f64, i32, i32, i32) {
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
        "Please enter the random mutation added to each parameter of a child (Floating point numbers allowed)",
    ));
    let generations = input_int(String::from(
        "Please enter the amount of generations the simulation should run for",
    ));
    let train_from_experiment = input_int(String::from(
        "Please enter the experiment you want to continue training from (-1 for last experiment, -2 to start fresh)",
    ));
    let train_from_generation = input_int(String::from(
        "Please enter the generation you want to start from in that experiment (-1 for last generation)",
    ));

    return (
        depth,
        pop_size,
        generations,
        mutation_rate,
        reproduction_number,
        train_from_experiment,
        train_from_generation,
    );
}

fn remove_empty_vectors(input: Vec<Vec<Vec<[[f64; 3]; 7]>>>) -> Vec<Vec<Vec<[[f64; 3]; 7]>>> {
    input
        .into_iter()
        .filter_map(|outer| {
            let filtered_outer: Vec<Vec<[[f64; 3]; 7]>> = outer
                .into_iter()
                .filter_map(|middle| {
                    let filtered_middle: Vec<[[f64; 3]; 7]> = middle.into_iter().collect();
                    if filtered_middle.is_empty() {
                        None
                    } else {
                        Some(filtered_middle)
                    }
                })
                .collect();

            if filtered_outer.is_empty() {
                None
            } else {
                Some(filtered_outer)
            }
        })
        .collect()
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

pub fn elementwise_avg(array1: &[[f64; 3]; 7], array2: &[[f64; 3]; 7]) -> [[f64; 3]; 7] {
    let mut avg_array = [[0f64; 3]; 7];
    for i in 0..array1.len() {
        for e in 0..array1[i].len() {
            avg_array[i][e] = (array1[i][e] + array2[i][e]) / 2f64;
        }
    }
    avg_array
}

pub fn retrieve_experiment_log() -> ExperimentLog {
    let file = fs::read("log/Generations.csv").unwrap();
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&file);

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(res.as_bytes());

    let mut experiment_log: ExperimentLog = vec![vec![vec![]]];
    let mut experiment: Experiment = vec![vec![]];

    for result in reader.records() {
        let record = result.unwrap(); //One line of the csv every time

        //Initiating variables
        let mut generation: Generation = vec![];
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

        //Looping through a single generation, stepping by three to make one parameter (through three game phases) one step
        for c in (0..record.clone().len()).step_by(3) {
            //Resetting the parameter
            let mut parameter: [f64; 3] = [0f64; 3];

            //This is only for testing purposes
            let _value_at_c = record.get(c).unwrap();

            //Getting values for parameter
            for i in 0..=2 {
                //Getting the parameter at c-2+1, because we want to start with the first one, but our c is 3 to large for that
                let parameter_phase_str = record.get(c + i).unwrap();
                //Checking if our parameter is "", so we can tell if we are done with our bot
                if parameter_phase_str == "" {
                    //Checking if we are done with the entire generation, by checking if our bot is just zeroes
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

                    //Appending bot to the generation and restarting
                    generation.push(bot);
                    bot = [[0f64; 3]; 7];
                    finished_bot = true;
                    parameter_index = 0;
                    break;
                }
                parameter[i] = FromStr::from_str(parameter_phase_str).unwrap();
            }

            //If we've finished our bot and the generation isn't done yet
            if finished_bot && !finished_generation {
                //Parameter is needed, so we can continue this loop from the inner loop
                finished_bot = false;
                continue;
            }

            if finished_generation {
                break;
            }

            //Append it to the bot and increase param index
            bot[parameter_index] = parameter;
            parameter_index += 1;
        }
        let mut end_test = true;
        for parameter in bot {
            for gf in parameter {
                if gf != 0f64 {
                    end_test = false
                }
            }
        }
        if !end_test {
            generation.push(bot);
        }
        experiment.push(generation);
    }
    return remove_empty_vectors(experiment_log);
}

pub fn append_to_csv(file_path: &str, row: &[&str]) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().append(true).open(file_path)?;

    let mut wtr = WriterBuilder::new()
        .has_headers(false) // We don't want to write headers again
        .from_writer(file);

    wtr.write_record(row)?;
    wtr.flush()?;

    Ok(())
}

pub fn write_hyperparams(
    elapsed: &Duration,
    depth: &i32,
    pop_size: &i32,
    generations: &i32,
    mutation_rate: &f64,
    reproduction_number: &i32,
    train_from_experiment: &i32,
    train_from_generation: &i32,
) {
    let all_data = vec![
        elapsed.as_secs_f64().to_string(),
        depth.to_string(),
        pop_size.to_string(),
        generations.to_string(),
        mutation_rate.to_string(),
        reproduction_number.to_string(),
        train_from_experiment.to_string(),
        train_from_generation.to_string(),
    ];

    let all_data_str: Vec<&str> = all_data.iter().map(AsRef::as_ref).collect();

    let _ = append_to_csv("log/Hyperparams.csv", &all_data_str);
}

pub fn write_generation(pop: &mut Vec<[[f64; 3]; 7]>) {
    let string_data = convert_vec_to_string_vec(pop.clone());

    let mut all_data_str: Vec<&str> = string_data.iter().map(AsRef::as_ref).collect();

    let mut index = 21;

    while index <= all_data_str.len() {
        for _ in 0..3 {
            all_data_str.insert(index, "");
        }
        index += 24;
    }

    let _ = append_to_csv("log/Generations.csv", &all_data_str);
}
