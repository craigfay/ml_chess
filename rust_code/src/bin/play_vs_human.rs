
use chess_engine::*;

// Serialization Libs
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;
use serde::{Serialize, Deserialize};

use reinforcement_learning_chess::*;

pub struct GameOptions {
    pub playing_as: Color,
}


pub fn play_vs_human(options: GameOptions) {

    // Create an agent, and attempt to restore
    // experiences created by previous training.
    let mut agent = ChessAgent::new();
    agent.retrieve_persisted_experiences("./experiences.ron");


    // Create a new game environment
    let mut environment = ChessEnvironment::new();

    // Set the agent to play as the correct color
    agent.playing_as = options.playing_as;


    // Play until the game is finished
    while false ==  environment.is_terminated() {

        if environment.state.to_move == agent.playing_as {
            let chosen_next_state = agent.react(&environment);
            environment.apply_change(chosen_next_state);
        }

        else {
            environment.apply_change_randomly();
        }

        println!("{}", environment.state.to_string()); 
    }

    agent.persist_experiences("./experiences.ron");
}

pub fn main() {
    play_vs_human(GameOptions {
        playing_as: Color::White,
    });
}
