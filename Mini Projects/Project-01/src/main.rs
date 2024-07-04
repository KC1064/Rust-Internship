use rand::Rng;

const NUM_ROUNDS: usize = 10;

fn main() {
    let mut scores = [0, 0];
    for i in 0..NUM_ROUNDS {
        // Player 1 rolls
        let roll1 = roll_dice();
        let roll1 = special_event(roll1);
        scores = update_scores(scores, roll1);

        // Player 2 rolls
        let roll2 = roll_dice();
        let roll2 = special_event(roll2);
        scores = update_scores(scores, roll2);

        println!("---------------------ROUND:{}-----------------", i + 1);
        println!("Player 1: {}", scores[0]);
        println!("Player 2: {}", scores[1]);
    }
    determine_winner(scores);
}

fn update_scores(prev_scores: [i32; 2], current_scores: (i32, i32)) -> [i32; 2] {
    [
        prev_scores[0] + current_scores.0,
        prev_scores[1] + current_scores.1
    ]
}

fn determine_winner(scores: [i32; 2]) {
    if scores[0] > scores[1] {
        println!("Player 1 wins with a score of {}", scores[0]);
    } else if scores[1] > scores[0] {
        println!("Player 2 wins with a score of {}", scores[1]);
    } else {
        println!("It's a tie with both players scoring {}", scores[0]);
    }
}

fn special_event(roll: (i32, i32)) -> (i32, i32) {
    if roll.0 == roll.1 {
        // Both dice have the same number then +5
        return (roll.0 + 5, roll.1 + 5);
    } else if roll.0 + roll.1 <= 3 {
        // Sum of dice is less than or equal to 3 then -1 for each dice
        return (roll.0 - 1, roll.1 - 1);
    }
    roll
}

// DO NOT CHANGE
fn roll_dice() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(1..=6), rng.gen_range(1..=6))
}
