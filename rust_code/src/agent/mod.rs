
static DEBUG: bool = false;


use rand::Rng;
use chess_engine::*;
use crate::environment::*;
use std::collections::HashMap;

mod experience;
use experience::Experience;

pub struct ChessAgent {
    pub playing_as: Color,
    pub experience: Experience,
    pub last_decision: GameState,
    pub foresight: i32,
    pub discount: f32,
    pub positions_evaluated: i32,
    pub exploration_propensity: f32,
    pub memory_purge_threshold: usize,
}

impl ChessAgent {
    pub fn new() -> ChessAgent {
        ChessAgent {
            playing_as: Color::White,
            experience: Experience {
                long_term_memory_file: "./experience".to_string(),
                value_map: HashMap::new(),
                memory_purge_threshold: 100_000,
            },
            last_decision: GameState::new(),
            foresight: 4,
            discount: 0.9,
            positions_evaluated: 0,
            exploration_propensity: 0.5,
            memory_purge_threshold: 100_000,
        }
    }

    pub fn will_explore(&self) -> bool {
        // Generate a random float between 0 and 1,
        // returning true if it's higher than the  agent's
        // propensity to explore, and false otherwise.
        let mut rng = rand::thread_rng();
        let random_float = rng.gen::<f32>();
        random_float - (random_float as i32) as f32 > self.exploration_propensity
    }

    pub fn rank_confidence_in_positions(&self, positions: &mut Vec<GameState>) {
        positions.sort_by(|a, b| {
            let value_a = self.experience.value_of(&a);
            let value_b = self.experience.value_of(&b);

            match self.playing_as {
                Color::White => value_a.partial_cmp(&value_b).unwrap(),
                Color::Black => value_b.partial_cmp(&value_a).unwrap(),
            }
        })
    }

    // Policy Function
    pub fn react(&mut self, environment: &ChessEnvironment) -> GameState {
        let decisions = environment.available_decisions();

        let mut best_decision = decisions[0];
        let mut best_value: f32 = -1.0;
        
        for decision in decisions.iter() {
            let next_environment = ChessEnvironment { state: *decision };
            let value_for_white = self.evaluate(&next_environment, 1);
            let value_for_self = self.value_from_own_perspective(value_for_white);

            if value_for_self > best_value {
                best_decision = *decision;
                best_value = value_for_self;
            }
        }
        
        self.last_decision = best_decision;
        best_decision
    }

    // Value Function / Bellman Equation
    pub fn evaluate(&mut self, environment: &ChessEnvironment, depth: i32) -> f32 {
        let (white_score, black_score) = relative_material_values(&environment.state);

        // Recursion base case
        if environment.is_terminated() {
            let value = match environment.terminal_state(self.playing_as) {
                TerminalState::Win => 1.0,
                TerminalState::Loss => -1.0,
                TerminalState::Draw => 0.0,
            };

            // Apply discount function
            return value * self.discount.powf(depth as f32)
        }

        // Recursion base case
        if depth == self.foresight {
            // Calculate expected value based on material, normalizing
            // between -1.0 and 1.0.
            let max_score = std::cmp::max(white_score, black_score) as f32;
            let value = (white_score as f32 - black_score as f32) / max_score;

            // Values are discounted based on their distance into the future.
            // This accounts for uncertainty, and the represents the idea that
            // it's high probability reward now is usually more valueable than
            // lower probability reward later.
            let discounted_value = value * self.discount.powf(depth as f32);
            return discounted_value;
        }

        let mut decisions = environment.available_decisions();
        let mut decision_index_to_evaluate = 0;


        // Choose between exploration / exploitation
        if self.will_explore() {
            // Chose a random next position to explore
            let mut rng = rand::thread_rng();
            decision_index_to_evaluate = rng.gen_range(0, decisions.len());
        }

        else {
            // Exploit the position that's most familiar / confident
            self.rank_confidence_in_positions(&mut decisions);
        }

        let next_state = ChessEnvironment {
            state: decisions[decision_index_to_evaluate],
        };

        // The value of the current state, discounting for distance into the future
        let expected_value = self.experience.value_of(&environment.state);
        let discounted_value = expected_value * self.discount.powf(depth as f32);

        // Define the current value in terms of the value of the next state
        let value_of_next_state = self.evaluate(&next_state, depth + 1);
        let value = (discounted_value + value_of_next_state) / 2.0;

        self.positions_evaluated += 1;
        if DEBUG {
            // Print Debugging Info
            println!("#{}", self.positions_evaluated);
            println!("{}", environment.state.to_string());

            println!("W/B Material: {}/{}", white_score, black_score);
            println!("expected_value: {}", expected_value);
            println!("discounted_value: {}", discounted_value);
            println!("recursive_value: {}\n", value);
        }

        self.experience.memorize(&environment, value);
        value
    }

    fn value_from_own_perspective(&self, value: f32) -> f32 {
        match self.playing_as {
            Color::White => value,
            Color::Black => value * -1.0,
        }
    }
}

