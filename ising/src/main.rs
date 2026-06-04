use core::f64;
use std::fs::File;

use gif::{Encoder, Frame};
use rand::RngExt;
use utils::{grid_pbc::PBCGrid, metropolis::*, spin::Spin};

#[derive(Debug, Clone)]
struct Config {
    pub h: f64,
    pub j: f64,
    pub beta: f64, // (k_B \cdot T)^{-1}
}

#[derive(Clone, Debug)]
struct Ising {
    pub lambda: PBCGrid<Spin>,
    pub config: Config,
}

fn hamiltoniana_ising(s: &Ising) -> f64 {
    let sum_j: f64 = s
        .lambda
        .get_all_adjacent()
        .into_iter()
        .map(|(i, viz)| {
            viz.into_iter()
                .map(|v| (*v * *i * s.config.j) as f64)
                .sum::<f64>()
        })
        .sum();

    let sum_h: f64 = s.lambda.get_all().iter().map(|x| *x * s.config.h).sum();

    -sum_j - sum_h
}

fn p_ising(s: &Ising) -> f64 {
    return f64::exp(-s.config.beta * hamiltoniana_ising(s));
}

impl Metropolis for Ising {
    fn p(&self) -> f64 {
        p_ising(self)
    }

    fn prop(&self) -> Self {
        let mut rng = rand::rng();
        let mut prop = self.clone();
        let idx_flip = rng.random_range(0..self.lambda.x() * self.lambda.y());
        prop.lambda.data[idx_flip] = -prop.lambda.data[idx_flip];
        prop
    }
}

fn main() {
    let x = 100;
    let y = 100;

    let mut rng = rand::rng();
    let mut sys = Ising {
        lambda: PBCGrid::new_from_vec(
            x,
            y,
            (0..x * y)
                .into_iter()
                .map(|_| Spin::new(rng.random()))
                .collect::<Vec<Spin>>(),
        ),
        config: Config {
            h: 1.0,
            j: 1.0,
            beta: 100.0,
        },
    };

    let mut res = vec![sys.clone()];

    for _ in 0..300000 {
        sys = metropolis_pass(sys, rand::rng());
        res.push(sys.clone());
    }

    let mut image = File::create("result.gif").unwrap();
    let mut enc =
        Encoder::new(&mut image, x as u16, y as u16, &[0xFF, 0xFF, 0xFF, 0, 0, 0]).unwrap();

    enc.set_repeat(gif::Repeat::Infinite).unwrap();
    for s in res.iter().step_by(500) {
        let mut frame = Frame::default();
        frame.height = x as u16;
        frame.width = y as u16;

        let pixels: Vec<u8> = s
            .lambda
            .data
            .iter()
            .map(|x| if f64::from(*x) == 1.0 { 1 } else { 0 })
            .collect();
        frame.delay = 1;
        frame.buffer = std::borrow::Cow::Owned(pixels);

        enc.write_frame(&frame).unwrap();
    }

    println!(
        "m_i {} m_f {}",
        res[0]
            .lambda
            .data
            .iter()
            .map(|x| f64::from(*x))
            .sum::<f64>(),
        res[res.len() - 1]
            .lambda
            .data
            .iter()
            .map(|x| f64::from(*x))
            .sum::<f64>()
    )
}
