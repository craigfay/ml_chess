mod vectors;

extern crate rand;

use std::collections::HashMap;
use rand::Rng;

use chess_engine::*;
use vectors::*;

static DEBUG: bool = true;

pub struct TrainingOptions {
    pub game_limit: i32,
    pub turn_limit: i32,
}

pub fn training_pipeline(options: TrainingOptions) {
    let mut agent = ChessAgent::new();

    for _ in 0..options.game_limit {

        let mut environment = ChessEnvironment::new();
        agent.playing_as = match agent.playing_as {
            Color::White => Color::Black,
            Color::Black=> Color::White,
        };

        for _ in 0..options.turn_limit {

            if environment.is_terminated() {
                break;
            }
            
            let decision = agent.react(&environment);
            environment.apply(decision);
        }
    }
}

pub fn main() {
    training_pipeline(TrainingOptions {
        game_limit: 5,
        turn_limit: 40,
    });
}

// Eventually, it would be better to use a numeralized
// gamestate, or PGN chess notation as the hash. For now,
// the primary obstacle to numeralization is Rust's problem
// with arrays > 32 elements long.
pub fn hash_gamestate(state: &GameState) -> String {
    fen_notation(&state)
}


struct ChessEnvironment {
    pub state: GameState,
}

struct ChessAgent {
    playing_as: Color,
    experiences: HashMap<String, Experience>,
    last_decision: GameState,
    foresight: i32,
    discount: f32,
    positions_evaluated: i32,
    exploration_propensity: f32,
}

#[derive(Copy, Clone, Debug)]
struct Experience {
    times_encountered: i32,
    average_value: f32,
}

impl Experience {
    fn new() -> Experience {
        Experience {
            times_encountered: 1,
            average_value: 0.0,
        }
    }
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
            playing_as: Color::White,
            experiences: HashMap::new(),
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
            let value_a = self.experiences.get(&hashed_a).unwrap_or(&Experience::new()).average_value;
            let value_b = self.experiences.get(&hashed_b).unwrap_or(&Experience::new()).average_value;

            match self.playing_as {
                Color::White => value_a.partial_cmp(&value_b).unwrap(),
                Color::Black => value_b.partial_cmp(&value_a).unwrap(),
            }
        })
    }

    pub fn recall_experience(&self, environment: &ChessEnvironment) -> Experience {
        let hash = hash_gamestate(&environment.state);
        match self.experiences.get(&hash) {
            None => Experience::new(),
            Some(experience) => *experience,
        }
    }

    pub fn memorize_experience(&mut self, environment: &ChessEnvironment, value: f32) {
        let hash = hash_gamestate(&environment.state);
        let experience = &self.recall_experience(&environment);

        let revised_experience = Experience {
            times_encountered: experience.times_encountered + 1,
            average_value: (experience.average_value + value) as f32 / experience.times_encountered as f32,
        };

        self.experiences.insert(hash, revised_experience);
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
        let expected_value = self.recall_experience(environment).average_value;
        let discounted_value = expected_value * self.discount.powf(depth as f32);

        // Define the current value in terms of the value of the next state
        let value_of_next_state = self.evaluate(&next_state, depth + 1);
        let value = (discounted_value + value_of_next_state) / 2.0;

        self.positions_evaluated += 1;
        if DEBUG {
            // Print Debugging Info
            println!("#{}", self.positions_evaluated);
            println!("{}", environment.state.to_string());
            println!("{}", hash_gamestate(&environment.state));

            println!("W/B Material: {}/{}", white_score, black_score);
            println!("expected_value: {}", expected_value);
            println!("discounted_value: {}", discounted_value);
            println!("recursive_value: {}\n", value);
        }

        self.memorize_experience(&environment, value);
        value
    }

    fn value_from_own_perspective(&self, value: f32) -> f32 {
        match self.playing_as {
            Color::White => value,
            Color::Black => value * -1.0,
        }
    }
}

