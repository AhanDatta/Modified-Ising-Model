use rand::Rng; //to create a default rng state 

pub const NUM_PARTICLES: usize = 52;
pub const BETA: f64 = 10.0; //Inverse temperature constant for deciding whether to accept higher energy state

//Spin vector type for each lattice point
//Basically a polar vector in R^2, made for readability
#[derive(Debug, Copy, Clone)]
pub struct Spin {
    pub norm: f64,
    pub arg: f64, 
}

impl Spin {
    pub fn new_zero () -> Spin {
        return Spin {norm: 0.0, arg: 0.0};
    }
}

//Type for whole microstate of the system
#[derive(Debug, Clone, Copy)]
pub struct Lattice {
    pub points: [Spin; NUM_PARTICLES],
}

impl Lattice {
    //Constructor that gives random Lattice from uniform distribution
    //Then normalizes lengths to give the desired energy 
    pub fn new_rand(max_lattice_energy: f64) -> Lattice {
        let mut rng = rand::thread_rng();
        let mut new_point_array: [Spin; NUM_PARTICLES] = [Spin::new_zero(); NUM_PARTICLES];
        
        //Assigns random spins according to seed
        for i in 0..NUM_PARTICLES {
            new_point_array[i] = Spin{norm: 1.0, arg: rng.gen::<f64>() * 2.0*std::f64::consts::PI};
        }

        //Constructs lattice and normalizes
        let mut ret_lattice: Lattice = Lattice {points: new_point_array};
        let mut curr_max_lattice_energy: f64 = 1.0;
        for point in ret_lattice.points.iter() {
            curr_max_lattice_energy *= point.norm.powi(2);
        }
        let normalization_factor: f64 = (max_lattice_energy/curr_max_lattice_energy).sqrt();
        for point in ret_lattice.points.iter_mut() {
            point.norm = normalization_factor * point.norm;
        }

        //Adding a negative sign if the state starts with negative energy
        if ret_lattice.energy() < 0.0 {
            for point in ret_lattice.points.iter_mut() {
                point.arg = -point.arg;
            }
        }

        return ret_lattice;
    }

    //Gives a perturbed version of the input Lattice 
    //Picks point of disturbance, changes norm and angle
    //Returns the new lattice
    pub fn perturb (input_lattice: &Lattice) -> Lattice {
        let mut rng = rand::thread_rng();
        let disturbance_number: usize = rng.gen_range(0..NUM_PARTICLES);
        let mut perturbed_lattice: Lattice = input_lattice.clone();

        //Does the disturbing. Norm is 0.9 to 1.1 of original, arg is arbitrary
        perturbed_lattice.points[disturbance_number] = Spin{
            norm: input_lattice.points[disturbance_number].norm, 
            arg: rng.gen::<f64>() * 2.0*std::f64::consts::PI};
        
        return perturbed_lattice;
    }

    //Takes in itself as a microstate and returns the 
    //Hamiltonian of sum over cross products
    pub fn energy(&self) -> f64 {
        let mut state_energy: f64 = 0.0;
        //Computes the energy for the interior points
        for i in 0..(NUM_PARTICLES - 1) {
            let p1: &Spin = &self.points[i];
            let p2: &Spin = &self.points[i+1];
            state_energy += p1.norm * p2.norm * (p2.arg - p1.arg).sin();
        }

        //Imposing periodic boundary conditions
        state_energy += (self.points[NUM_PARTICLES - 1].norm) * (self.points[0].norm) * (self.points[0].arg - self.points[NUM_PARTICLES - 1].arg).sin();
        return state_energy;
    }
}