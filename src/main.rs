mod yahtzee;
mod scorecard;
mod test;
mod engine;
mod ai_keep_first_roll;

use crate::yahtzee::{new_game, roll};
use crate::scorecard::{new_scorecard, get_highest_scores};
use std::time::Instant;
use crate::engine::score_roll;
use crate::ai_keep_first_roll::ai_play;

fn main(){
    const GAMES: usize = 1000;
    let mut scores: Vec<u32> = vec![0; GAMES];
    for i in 0..GAMES {
        let mut game = new_game();
        let score = ai_play(&mut game) as u32;
        scores[i] = score;
        //println!("Score {}: {}", i, score);
    }
    scores.sort();
    let sorted = scores.clone();
    let last = scores.last().unwrap().clone();
    let first = scores[0];
    println!("Average Score: {}", mean(scores));
    println!("Best Score: {}", last);
    println!("Worse Score: {}", first);
    println!("{:?}", sorted);
}

fn mean(list: Vec<u32>) -> f64 {
    let sum = list.iter().sum::<u32>();
    f64::from(sum) / (list.len() as f64)
}

fn simulate_rolls_main() {
    let mut game = new_game();
    roll(&mut game);
    println!("{:?}", game);
    println!("Scoring Roll...");
    println!("{:?}", score_roll(&game.roll));

    const ROLLS: usize = 100000;
    println!("Simulating rolls for {} rolls", ROLLS);

    let now = Instant::now();
    let mut scores = vec![new_scorecard(); ROLLS];
    for i in 0..ROLLS {
        roll(&mut game);
        scores[i] = score_roll(&game.roll)
    }
    println!("Simulated {} Rolls. Completed in {}ms", ROLLS, now.elapsed().as_millis());

    let mut three_of_a_kind = 0;
    let mut four_of_a_kind = 0;
    let mut full_house = 0;
    let mut sm_straight = 0;
    let mut lg_straight = 0;
    let mut yahtzee_count = 0;
    for card in scores.iter(){
        if card.three_of_a_kind > 0{
            three_of_a_kind += 1;
        }
        if card.four_of_a_kind > 0{
            four_of_a_kind += 1;
        }
        if card.full_house{
            full_house += 1;
        }
        if card.sm_straight{
            sm_straight += 1;
        }
        if card.lg_straight{
            lg_straight += 1;
        }
        if card.yahtzee{
            yahtzee_count += 1;
        }
    }
    println!("Three of a kind Count: {}", three_of_a_kind);
    println!("Four of a kind Count: {}", four_of_a_kind);
    println!("Full House Count: {}", full_house);
    println!("Small Straight Count: {}", sm_straight);
    println!("Large Straight Count: {}", lg_straight);
    println!("Yahtzee Count: {}", yahtzee_count);
}
