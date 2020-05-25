
use std::collections::HashMap;
use std::path::Path;
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
        let hash = hash_gamestate(&state);

        // Short Term Memory
        match self.value_map.get(&hash) {
            Some(rec) => return rec.average_value,
            None => (),
        }

        // Long Term Memory
        match self.long_term_recall(&hash) {
            Some(rec) => return rec.average_value,
            None => (),
        }

        Recollection::new().average_value
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
            self.value_map.insert(hash.to_string(), revised_recollection);
            self.long_term_memorize(&hash, &revised_recollection);
        }
    }

    // Write experiences to file
    pub fn long_term_memorize(&self, hash: &str, rec: &Recollection) {
        let filename = Path::new(&self.long_term_memory_directory)
            .join(format!("{}.exp", &hash));

        let pretty = PrettyConfig {
            new_line: "\n".to_string(),
            indentor: "    ".to_string(),
            depth_limit: 4,
            separate_tuple_members: true,
            enumerate_arrays: true,
        };

        let text = to_string_pretty(&rec, pretty).expect("Serialization failed");
        std::fs::write(filename.as_os_str(), text).expect("Unable to write file");
    }
    

    pub fn long_term_recall(&self, hash: &str) -> Option<Recollection> {
        let filename = Path::new(&self.long_term_memory_directory)
            .join(format!("{}.exp", &hash));

        let text = match std::fs::read_to_string(filename) {
            Ok(t) => t,
            Err(_) => return None,
        };

        match from_str(&text) {
            Ok(r) => return Some(r),
            Err(_) => return None,
        };
    }

}

// Eventually, it would be better to use a numeralized
// gamestate, or PGN chess notation as the hash. For now,
// the primary obstacle to numeralization is Rust's problem
// with arrays > 32 elements long.
pub fn hash_gamestate(state: &GameState) -> String {
    fen_notation(&state).replace("/", "|")
}

