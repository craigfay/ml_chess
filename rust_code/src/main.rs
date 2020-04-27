mod vectors;

use std::collections::HashMap;
use chess_engine::*;
use vectors::*;


pub fn training_pipeline(cycles: i32) {
    let mut agent = ChessAgent::new();
    let mut environment = ChessEnvironment::new();

    for i in 0..cycles {
        let decision = agent.react(&environment);
        let consequences = environment.apply(decision);

        // agent.reward(&consequences);
        //println!("{:?}", numeralize_gamestate(&environment.state));
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
    last_decision: NumericGameState,
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
}


impl ChessAgent {
    pub fn new() -> ChessAgent {
        ChessAgent {
            associations: HashMap::new(),
            last_decision: numeralize_gamestate(&GameState::new()),
        }
    }
    pub fn react(&self, environment: &ChessEnvironment) -> GameState {
        let decisions = environment.available_decisions();
        return decisions[0];
    }
}


