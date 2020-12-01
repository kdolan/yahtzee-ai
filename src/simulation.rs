use crate::yahtzee::{new_game, YahtzeeGame};
pub struct Simulation{
    games: usize,
    scores: Vec<u32>,
    sorted_scores: Vec<u32>,
    average: f64,
    best: u32,
    worst: u32,
    ai: fn(&mut YahtzeeGame) -> u16
}

pub fn new_simulation(games: usize, ai: fn(&mut YahtzeeGame) -> u16) -> Simulation{
    Simulation{
        games,
        scores: vec![0; games],
        sorted_scores: vec![0;0],
        average: 0.0,
        best: 0,
        worst: 0,
        ai: ai
    }
}

pub fn run_sim(sim: &mut Simulation){
    for i in 0..sim.games {
        let mut game = new_game();
        let score = (sim.ai)(&mut game) as u32;
        sim.scores[i] = score;
    }
}

pub fn print_sim_results(sim: &mut Simulation){
    compute_results(sim);
    println!("Average Score: {}", sim.average);
    println!("Best Score: {}", sim.best);
    println!("Worse Score: {}", sim.worst);
    println!("{:?}", sim.sorted_scores);
}

pub fn compute_results(sim: &mut Simulation){
    sim.sorted_scores = sim.scores.clone();
    sim.sorted_scores.sort();
    sim.best = sim.sorted_scores.last().unwrap().clone();
    sim.worst = sim.sorted_scores[0];
    sim.average = mean(&sim.scores);
}

fn mean(list: &Vec<u32>) -> f64 {
    let sum = list.iter().sum::<u32>();
    f64::from(sum) / (list.len() as f64)
}