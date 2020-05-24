
use chess_engine::*;
use rand::Rng;


pub struct ChessEnvironment {
    pub state: GameState,
}

pub enum TerminalState {
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

    // Set the environment state to a given state,
    // presumably (but not necessarily) one of the
    // legal next states based on the current state.
    pub fn apply_change(&mut self, state: GameState) {
        self.state = state;
    }

    // Set the environment state to a random available
    // next state.
    pub fn apply_change_randomly(&mut self) {
        if !self.is_terminated() {
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

    pub fn terminal_state(&self, perspective: Color) -> TerminalState {
        if is_checkmate(&self.state) {
            return match self.state.to_move == perspective {
                false => TerminalState::Win,
                true => TerminalState::Loss,
            };
        }
        TerminalState::Draw
    }
}



