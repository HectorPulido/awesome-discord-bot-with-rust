use super::game_world::{ElementType, GameWorld, WorldElement};
use super::story::GameStory;

use std::collections::HashMap;

// fn main() {
//     let mut game_world = GameWorld::new(10, 10);
//     // Add player named "player" in the center of the map
//     game_world.create_insert_player("player".to_string(), 5, 5);
//     // Add enemy named "enemy" in a random place of the map
//     let enemy = WorldElement::new(
//         "enemy".to_string(),
//         "You are enemy".to_string(),
//         1,
//         100,
//         10,
//         10,
//         ElementType::ENEMY,
//         None,
//         true,
//     );
//     game_world.add_element_to_world(
//         enemy,
//         rand::random::<usize>() % 10,
//         rand::random::<usize>() % 10,
//     );

//     let player_name = "player";

//     loop {
//         println!("Please enter your command:");
//         let mut command = String::new();
//         std::io::stdin().read_line(&mut command).unwrap();
//         let command = command.trim().to_lowercase();
//         let command_split: Vec<&str> = command.split_whitespace().collect();
//         if command_split.len() == 0 {
//             println!("Unknown command!");
//         }

//         let primary_command = command_split[0];
//         let first_argument = command_split.get(1).unwrap_or(&"");

//         let response = match primary_command {
//             "move" => game_world.cmd_move_player(player_name, first_argument),
//             "attack" => game_world.cmd_attack_player(player_name, first_argument),
//             "describe" => game_world.describe_world(),
//             "interact" => game_world.cmd_interact_player(player_name, first_argument),
//             _ => "Unknown command!".to_string(),
//         };

//         println!("{}", response);

//         let update_description = game_world.update_world();
//         println!("{}", update_description);
//     }
// }

// use std::fs;

// const FILENAME: &str = "history.csv";
// fn main() {
//     let contents = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");

//     // let records = StoryRow::read_rows(contents);
//     let mut game_story = GameStory::new(contents);

//     // Game Loop
//     loop {
//         let options = game_story.show_options();
//         println!("{}", options);

//         let mut selection = String::new();
//         std::io::stdin().read_line(&mut selection).unwrap();

//         let (end_game, response) = game_story.next_step(&selection);

//         println!("{}", response);
//         if end_game {
//             break;
//         }
//     }
// }
