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

// Eventually, it would be better to use a numeralized
// gamestate, or PGN chess notation as the hash. For now,
// the primary obstacle to numeralization is Rust's problem
// with arrays > 32 elements long.
pub fn hash_gamestate(state: &GameState) -> String {
    format!("{:?}", state)
}


struct ChessEnvironment {
    pub state: GameState,
}

struct ChessAgent {
    associations: HashMap<String, i32>,
    last_decision: GameState,
    foresight: i32,
    discount: f32,
    positions_evaluated: i32,
    exploration_propensity: f32,
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
            exploration_propensity: 0.5,
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
            let hashed_a = hash_gamestate(&a);
            let hashed_b = hash_gamestate(&b);
            let value_a = self.associations.get(&hashed_a);
            let value_b = self.associations.get(&hashed_b);
            value_a.partial_cmp(&value_b).unwrap()
        })
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

        let mut decisions = environment.available_decisions();
        let mut decision_index_to_evaluate = 0;

        // Choose between exploration / exploitation
        if self.will_explore() {
            // Chose a random next position to evaluate
            let mut rng = rand::thread_rng();
            decision_index_to_evaluate = rng.gen_range(0, decisions.len());
        }

        else {
            self.rank_confidence_in_positions(&mut decisions);
        }

        let next_state = ChessEnvironment {
            state: decisions[decision_index_to_evaluate],
        };

        let value_of_next_state = self.evaluate(&next_state, depth + 1);
        discounted_value + value_of_next_state
    }
}


