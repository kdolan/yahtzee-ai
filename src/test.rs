#[cfg(test)]
mod tests {
    use crate::yahtzee::{new_game};
    use crate::engine::score_roll;

    #[test]
    fn lg_straight() {
        let mut game = new_game();
        game.roll = vec![1, 2, 3, 4, 5];
        let score = score_roll(&game.roll);
        assert_eq!(score.lg_straight, true);
    }

    #[test]
    fn sm_straight() {
        let mut game = new_game();
        game.roll = vec![1, 2, 3, 4, 1];
        let score = score_roll(&game.roll);
        assert_eq!(score.sm_straight, true);
    }

    #[test]
    fn yahtzee() {
        let mut game = new_game();
        game.roll = vec![5, 5, 5, 5, 5];
        let score = score_roll(&game.roll);
        assert_eq!(score.yahtzee, true);
        assert_eq!(score.chance, 25);
    }

    #[test]
    fn full_house() {
        let mut game = new_game();
        game.roll = vec![5, 5, 5, 2, 2];
        let score = score_roll(&game.roll);
        assert_eq!(score.full_house, true);
    }
}
