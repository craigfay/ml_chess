mod vectors;

extern crate rand;

use std::collections::HashMap;
use rand::Rng;

use chess_engine::*;
use vectors::*;


pub fn training_pipeline(cycles: i32) {
    let mut agent = ChessAgent::new();
    let mut environment = ChessEnvironment::new();

    for _ in 0..cycles {

        if environment.is_terminated() {
            break;
        }
        
        // Does the agent experience reward only after committing to
        // a decision? Can it hypothesize?
        
        let decision = agent.react(&environment);
        let consequences = environment.apply(decision);

        // agent.reward(&consequences);
    }
}

pub fn main() {
    training_pipeline(40);
}


struct ChessEnvironment {
    pub state: GameState,
}

struct ChessAgent {
    associations: HashMap<NumericGameState, i32>,
    last_decision: GameState,
    foresight: i32,
    discount: f32,
    positions_evaluated: i32,
}

enum TerminalState {
    Win, 
    Loss,
    Draw,
}

impl ChessEnvironment {
    pub fn new() -> ChessEnvironment {
        ChessEnvironment {
            state: GameState::new(),
        }
    }

    pub fn apply(&mut self, decision: GameState) {
        // Apply agent move
        self.state = decision;

        if !self.is_terminated() {
            // Apply random opponent move
            let decisions = self.available_decisions();
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0, decisions.len());
            self.state = decisions[random_index];
        }
    }

    pub fn available_decisions(&self) -> Vec<GameState> {
        legal_next_states(&self.state)
    }

    pub fn is_terminated(&self) -> bool {
        is_checkmate(&self.state) || is_stalemate(&self.state)
    }

    pub fn terminal_state(&self) -> TerminalState {
        if is_checkmate(&self.state) {
            if self.state.to_move == Color::White {
                return TerminalState::Loss;
            }
            else {
                return TerminalState::Win;
            }
        }
        return TerminalState::Draw;
    }
}


impl ChessAgent {
    pub fn new() -> ChessAgent {
        ChessAgent {
            associations: HashMap::new(),
            last_decision: GameState::new(),
            foresight: 4,
            discount: 0.9,
            positions_evaluated: 0,
        }
    }

    // Policy Function
    pub fn react(&mut self, environment: &ChessEnvironment) -> GameState {
        let decisions = environment.available_decisions();

        let mut best_decision = decisions[0];
        let mut best_value: f32 = -1.0;
        
        for decision in decisions.iter() {
            let next_environment = ChessEnvironment { state: *decision };
            let value = self.evaluate(&next_environment, 1);
            if value > best_value {
                best_decision = *decision;
                best_value = value;
            }

        }
        
        self.last_decision = best_decision;
        best_decision
    }

    // Value Function / Bellman Equation
    pub fn evaluate(&mut self, environment: &ChessEnvironment, depth: i32) -> f32 {
        self.positions_evaluated += 1;

        println!("#{}", self.positions_evaluated);
        println!("{}", environment.state.to_string());

        // Discount function
        let (white_score, black_score) = relative_material_values(&environment.state);

        // Normalizing our value between -1.0 and 1.0.
        let max_score = std::cmp::max(white_score, black_score) as f32;
        let value = (white_score as f32 - black_score as f32) / max_score;

        // Values are discounted based on their distance into the future.
        // This accounts for uncertainty, and the represents the idea that
        // it's high probability reward now is usually more valueable than
        // lower probability reward later.
        let discounted_value = value * self.discount.powf(depth as f32);

        println!("material: {}/{}", white_score, black_score);
        println!("value: {}", value);
        println!("discounted_value: {}\n", discounted_value);

        // Recursion base case
        if environment.is_terminated() {
            let value = match environment.terminal_state() {
                TerminalState::Win => 1.0,
                TerminalState::Loss => -1.0,
                TerminalState::Draw => 0.0,
            };

            // Apply discount function
            return value * self.discount.powf(depth as f32)
        }

        // Recursion base case
        if depth == self.foresight {
            return discounted_value;
        }

        // Chose a random next position to evaluate
        let decisions = environment.available_decisions();
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0, decisions.len());

        let next_state = ChessEnvironment {
            state: decisions[random_index],
        };

        let value_of_next_state = self.evaluate(&next_state, depth + 1);
        discounted_value + value_of_next_state
    }
}


