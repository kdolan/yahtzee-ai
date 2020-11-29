pub const ACES:u8 = 0;
pub const TWOS:u8 = 1;
pub const THREES:u8 = 2;
pub const FOURS:u8 = 3;
pub const FIVES:u8 = 4;
pub const SIXES:u8 = 5;
pub const THREE_OF_A_KIND:u8 = 6;
pub const FOUR_OF_A_KIND:u8 = 7;
pub const FULL_HOUSE:u8 = 8;
pub const SM_STRAIGHT:u8 = 9;
pub const LG_STRAIGHT:u8 = 10;
pub const YAHTZEE:u8 = 11;
pub const CHANCE:u8 = 12;
pub const YAHTZEE_BONUS:u8 = 13;

#[derive(Debug)]
#[derive(Clone)]
pub struct Scorecard{
    pub aces: u8,
    pub twos: u8,
    pub threes: u8,
    pub fours: u8,
    pub fives: u8,
    pub sixes: u8,
    pub three_of_a_kind: u8,
    pub four_of_a_kind: u8,
    pub full_house: bool,
    pub sm_straight: bool,
    pub lg_straight: bool,
    pub yahtzee: bool,
    pub chance: u8,
    pub yahtzee_bonus_count: u8
}

pub fn new_scorecard() -> Scorecard{
    Scorecard{
        aces: 0,
        twos: 0,
        threes: 0,
        fours: 0,
        fives: 0,
        sixes: 0,
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

pub struct DieCountScores{
    pub is_two_of_a_kind: bool,
    pub is_three_of_a_kind: bool,
    pub is_four_of_a_kind: bool,
    pub is_five_of_a_kind: bool,
    pub is_full_house: bool,
}

//Returns array of tuples (index, value) sorted by value greatest to least
pub fn get_highest_scores(score_card: &Scorecard) -> Vec<(u8, u8)>{
    let mut score_vec = get_tuple_vec_of_score_card(score_card);
    score_vec.sort_by_key(|i| i.1);
    score_vec.reverse();
    score_vec
}

pub fn get_tuple_vec_of_score_card(sore_card: &Scorecard) -> Vec<(u8, u8)>{
   vec![
        (ACES, get_score_by_index(sore_card, ACES)),
        (TWOS, get_score_by_index(sore_card, TWOS)),
        (THREES, get_score_by_index(sore_card, THREES)),
        (FOURS, get_score_by_index(sore_card, FOURS)),
        (FIVES, get_score_by_index(sore_card, FIVES)),
        (SIXES, get_score_by_index(sore_card, SIXES)),
        (THREE_OF_A_KIND, get_score_by_index(sore_card, THREE_OF_A_KIND)),
        (FOUR_OF_A_KIND, get_score_by_index(sore_card, FOUR_OF_A_KIND)),
        (FULL_HOUSE, get_score_by_index(sore_card, FULL_HOUSE)),
        (SM_STRAIGHT, get_score_by_index(sore_card, SM_STRAIGHT)),
        (LG_STRAIGHT, get_score_by_index(sore_card, LG_STRAIGHT)),
        (YAHTZEE, get_score_by_index(sore_card, YAHTZEE)),
        (CHANCE, get_score_by_index(sore_card, CHANCE)),
        //(YAHTZEE_BONUS, get_score_by_index(sore_card, YAHTZEE_BONUS)),
    ]
}

pub fn get_score_by_index(score_card: &Scorecard, index:u8) -> u8{
    match index {
        ACES => score_card.aces,
        TWOS => score_card.twos,
        THREES => score_card.threes,
        FOURS => score_card.fours,
        FIVES => score_card.fives,
        SIXES => score_card.sixes,
        THREE_OF_A_KIND => score_card.three_of_a_kind,
        FOUR_OF_A_KIND => score_card.four_of_a_kind,
        FULL_HOUSE => if score_card.full_house {25} else {0},
        SM_STRAIGHT => if score_card.sm_straight {30} else {0},
        LG_STRAIGHT => if score_card.lg_straight {40} else {0},
        YAHTZEE => if score_card.yahtzee {50} else {0},
        CHANCE => score_card.chance,
        YAHTZEE_BONUS => score_card.yahtzee_bonus_count * 100,
        _ => 0,
    }
}

pub fn set_score_by_index(score_card: &mut Scorecard, score:u8, index:u8){
    match index {
        ACES => score_card.aces = score,
        TWOS => score_card.twos= score,
        THREES => score_card.threes = score,
        FOURS => score_card.fours = score,
        FIVES => score_card.fives = score,
        SIXES => score_card.sixes = score,
        THREE_OF_A_KIND => score_card.three_of_a_kind = score,
        FOUR_OF_A_KIND => score_card.four_of_a_kind = score,
        FULL_HOUSE => score_card.full_house = score != 0,
        SM_STRAIGHT => score_card.sm_straight = score != 0,
        LG_STRAIGHT => score_card.lg_straight = score != 0,
        YAHTZEE => score_card.yahtzee = score != 0,
        CHANCE => score_card.chance = score,
        _ => panic!("Invalid score index")
    }
}

pub fn get_upper_score(score_card: &Scorecard) -> u16{
    get_raw_upper_score(score_card) + get_upper_bonus(score_card)
}

pub fn get_raw_upper_score(score_card: &Scorecard) -> u16{
    (score_card.aces as u16 + score_card.twos as u16 + score_card.threes as u16 + score_card.fours as u16 + score_card.fives as u16 + score_card.sixes as u16) as u16
}

pub fn get_upper_bonus(score_card: &Scorecard) -> u16{
    if get_raw_upper_score(score_card) >= 63{
       return 35;
    }
    0
}

pub fn get_lower_score(score_card: &Scorecard)-> u16{
    let mut score:u16 = (score_card.three_of_a_kind as u16 + score_card.four_of_a_kind as u16 + score_card.chance as u16) as u16;
    if score_card.full_house{
        score += 25;
    }
    if score_card.sm_straight{
        score += 30;
    }
    if score_card.lg_straight{
        score += 40;
    }
    if score_card.yahtzee{
        score += 50;
    }
    score += score_card.yahtzee_bonus_count as u16 * 100;
    score
}

pub fn get_total_score(score_card: &Scorecard)-> u16{
    get_upper_score(score_card) + get_lower_score(score_card)
}