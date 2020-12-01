mod yahtzee;
mod scorecard;
mod test;
mod engine;
mod ai_keep_first_roll;
mod simulation;

use crate::yahtzee::{new_game, roll};
use crate::scorecard::{new_scorecard};
use std::time::Instant;
use crate::engine::score_roll;
use crate::ai_keep_first_roll::ai_play;
use std::env;
use crate::simulation::{new_simulation, run_sim, print_sim_results};

fn main(){
    const GAMES: usize = 1000;
    let mut sim = new_simulation(GAMES, ai_play);
    run_sim(&mut sim);
    print_sim_results(&mut sim);
}