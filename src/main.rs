use rand::Rng; //for all random gen
use std::fs::File;
use csv;
mod lattice; 
use crate::lattice::Lattice;

const NUM_STEPS: i32 = 10000 as i32; 
const WRITE_COUNT: u32 = 10 as u32;

//Rounding function for display 
/*
fn round_dp(num: f64, precision: i32) -> f64 {
    return ((num * (10.0_f64).powi(precision)).round())/((10.0_f64).powi(precision));
}
*/

//Handling csv writting shit
fn write_csv(writer: &mut csv::Writer<File>, writing_vec: &Vec<String>) -> Result<(), String> {
    // Write some records.
    writer.write_record(writing_vec).expect("Error writing header"); 

    // Flush the writer to ensure everything gets written. In Python, you wouldn't use that if you use a "with open('..') as f:"
    writer.flush().expect("Error writing");

    Ok(())
}

//Main algorithm found here: https://en.wikipedia.org/wiki/Ising_model
fn main() {
    // Create a new CSV writer.
    let file = File::create("output.csv").expect("Couldn't create output.csv");
    let mut writer = csv::Writer::from_writer(file);
    let lattice_energy: f64 = 1.0; //is arbitrary, start wherever you want except 0
    let mut plotting_num: u32 = 0;
    let mut state: Lattice = Lattice::new_rand(lattice_energy);
    let mut rng = rand::thread_rng();
    let mut writing_vec: Vec<String> = vec![];
    for _ in 0..NUM_STEPS {
        //Outputs state to csv file
        if (plotting_num % WRITE_COUNT) == 0 {
            for spin in state.points.iter() {
                let spin_str = spin.arg.to_string();
                writing_vec.push(spin_str);
            }
            write_csv(&mut writer, &writing_vec).unwrap();
            writing_vec = Vec::new();
        }
        plotting_num += 1;

        let curr_energy: f64 = state.energy();
        //dbg!(curr_energy);
        let new_state: Lattice = Lattice::perturb(&state);
        let new_energy: f64 = new_state.energy();

        if new_energy < curr_energy {
            state = new_state;
        }
        else {
            let rand_probability = rng.gen::<f64>();
            let acceptance_probability: f64 = (-lattice::BETA * (new_energy - curr_energy)).exp();
            if rand_probability < acceptance_probability {
                state = new_state;
            }
            else {
                state = state;
            }
        }
    }
}
