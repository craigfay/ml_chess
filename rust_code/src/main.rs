use std::collections::HashMap;
use chess_engine::*;

pub fn trainingPipeline(cycles: i32) {
    let mut agent = ChessAgent::new();
    let mut environment = ChessEnvironment::new();

    for i in 0..cycles {
        let decision = agent.react(&environment);
        let consequences = environment.apply(decision);
    }
}

pub fn main() {
    trainingPipeline(5);
}


struct ChessEnvironment {
    pub state: GameState,
}

struct ChessAgent {
    // associations: HashMap<ChessEnvironment, i32>
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
            // associations: HashMap::new(),
        }
    }
    pub fn react(&self, environment: &ChessEnvironment) -> GameState {
        let decisions = environment.available_decisions();
        return decisions[0];
    }
}


