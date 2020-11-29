use rand::distributions::{Distribution, Uniform};
use crate::scorecard::{Scorecard, new_scorecard, get_score_by_index, YAHTZEE_BONUS, get_highest_scores, YAHTZEE, set_score_by_index};
use std::collections::HashSet;

#[derive(Debug)]
pub struct YahtzeeGame{
    pub roll: Vec<u8>,
    pub scorecard: Scorecard,
    pub scored_categories: HashSet<u8>,
    pub roll_num: u8
}


pub fn new_game() -> YahtzeeGame{
    let roll: Vec<u8> = vec![0, 0, 0, 0, 0];
    let game = YahtzeeGame {
        roll,
        scorecard: new_scorecard(),
        scored_categories: HashSet::new(),
        roll_num: 0
    };
    game
}

pub fn roll(game: &mut YahtzeeGame){
    if game.roll_num != 0{
        panic!("Roll already in progress. Roll # {}. Score current roll before starting a new turn", game.roll_num);
    }
    game.roll_num = 1;
    let mut rng = rand::thread_rng();
    let die_roller = Uniform::from(1..7);

    for i in 0..game.roll.len() {
        game.roll[i] = die_roller.sample(&mut rng);
    }
}

pub fn re_roll(game: &mut YahtzeeGame, hold_die: [bool; 5]){
    game.roll_num += 1;
    if game.roll_num > 3 {
        panic!("Cannot re-roll. Attempting roll # {}", game.roll_num);
    }

    let mut rng = rand::thread_rng();
    let die_roller = Uniform::from(1..7);

    for i in 0..hold_die.len() {
        if !hold_die[i]{
            game.roll[i] = die_roller.sample(&mut rng);
        }
    }
}

pub fn record_score(game: &mut YahtzeeGame, roll_score: &Scorecard, score_index:u8) {
    if score_index == YAHTZEE_BONUS{
        game.roll_num = 0;
        game.scorecard.yahtzee_bonus_count += 1;
        record_highest_non_yahtzee_score(game, roll_score);
        return;
    }

    if !game.scored_categories.insert(score_index) {
        panic!("Already scored for specified score_index {}", score_index);
    }
    game.roll_num = 0;
    let score = get_score_by_index(roll_score, score_index);
    set_score_by_index(&mut game.scorecard, score, score_index);
}

pub fn record_highest_non_yahtzee_score(game: &mut YahtzeeGame, roll_score: &Scorecard){
    let sorted_scores = get_highest_scores(roll_score);
    for i in 0..sorted_scores.len(){
        if sorted_scores[i].0 == YAHTZEE{
            continue;
        }

        if !game.scored_categories.contains(&sorted_scores[i].0){
            record_score(game, roll_score, sorted_scores[i].0);
            return;
        }
    }
    panic!("Unable to score highest");
}

pub fn is_game_over(game: &mut YahtzeeGame) -> bool{
    game.scored_categories.len() == 13
}