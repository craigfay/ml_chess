
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



