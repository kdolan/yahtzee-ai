use crate::yahtzee::{YahtzeeGame, roll, record_score, is_game_over};
use crate::engine::score_roll;
use crate::scorecard::{get_highest_scores, get_total_score};

pub fn ai_play(game: &mut YahtzeeGame) -> u16{
    while !is_game_over(game){
        roll(game);
        let result = record_best_score(game);
        //println!("Scored index {} for {} points", result.0, result.1);
    }
    //println!("Game Final Score: {}", get_total_score(&game.scorecard));
    //println!("Game Results: {:?}", game);
    get_total_score(&game.scorecard)
}

pub fn record_best_score(game: &mut YahtzeeGame) -> (u8, u8){
    let roll_score = score_roll(&game.roll);
    let best_scores = get_highest_scores(&roll_score);

    for i in 0..best_scores.len(){
        if !game.scored_categories.contains(&best_scores[i].0){
            record_score(game, &roll_score, best_scores[i].0);
            return best_scores[i];
        }
    }
    panic!("Failed to score any category");
}