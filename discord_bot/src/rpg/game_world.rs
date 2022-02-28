use super::bidimensional_array::MultipleDimentionalIdArray;
use super::story::GameStory;

use std::collections::HashMap;
use std::fs;

pub enum ElementType {
    PLAYER,
    INANIMATED,
    ENEMY,
    EVENT,
}

pub struct WorldElement {
    name: String,
    description: String,
    level: i32,
    health: i32,
    attack: i32,
    defense: i32,
    element_type: ElementType,
    event: Option<GameStory>,
    active: bool,
}

impl WorldElement {
    pub fn new(
        name: String,
        description: String,
        level: i32,
        health: i32,
        attack: i32,
        defense: i32,
        element_type: ElementType,
        event: Option<GameStory>,
        active: bool,
    ) -> WorldElement {
        WorldElement {
            name,
            description,
            level,
            health,
            attack,
            defense,
            element_type,
            event,
            active,
        }
    }

    fn interact(&self) -> String {
        if let Some(event) = &self.event {
            // const FILENAME: &str = "history.csv";
            // let contents =
            //     fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
            // let mut game_story = GameStory::new(contents, player_name);

            // return format!("{}", event);
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

pub struct GameWorld {
    players_ids: HashMap<String, i32>,
    world_elements: HashMap<i32, WorldElement>,
    wold_grid: MultipleDimentionalIdArray,
}

impl GameWorld {
    pub fn new(height: usize, width: usize) -> GameWorld {
        let world_elements: HashMap<i32, WorldElement> = HashMap::new();
        let players_ids: HashMap<String, i32> = HashMap::new();
        let wold_grid = MultipleDimentionalIdArray::new(height, width);

        GameWorld {
            players_ids,
            world_elements,
            wold_grid,
        }
    }

    pub fn add_element_to_world(&mut self, element: WorldElement, x: usize, y: usize) -> i32 {
        let id = self.world_elements.len() as i32 + 1;
        self.world_elements.insert(id, element);
        self.wold_grid.add_id(id, x, y);

        id
    }

    pub fn create_insert_player(&mut self, name: String, x: usize, y: usize) -> i32 {
        let player = WorldElement::new(
            name.clone(),
            format!("You are {}", name),
            1,
            100,
            100,
            10,
            ElementType::ENEMY,
            None,
            true,
        );

        let id = self.add_element_to_world(player, x, y);
        self.players_ids.insert(name, id);

        id
    }

    pub fn describe_world(&self) -> String {
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

    pub fn cmd_move_player(&mut self, name: &str, direction: &str) -> String {
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

    pub fn cmd_attack_player(&mut self, name: &str, direction: &str) -> String {
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

    pub fn cmd_interact_player(&mut self, name: &str, direction: &str) -> String {
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

    pub fn update_world(&mut self) -> String {
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
