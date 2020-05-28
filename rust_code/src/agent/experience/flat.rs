
use std::collections::HashMap;
use std::path::Path;
use chess_engine::*;
use crate::environment::*;

#[derive(Copy, Clone, Debug)]
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

    // Write experiences to .exp file
    pub fn long_term_memorize(&self, hash: &str, rec: &Recollection) {
        let filename = Path::new(&self.long_term_memory_directory)
            .join(format!("{}.exp", &hash));

        let text = format!("{}\n{}", rec.times_encountered, rec.average_value);
        std::fs::write(filename.as_os_str(), text).expect("Unable to write file");
    }
    

    // Attempt to recall a Recollection from an .exp file
    pub fn long_term_recall(&self, hash: &str) -> Option<Recollection> {
        let filename = Path::new(&self.long_term_memory_directory)
            .join(format!("{}.exp", &hash));

        match std::fs::read_to_string(filename) {
            Ok(text) => parse_exp_file(&text),
            Err(_) => None,
        }
    }

}

// Eventually, it would be better to use a numeralized
// gamestate, or PGN chess notation as the hash. For now,
// the primary obstacle to numeralization is Rust's problem
// with arrays > 32 elements long.
pub fn hash_gamestate(state: &GameState) -> String {
    fen_notation(&state).replace("/", "|")
}

// .exp files are a representation of a Recollection
// struct, with times_encountered on the first line,
// and average_value on the second.
pub fn parse_exp_file(text: &str) -> Option<Recollection> {
    let mut lines = text.split("\n");
    let maybe_line_1 = lines.next();
    let maybe_line_2 = lines.next();

    let mut line_1 = "";
    let mut line_2 = "";

    match maybe_line_1 {
        Some(line) => line_1 = line,
        _ => return None,
    };

    match maybe_line_2 {
        Some(line) => line_2 = line,
        _ => return None,
    };

    let mut times_encountered: i32 = 0;
    let mut average_value: f32 = 0.0;

    match line_1.parse::<i32>() {
        Ok(i) => times_encountered = i,
        Err(_) => return None,
    };

    match line_2.parse::<f32>() {
        Ok(f) => average_value = f,
        Err(_) => return None,
    };

    Some(Recollection {
        times_encountered,
        average_value,
    })
}

