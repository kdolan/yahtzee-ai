mod yahtzee;
mod scorecard;
mod test;

use crate::yahtzee::{new_game, roll, score_roll};

fn main() {
    let mut game = new_game();
    roll(&mut game);
    println!("{:?}", game);
    println!("Scoring Roll...");
    println!("{:?}", score_roll(&game));

    /*loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }*/
}
