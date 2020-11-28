use rand::distributions::{Distribution, Uniform};
use crate::scorecard::{Scorecard, new_scorecard};

#[derive(Debug)]
pub struct YahtzeeGame{
    pub roll: Vec<u8>,
    pub scorecard: Scorecard
}


pub fn new_game() -> YahtzeeGame{
    let roll: Vec<u8> = vec![0, 0, 0, 0, 0];
    let game = YahtzeeGame {
        roll,
        scorecard: new_scorecard()
    };
    game
}

pub fn roll(game: &mut YahtzeeGame){
    let mut rng = rand::thread_rng();
    let die_roller = Uniform::from(1..7);

    for i in 0..game.roll.len() {
        game.roll[i] = die_roller.sample(&mut rng);
    }
}

pub fn score_roll(game: & YahtzeeGame) -> Scorecard{
    Scorecard{
        aces: sum_if(&game.roll, 1),
        twos: sum_if(&game.roll, 2),
        threes: sum_if(&game.roll, 3),
        fours: sum_if(&game.roll, 4),
        fives: sum_if(&game.roll, 5),
        sixes: sum_if(&game.roll, 6),
        three_of_a_kind: 0,
        four_of_a_kind: 0,
        full_house: false,
        sm_straight: false,
        lg_straight: false,
        yahtzee: false,
        chance: 0,
        yahtzee_bonus_count: 0
    }
}

fn sum_if(vec: &Vec<u8>, x: u8) -> u8 {
    let mut sum: u8 = 0;
    for v in vec.iter() {
        if *v == x {
            sum += v;
        }
    }
    sum
}