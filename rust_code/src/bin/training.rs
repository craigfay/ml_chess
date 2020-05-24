
use chess_engine::*;

// Serialization Libs
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;
use serde::{Serialize, Deserialize};

use reinforcement_learning_chess::*;

pub struct TrainingOptions {
    pub game_limit: i32,
    pub turn_limit: i32,
}

pub fn training_pipeline(options: TrainingOptions) {
    // Create an agent, and attempt to restore
    // experiences created by previous training.
    let mut agent = ChessAgent::new();
    agent.retrieve_persisted_experiences("./experiences.ron");

    // Play until the game limit is reached
    for _ in 0..options.game_limit {
        // Create a new environment, and switch sides
        let mut environment = ChessEnvironment::new();
        agent.playing_as = match agent.playing_as {
            Color::White => Color::Black,
            Color::Black=> Color::White,
        };

        // Play until the turn limit is reached
        for _ in 0..options.turn_limit {
            if environment.is_terminated() {
                break;
            }
            
            let decision = agent.react(&environment);
            environment.apply(decision);
        }
    }

    agent.persist_experiences("./experiences.ron");
}

pub fn main() {
    training_pipeline(TrainingOptions {
        game_limit: 1,
        turn_limit: 5,
    });
}
