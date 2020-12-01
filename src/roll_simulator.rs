pub fn simulate_rolls(){
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
        game.roll_num = 0; //Reset roll number for sim
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
    println!("--------------------------------------");
}