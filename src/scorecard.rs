#[derive(Debug)]
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
