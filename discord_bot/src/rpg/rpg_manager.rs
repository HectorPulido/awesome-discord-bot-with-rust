use std::collections::HashMap;

struct MultipleDimentionalIdArray {
    height: usize,
    width: usize,
    ids: HashMap<i32, (usize, usize)>,
    grid: Vec<i32>,
}

impl MultipleDimentionalIdArray {
    fn new(height: usize, width: usize) -> MultipleDimentionalIdArray {
        let grid = vec![0; height * width];
        let ids: HashMap<i32, (usize, usize)> = HashMap::new();

        MultipleDimentionalIdArray {
            height,
            width,
            ids,
            grid,
        }
    }

    fn get_coordinates_from_id(&self, id: i32) -> Option<&(usize, usize)> {
        self.ids.get(&id)
    }

    fn get_id_from_coordinates(&self, x: usize, y: usize) -> Option<&i32> {
        self.grid.get(y * self.width + x)
    }

    fn cell_is_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y * self.width + x] == 0
    }

    fn add_id(&mut self, id: i32, x: usize, y: usize) -> bool {
        if !self.cell_is_empty(x, y) {
            return false;
        }

        self.grid[y * self.width + x] = id;
        self.ids.insert(id, (x, y));

        true
    }

    fn remove_id(&mut self, id: i32) -> bool {
        let option = self.ids.get(&id);

        if option == None {
            return false;
        }

        let (x, y) = option.unwrap();
        self.grid[y * self.width + x] = 0;

        self.ids.remove(&id);

        true
    }

    fn move_id(&mut self, id: i32, x: usize, y: usize) -> bool {
        if !self.cell_is_empty(x, y) {
            return false;
        }

        if !self.remove_id(id) {
            return false;
        }

        self.add_id(id, x, y)
    }

    // debug function
    fn represent_grid(&self) -> String {
        let mut grid_string = String::new();

        for i in 0..self.height {
            for j in 0..self.width {
                grid_string = format!("{}{}", grid_string, self.grid[i * self.width + j]);
            }
            grid_string = format!("{}\n", grid_string);
        }

        return grid_string;
    }
}

enum ElemetType {
    PLAYER,
    INANIMATED,
    ENEMY,
    EVENT,
}

struct WorldElement {
    name: String,
    description: String,
    level: i32,
    health: i32,
    attack: i32,
    defense: i32,
    element_type: ElemetType,
    event: Option<String>,
    active: bool,
}

impl WorldElement {
    fn interact(&self) -> String {
        if let Some(event) = &self.event {
            return format!("{}", event);
        }

        format!("{}", self.description)
    }

    fn get_damage(&mut self, damage: i32) -> i32 {
        let damage = damage - self.defense;

        if damage < 0 {
            return 0;
        }

        self.health -= damage;

        if self.health <= 0 {
            self.active = false;
        }

        damage
    }
}

struct GameWorld {
    players_ids: HashMap<String, i32>,
    world_elements: HashMap<i32, WorldElement>,
    wold_grid: MultipleDimentionalIdArray,
}

impl GameWorld {
    fn new(height: usize, width: usize) -> GameWorld {
        let world_elements: HashMap<i32, WorldElement> = HashMap::new();
        let players_ids: HashMap<String, i32> = HashMap::new();
        let wold_grid = MultipleDimentionalIdArray::new(height, width);

        GameWorld {
            players_ids,
            world_elements,
            wold_grid,
        }
    }

    fn add_element_to_world(&mut self, element: WorldElement, x: usize, y: usize) -> i32 {
        let id = self.world_elements.len() as i32 + 1;
        self.world_elements.insert(id, element);
        self.wold_grid.add_id(id, x, y);

        id
    }

    fn create_insert_player(&mut self, name: String, x: usize, y: usize) -> i32 {
        let player = WorldElement {
            name: name.clone(),
            description: format!("You are {}", name),
            level: 1,
            health: 100,
            attack: 100,
            defense: 10,
            element_type: ElemetType::ENEMY,
            event: None,
            active: true,
        };

        let id = self.add_element_to_world(player, x, y);
        self.players_ids.insert(name, id);

        id
    }

    fn describe_world(&self) -> String {
        format!("{}", self.wold_grid.represent_grid())
    }

    fn get_position_from_direction(&self, direction: &str, x: usize, y: usize) -> (usize, usize) {
        let mut new_x = x;
        let mut new_y = y;

        match direction {
            "north" | "up" => {
                if y > 0 {
                    new_y = y - 1;
                }
            }
            "south" | "down" => new_y = y + 1,
            "east" | "right" => new_x = x + 1,
            "west" | "left" => {
                if x > 0 {
                    new_x = x - 1;
                }
            }
            _ => {}
        }

        (new_x, new_y)
    }

    fn cmd_move_player(&mut self, name: &str, direction: &str) -> String {
        let player_id = self.players_ids.get(name).unwrap();
        let (x, y) = self.wold_grid.get_coordinates_from_id(*player_id).unwrap();

        let (new_x, new_y) = self.get_position_from_direction(direction, *x, *y);

        let description = if self.wold_grid.move_id(*player_id, new_x, new_y) {
            format!("You moved {}", direction)
        } else {
            format!("You can't move {}", direction)
        };

        description
    }

    fn cmd_attack_player(&mut self, name: &str, direction: &str) -> String {
        let player_id = self.players_ids.get(name).unwrap();
        let (x, y) = self.wold_grid.get_coordinates_from_id(*player_id).unwrap();
        let (new_x, new_y) = self.get_position_from_direction(direction, *x, *y);

        let description =
            if let Some(enemy_id) = self.wold_grid.get_id_from_coordinates(new_x, new_y) {
                let player_damage = {
                    let player_element = self.world_elements.get(player_id).unwrap();
                    player_element.attack.clone()
                };

                println!("DEBUG player_damage: {}", player_damage);

                if let Some(enemy) = self.world_elements.get_mut(enemy_id) {
                    let damage = enemy.get_damage(player_damage);

                    format!("You attacked {} for {} damage", enemy.name, damage)
                } else {
                    format!("There is no enemy in the {} direction", direction)
                }
            } else {
                format!("You cannot attack to {}", direction)
            };

        description
    }

    fn cmd_interact_player(&mut self, name: &str, direction: &str) -> String {
        let player_id = self.players_ids.get(name).unwrap();
        let (x, y) = self.wold_grid.get_coordinates_from_id(*player_id).unwrap();
        let (new_x, new_y) = self.get_position_from_direction(direction, *x, *y);

        let description =
            if let Some(object_id) = self.wold_grid.get_id_from_coordinates(new_x, new_y) {
                if let Some(object) = self.world_elements.get_mut(object_id) {
                    format!("You interacted with {}\n{}", object.name, object.interact())
                } else {
                    format!("There is nothing in the {} direction", direction)
                }
            } else {
                format!("You cannot interact to {}", direction)
            };

        description
    }

    fn update_world(&mut self) -> String {
        let mut description = String::new();
        let mut elements_to_destroy: Vec<i32> = Vec::new();

        // Remove inactive elements
        for (id, element) in self.world_elements.iter_mut() {
            if !element.active {
                elements_to_destroy.push(*id);
                description = format!("{}\n{} was destroyed", description, element.name.clone());
                self.wold_grid.remove_id(*id);
            }
        }

        for id in elements_to_destroy {
            self.world_elements.remove(&id);
        }

        description.trim().to_string()
    }
}

// fn main() {
//     let mut game_world = GameWorld::new(10, 10);
//     // Add player named "player" in the center of the map
//     game_world.create_insert_player("player".to_string(), 5, 5);
//     // Add enemy named "enemy" in a random place of the map
//     let enemy = WorldElement {
//         name: "enemy".to_string(),
//         description: "You are enemy".to_string(),
//         level: 1,
//         health: 100,
//         attack: 10,
//         defense: 10,
//         element_type: ElemetType::ENEMY,
//         event: None,
//         active: true,
//     };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_id() {
        let mut array = MultipleDimentionalIdArray::new(3, 3);

        let add_id = array.add_id(1, 0, 0);
        assert!(add_id);
        let arr_id = array.add_id(2, 0, 0);
        assert!(!arr_id);
    }

    #[test]
    fn test_get_coordinates_from_id() {
        let mut array = MultipleDimentionalIdArray::new(3, 3);

        array.add_id(1, 0, 0);
        array.add_id(2, 1, 1);
        array.add_id(3, 2, 2);

        assert_eq!(array.get_coordinates_from_id(1), Some(&(0, 0)));
        assert_eq!(array.get_coordinates_from_id(2), Some(&(1, 1)));
        assert_eq!(array.get_coordinates_from_id(3), Some(&(2, 2)));
    }

    #[test]
    fn test_get_id_from_coordinates() {
        let mut array = MultipleDimentionalIdArray::new(3, 3);

        array.add_id(1, 0, 0);
        array.add_id(2, 0, 0);
        array.add_id(3, 1, 1);
        array.add_id(4, 2, 2);

        assert_eq!(array.get_id_from_coordinates(0, 0), Some(&1));
        assert_eq!(array.get_id_from_coordinates(1, 1), Some(&3));
        assert_eq!(array.get_id_from_coordinates(2, 2), Some(&4));
    }

    #[test]
    fn test_cell_is_empty() {
        let mut array = MultipleDimentionalIdArray::new(3, 3);

        array.add_id(1, 0, 0);
        array.remove_id(1);
        array.add_id(3, 1, 1);
        array.add_id(4, 2, 2);

        assert!(array.cell_is_empty(0, 0));
        assert!(array.cell_is_empty(0, 1));
        assert!(!array.cell_is_empty(1, 1));
        assert!(!array.cell_is_empty(2, 2));
    }

    #[test]
    fn test_move_id() {
        let mut array = MultipleDimentionalIdArray::new(3, 3);

        array.add_id(1, 0, 0);
        let move_id_1 = array.move_id(1, 1, 1);
        assert!(move_id_1);

        assert!(array.cell_is_empty(0, 0));
        assert_eq!(array.get_id_from_coordinates(1, 1), Some(&1));
    }
}
