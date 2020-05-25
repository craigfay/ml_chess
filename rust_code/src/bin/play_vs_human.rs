
use chess_engine::*;
use reinforcement_learning_chess::*;

use std::collections::HashMap;

// Serialization Libs
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;
use serde::{Serialize, Deserialize};


pub struct GameOptions {
    pub agent_playing_as: Color,
}


pub fn play_vs_human(options: GameOptions) {

    // Create an agent, and attempt to restore
    // experiences created by previous training.
    let mut agent = ChessAgent::new();
    agent.retrieve_persisted_experiences("./experiences.ron");


    // Create a new game environment
    let mut environment = ChessEnvironment::new();

    // Set the agent to play as the correct color
    agent.playing_as = options.agent_playing_as;


    // Play until the game is finished
    while false ==  environment.is_terminated() {

        println!("{}", environment.state.to_string()); 

        if environment.state.to_move == agent.playing_as {
            let chosen_next_state = agent.react(&environment);
            environment.apply_change(chosen_next_state);
        }

        else {
            // Build a map of legal actions
            let mut legal_inputs: HashMap<String, GameState> = HashMap::new();
            legal_actions(&environment.state).iter().for_each(|s| {
                let notation = s.as_algebraic_notation(&environment.state);
                let leads_to_state = s.apply(&environment.state);
                legal_inputs.insert(notation, leads_to_state);
            });

            // Ask the player to chose a move
            let mut input = String::new();
            while !legal_inputs.contains_key(&input) {
                println!("Legal moves: {:?}", legal_inputs.keys());
                input = get_input("Choose your move: ");
                println!();
            }

            // Apply the chosen move
            let chosen_next_state = legal_inputs.get(&input).unwrap();
            environment.apply_change(*chosen_next_state);

        }
    }

    // Print the results of the game. The terminal state
    // is from the perspective of the agent, but the
    // terminal output is for the human.
    match environment.terminal_state(agent.playing_as) {
        TerminalState::Loss => println!("You win!"),
        TerminalState::Win => println!("You lose!"),
        TerminalState::Draw => println!("Draw!"),
    }

    agent.persist_experiences("./experiences.ron");
}

pub fn main() {
    play_vs_human(GameOptions {
        agent_playing_as: Color::White,
    });
}
