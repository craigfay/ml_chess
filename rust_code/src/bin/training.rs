
use chess_engine::*;

// Serialization Libs
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;
use serde::{Serialize, Deserialize};

use reinforcement_learning_chess::*;

pub struct TrainingOptions {
    pub game_limit: i32,
    pub turn_limit: i32,
    pub save_after_every_nth_game: i32,
}

// TODO prompt for training options
// TODO multithreading?
// TODO debug setting
// TODO reverse gameboard string
// TODO agent time limit / turn
// TODO experience filepath as training option
// TODO use correct fen format, but slice it to use as a hash
// TODO use experience pruning

pub fn training_pipeline(options: TrainingOptions) {
    // Create an agent, and attempt to restore
    // experiences created by previous training.
    let mut agent = ChessAgent::new();
    agent.retrieve_persisted_experiences("./experiences.ron");

    // Play until the game limit is reached
    for game_count in 0..options.game_limit {

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
            
            // Apply the agent's choice
            let chosen_next_state = agent.react(&environment);
            environment.apply_change(chosen_next_state);

            // Apply a random choice on behalf of
            // an imaginary opponent.
            environment.apply_change_randomly();
        }

        if game_count % options.save_after_every_nth_game == 0 {
            agent.persist_experiences("./experiences.ron");
        }
    }

    agent.persist_experiences("./experiences.ron");
}

pub fn main() {
    let options = prompt_training_options();
    training_pipeline(options);
}

fn prompt_training_options() -> TrainingOptions {
    let game_limit: i32 = get_input("Game limit: ") 
        .parse()
        .unwrap();

    let turn_limit: i32 = get_input("Turn limit: ") 
        .parse()
        .unwrap();

    let save_after_every_nth_game: i32 = get_input("Save after every nth game: ") 
        .parse()
        .unwrap();

    TrainingOptions {
        game_limit,
        turn_limit,
        save_after_every_nth_game,
    }
}
