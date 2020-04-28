mod vectors;

use std::collections::HashMap;
use chess_engine::*;
use vectors::*;


pub fn training_pipeline(cycles: i32) {
    let mut agent = ChessAgent::new();
    let mut environment = ChessEnvironment::new();

    for _ in 0..cycles {
        // evaluate()?
        
        // Does the agent experience reward only after committing to
        // a decision? Can it hypothesize?
        
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
    last_decision: GameState,
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
            last_decision: GameState::new(),
        }
    }

    pub fn react(&mut self, environment: &ChessEnvironment) -> GameState {
        let decisions = environment.available_decisions();

        let best_decision: Option<GameState> = None;
        
        for decision in decisions.iter() {
            println!("{}", decision.to_string());
        }
        //self.last_decision = decisions[0].clone();
        decisions[0]
    }

    pub fn evaluate(&mut self, environment: &ChessEnvironment) -> GameState {
        let decisions = environment.available_decisions();
        self.last_decision = decisions[0].clone();
        decisions[0]
    }
}


