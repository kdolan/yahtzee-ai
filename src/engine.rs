use crate::scorecard::{DieCountScores, Scorecard};

pub fn score_roll(roll: &Vec<u8>) -> Scorecard{
    let mut aces:u8 = 0;
    let mut twos:u8 = 0;
    let mut threes:u8 = 0;
    let mut fours:u8 = 0;
    let mut fives:u8 = 0;
    let mut sixes:u8 = 0;
    let mut die_total:u8 = 0;

    let mut sorted_vec = vec![0, 0, 0, 0, 0];
    for i in 0..roll.len() {
        let v = roll[i];
        sorted_vec[i] = v;

        if v == 1 {
            aces += v;
        }
        else if v == 2 {
            twos += v;
        }
        else if v == 3 {
            threes += v;
        }
        else if v == 4 {
            fours += v;
        }
        else if v == 5 {
            fives += v;
        }
        else if v == 6 {
            sixes += v;
        }
        die_total += v;
    }
    sorted_vec.sort();
    let die_count = get_die_count(aces, twos, threes, fours, fives, sixes);
    let die_count_scores = get_die_count_based_scores(&die_count);

    Scorecard{
        aces,
        twos,
        threes,
        fours,
        fives,
        sixes,
        three_of_a_kind: if die_count_scores.is_three_of_a_kind { die_total } else { 0 },
        four_of_a_kind: if die_count_scores.is_four_of_a_kind { die_total } else { 0 },
        full_house: die_count_scores.is_full_house,
        sm_straight: is_small_straight(&sorted_vec),
        lg_straight: is_large_straight(&sorted_vec),
        yahtzee: die_count_scores.is_five_of_a_kind,
        chance: die_total,
        yahtzee_bonus_count: 0
    }
}

fn is_small_straight(sorted_vec: &Vec<u8>) -> bool{
    let mut vec_no_dups: Vec<u8> = sorted_vec.to_vec();
    vec_no_dups.dedup();
    if vec_no_dups.len() < 4 {
        return false;
    }
    let first_vale = vec_no_dups[0];
    for i in 1..vec_no_dups.len(){
        let iu8: u8 = i as u8;
        if vec_no_dups[i] != first_vale + iu8{
            return false;
        }
    }
    true
}

fn is_large_straight(sorted_vec: &Vec<u8>) -> bool{
    for i in 0..sorted_vec.len() {
        let loop_i: u8 = i as u8;
        if loop_i + 1 != sorted_vec[i] {
            return false;
        }
    }
    true
}

fn get_die_count(aces: u8, twos: u8, threes: u8, fours: u8, fives: u8, sixes: u8) -> [u8; 6] {
    [aces, twos / 2, threes / 3, fours / 4, fives / 5, sixes / 6]
}

fn get_die_count_based_scores(die_count:&[u8; 6]) -> DieCountScores{
    let mut is_two_of_a_kind = false;
    let mut is_three_of_a_kind = false;
    let mut is_four_of_a_kind = false;
    let mut is_five_of_a_kind = false;
    for i in 0..die_count.len(){
        let v: u8 = die_count[i];
        if v == 2 {
            is_two_of_a_kind = true;
        }
        if v == 3{
            is_three_of_a_kind = true;
        }
        if v == 4{
            is_four_of_a_kind = true;
        }
        if v == 5 {
            is_five_of_a_kind = true;
        }
    }
    DieCountScores {is_two_of_a_kind, is_three_of_a_kind, is_four_of_a_kind, is_five_of_a_kind, is_full_house: is_two_of_a_kind && is_three_of_a_kind}
}