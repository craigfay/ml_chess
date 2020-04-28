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
    training_pipeline(5);
}


struct ChessEnvironment {
    pub state: GameState,
}

struct ChessAgent {
    associations: HashMap<NumericGameState, i32>,
    last_decision: GameState,
    foresight: i32,
    discount: f32,
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
        self.state = decision;
        // ... apply random opponent move
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
        }
    }

    pub fn react(&mut self, environment: &ChessEnvironment) -> GameState {
        let decisions = environment.available_decisions();

        let mut best_decision = decisions[0];
        let mut best_value: f32 = -1.0;
        
        for decision in decisions.iter() {
            let next_environment = ChessEnvironment { state: *decision };
            let value = self.evaluate(&next_environment, 0);
            if value > best_value {
                best_decision = *decision;
                best_value = value;
            }

        }
        
        self.last_decision = best_decision;
        best_decision
    }

    pub fn evaluate(&mut self, environment: &ChessEnvironment, depth: i32) -> f32 {

        println!("{}", environment.state.to_string());

        // Discount function
        let (white_score, black_score) = relative_material_values(&environment.state);
        let value: f32 = white_score as f32 / black_score as f32;
        let discounted_value = value * self.discount.powf(depth as f32);

        // TODO normalize value in a way that makes wins non problematic
        if environment.is_terminated() {
            match environment.terminal_state() {
                TerminalState::Win => return f32::MAX,
                TerminalState::Loss => return 0.0,
                TerminalState::Draw => return 1.0,
            }
        }

        // Recursion base case
        if depth == self.foresight || environment.is_terminated() {
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


