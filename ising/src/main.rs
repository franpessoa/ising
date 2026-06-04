pub mod spin;
use crate::spin::Spin;

struct Config {
    pub h: f64,
    pub j: f64
}

struct Ising {
    lambda: Vec<Spin>,
    x: usize,
    y: usize,
    config: Config
}

impl Ising {
    pub fn hamiltonian(&self) -> f64 {
        let mut sum_j = 0.0;
        let mut sum_h = 0.0;

        // soma H
        for 
    }
}

fn main() {
    println!("Hello, world!");
}
