use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;

const INITIAL_TAG: &str = "INICIO";

#[derive(Clone, Debug)]
enum DataType {
    SITUATION,
    OPTION,
    ERROR,
}

#[derive(Clone, Debug)]
pub struct StoryRow {
    data_type: DataType,
    pub tag: String,
    pub text: String,
    pub health_change: i32,
    pub options: Vec<StoryRow>,
}

impl StoryRow {
    fn new(record: StringRecord) -> StoryRow {
        let data_type = record.get(0).unwrap().trim();
        let data_type = match data_type {
            "SITUACION" => DataType::SITUATION,
            "OPCION" => DataType::OPTION,
            _ => DataType::ERROR,
        };

        let tag: String = String::from(record.get(1).unwrap().trim());
        let text: String = String::from(record.get(2).unwrap().trim());

        let health_change = record.get(3).unwrap().trim();
        let health_change: i32 = health_change.parse().unwrap_or(0);

        let options: Vec<StoryRow> = vec![];

        StoryRow {
            data_type,
            tag,
            text,
            health_change,
            options,
        }
    }

    pub fn read_rows(contents: String) -> HashMap<String, StoryRow> {
        let mut last_record: String = "".to_string();
        let mut records: HashMap<String, StoryRow> = HashMap::new();

        // Read csv
        let mut rdr = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(contents.as_bytes());

        // Get data from csv
        for result in rdr.records() {
            let record = StoryRow::new(result.unwrap());

            match record.data_type {
                DataType::SITUATION => {
                    last_record = record.tag.clone();
                    records.insert(record.tag.clone(), record);
                }
                DataType::OPTION => {
                    if let Some(data) = records.get_mut(&last_record) {
                        (*data).options.push(record);
                    }
                }
                _ => {}
            }
        }

        records
    }
}

pub struct GameStory {
    game_variables: HashMap<String, String>,
    health: i32,
    current_tag: String,
    records: HashMap<String, StoryRow>,
}

impl GameStory {
    pub fn new(contents: String, player_name: String) -> GameStory {
        let mut game_variables = HashMap::new();
        game_variables.insert("player_name".to_string(), player_name);

        let health = 100;
        let current_tag = INITIAL_TAG.to_string();
        let records = StoryRow::read_rows(contents);

        GameStory {
            game_variables,
            health,
            current_tag,
            records,
        }
    }

    pub fn show_options(&self) -> String {
        let mut response: String = format!("Tienes {} de vida", self.health);
        if let Some(data) = self.records.get(&self.current_tag) {
            response = format!("{}\n{}", response, data.text);

            for (key, value) in self.game_variables.iter() {
                let k = format!("|{}|", key);

                response = response.replace(&k, value);
            }

            //

            for (i, option) in data.options.iter().enumerate() {
                response = format!("{}\n[{}] {}", response, i, option.text);
            }
        }
        response
    }

    pub fn next_step(&mut self, selection: &str) -> (bool, String) {
        let mut response: String = format!("");
        let mut end_game: bool = false;

        if let Some(data) = self.records.get(&self.current_tag) {
            let selection = selection.trim().parse().unwrap_or(99);

            if let Some(selection) = &data.options.get(selection) {
                self.current_tag = selection.tag.to_string();
            } else {
                response = format!("{}\nComando no valido", response);
            }

            self.health += data.health_change;
            response = format!("{}\n", response);
        } else {
            response = format!("{}\nError interno en linea {}", response, self.current_tag);
            end_game = true;
        }

        if self.health <= 0 {
            response = format!("{}\n== Game Over ==", response);
            end_game = true;
        }

        (end_game, response)
    }
}
