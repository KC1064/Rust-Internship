# Activity Week 1: Advanced 2-Player Dice Game

## Overview

This Rust program simulates an engaging 2-player dice game. Throughout the game, players alternate turns to roll two 6-sided dice. Their scores are updated according to the outcomes, incorporating special events that can either boost their points or impose penalties based on specific dice roll outcomes. The game progresses through a predetermined number of rounds, culminating in the announcement of the winner.

## Game Instructions

### Variables and Data Types

- **Scores Array:** Utilize a mutable integer array `scores` with a length of 2 to track the scores of both players, initially set to `[0, 0]`.
- **Number of Rounds:** Define a constant `NUM_ROUNDS` with a value of `10` to set the total number of rounds in the game.

### Core Functions

1. **`roll_dice` Function:** This function simulates rolling two 6-sided dice and returns a tuple containing two integers representing the outcome.
2. **`update_scores` Function:** Accepts the current scores array and the results from the dice roll, then returns the updated scores.
3. **`determine_winner` Function:** Analyzes the final scores array to print the winner's name or declare a tie if applicable.
4. **`special_event` Function:** Manages special events triggered by specific dice roll outcomes, returning a score modifier as a tuple.

### Game Rules

1. **Automatic Rounds:** The game automatically progresses through rounds without requiring player prompts to roll the dice.
2. **Random Dice Rolls:** Dice roll values are generated randomly.
3. **Scoring:** A player's score for a round is the sum of the dice roll values plus any bonuses from special scenarios.
4. **Winning Condition:** The player with the highest score at the end of all rounds is declared the winner.
5. **Learning Application:** Ensure to apply all concepts learned so far. The game's design justifies the use of every concept introduced.

### Important Note

- **Do Not Modify the `roll_dice` Function:** The `roll_dice` function is provided to you as part of the game's setup and should not be altered.
