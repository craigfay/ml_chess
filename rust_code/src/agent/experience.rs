
use std::collections::HashMap;
use chess_engine::*;
use crate::environment::*;

// Serialization Libs
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;
use serde::{Serialize, Deserialize};



#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Recollection {
    pub times_encountered: i32,
    pub average_value: f32,
}

impl Recollection {
    fn new() -> Recollection {
        Recollection {
            times_encountered: 0,
            average_value: 0.0,
        }
    }
}

pub struct Experience {
    long_term_memory_directory: String,
    value_map: HashMap<String, Recollection>
}


impl Experience {
    pub fn new(filename: &str) -> Experience {
        Experience {
            long_term_memory_directory: filename.to_string(),
            value_map: HashMap::new(),
        }
    }

    pub fn value_of(&self, state: &GameState) -> f32 {
        self.value_map
            .get(&hash_gamestate(&state))
            .unwrap_or(&Recollection::new())
            .average_value
    }

    pub fn memorize(&mut self, environment: &ChessEnvironment, value: f32) {
        let hash = hash_gamestate(&environment.state);
        let recollection = match self.value_map.get(&hash) {
            None => Recollection::new(),
            Some(r) => *r,
        };

        let revised_recollection = Recollection {
            times_encountered: recollection.times_encountered + 1,
            // Use bitwise or to prevent dividing by zero
            average_value: (recollection.average_value + value) / (recollection.times_encountered | 1) as f32,
        };

        // For the time being, we won't remember neutral experiences
        if revised_recollection.average_value != 0.0 || recollection.average_value != 0.0 {
            let hash = hash_gamestate(&environment.state);
            self.value_map.insert(hash, revised_recollection);
        }
    }

    // Write experiences to file
    pub fn persist_experiences(&self) {
        let filename = &self.long_term_memory_directory;

        let pretty = PrettyConfig {
            new_line: "\n".to_string(),
            indentor: "    ".to_string(),
            depth_limit: 4,
            separate_tuple_members: true,
            enumerate_arrays: true,
        };

        let text = to_string_pretty(&self.value_map, pretty).expect("Serialization failed");
        std::fs::write(filename, text).expect("Unable to write file");
    }
    
    pub fn retrieve_persisted_experiences(&mut self) {
        let filename = &self.long_term_memory_directory;
        let text = match std::fs::read_to_string(filename) {
            Ok(t) => t,
            Err(_) => return,
        };
        match from_str(&text) {
            Ok(map) => self.value_map = map,
            Err(_) => return,
        };
    }

}

// Eventually, it would be better to use a numeralized
// gamestate, or PGN chess notation as the hash. For now,
// the primary obstacle to numeralization is Rust's problem
// with arrays > 32 elements long.
pub fn hash_gamestate(state: &GameState) -> String {
    fen_notation(&state)
}

