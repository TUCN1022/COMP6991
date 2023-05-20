use adventurers_quest::{Task1Status, Task2Status, Task3Status};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;
use std::time::Duration;
use termgame::{run_game, Controller, Game, GameEvent, GameSettings, Message, SimpleEvent};
use termgame::{GameColor, GameStyle, KeyCode, StyledCharacter};

#[derive(Deserialize, Debug, Clone)]
struct PositionType {
    position: (i32, i32),
    object_type: String,
}

fn parse_position_type(s: &str) -> Option<PositionType> {
    let parts: Vec<&str> = s.trim().split(":").collect();
    if parts.len() != 2 {
        println!("parts's len is {}", parts.len());
        return None;
    }

    let pos_str = parts[0];
    let ori_obj_type = parts[1].trim();
    let obj_type = &ori_obj_type[..ori_obj_type.len() - 1];

    let pos_parts: Vec<&str> = pos_str.split(",").collect();

    if pos_parts.len() != 2 {
        println!("pos parts's len is {}", pos_parts.len());
        return None;
    }
    let x_trim = pos_parts[0].trim();
    let x_trim_slice = &x_trim[1..];
    let y_trim = pos_parts[1].trim();
    let y_trim_slice = &y_trim[..y_trim.len() - 1];
    let x = x_trim_slice.parse().ok()?;
    let y = y_trim_slice.parse().ok()?;

    Some(PositionType {
        position: (x, y),
        object_type: obj_type.to_owned(),
    })
}

fn get_map_file() -> HashMap<(i32, i32), String> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let map_file = File::open(file_path).unwrap();
    let reader = BufReader::new(map_file);
    let mut lines = reader.lines().skip(1);
    // Read Maps data
    let mut map_datas: HashMap<(i32, i32), String> = HashMap::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.trim() == "}" {
            break;
        }
        let trim_line = line.trim();

        let position_type = parse_position_type(&trim_line).unwrap();
        map_datas.insert(position_type.position, position_type.object_type);
    }
    return map_datas;
}

fn get_object_background_color() -> HashMap<String, GameColor> {
    let mut map_obj2color: HashMap<String, GameColor> = HashMap::new();
    let map_datas = get_map_file();
    map_obj2color.insert("Grass".to_string(), termgame::GameColor::Green);
    map_obj2color.insert("Sand".to_string(), termgame::GameColor::Yellow);
    map_obj2color.insert("Rock".to_string(), termgame::GameColor::DarkGray);
    map_obj2color.insert("Cinderblock".to_string(), termgame::GameColor::LightRed);
    map_obj2color.insert("Flower".to_string(), termgame::GameColor::Magenta);
    map_obj2color.insert("Water".to_string(), termgame::GameColor::Blue);
    map_obj2color.insert("Barrier".to_string(), termgame::GameColor::White);
    map_obj2color.insert("Flowerbush".to_string(), termgame::GameColor::Cyan);
    for (_key, value) in map_datas {
        if !map_obj2color.contains_key(&value) {
            map_obj2color.insert(value, termgame::GameColor::Black);
        }
    }
    return map_obj2color;
}

struct MyGame {
    pos_x: i32,
    pos_y: i32,
    water_time: i32,
    request1: Task1Status,
    request2: Task2Status,
    request3: Task3Status,
}

impl MyGame {
    fn new() -> Self {
        MyGame {
            pos_x: 2,
            pos_y: 2,
            water_time: 0,
            request1: Task1Status::new(),
            request2: Task2Status::new(),
            request3: Task3Status::new(),
        }
    }
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        //get the ron file map
        let map_datas = get_map_file();
        //get the object to color map
        let obj2color_map = get_object_background_color();
        for (map_data_key, map_data_value) in &map_datas {
            let (row, col) = map_data_key;
            let block_type = map_data_value;
            match block_type.as_str() {
                "Grass" => game.set_screen_char(
                    *row,
                    *col,
                    Some(
                        StyledCharacter::new(' ').style(
                            GameStyle::new()
                                .background_color(Some(GameColor::Green))
                                .color(Some(GameColor::Green)),
                        ),
                    ),
                ),
                "Sand" => game.set_screen_char(
                    *row,
                    *col,
                    Some(
                        StyledCharacter::new(' ')
                            .style(GameStyle::new().background_color(Some(GameColor::Yellow))),
                    ),
                ),
                "Rock" => game.set_screen_char(
                    *row,
                    *col,
                    Some(
                        StyledCharacter::new(' ')
                            .style(GameStyle::new().background_color(Some(GameColor::DarkGray))),
                    ),
                ),
                "Cinderblock" => game.set_screen_char(
                    *row,
                    *col,
                    Some(
                        StyledCharacter::new(' ')
                            .style(GameStyle::new().background_color(Some(GameColor::LightRed))),
                    ),
                ),
                "Flower" => game.set_screen_char(
                    *row,
                    *col,
                    Some(
                        StyledCharacter::new(' ')
                            .style(GameStyle::new().background_color(Some(GameColor::Magenta))),
                    ),
                ),
                "Water" => game.set_screen_char(
                    *row,
                    *col,
                    Some(
                        StyledCharacter::new(' ')
                            .style(GameStyle::new().background_color(Some(GameColor::Blue))),
                    ),
                ),
                "Barrier" => game.set_screen_char(
                    *row,
                    *col,
                    Some(
                        StyledCharacter::new(' ')
                            .style(GameStyle::new().background_color(Some(GameColor::White))),
                    ),
                ),
                "Flowerbush" => game.set_screen_char(
                    *row,
                    *col,
                    Some(
                        StyledCharacter::new(' ')
                            .style(GameStyle::new().background_color(Some(GameColor::Cyan))),
                    ),
                ),
                s if s.starts_with("Sign") => {
                    game.set_screen_char(
                        *row,
                        *col,
                        Some(
                            StyledCharacter::new('💭')
                                .style(GameStyle::new().background_color(Some(GameColor::Black))),
                        ),
                    );
                }
                s if s.starts_with("Object") => {
                    let character_char = s.chars().nth(8).unwrap();
                    game.set_screen_char(
                        *row,
                        *col,
                        Some(
                            StyledCharacter::new(character_char)
                                .style(GameStyle::new().background_color(Some(GameColor::Black))),
                        ),
                    );
                }
                _ => game.set_screen_char(
                    *row,
                    *col,
                    Some(
                        StyledCharacter::new(' ')
                            .style(GameStyle::new().background_color(Some(GameColor::Black))),
                    ),
                ),
            }
        }

        let mut ori_background_color: GameColor = GameColor::Black;
        if let Some(object) = map_datas.get(&(self.pos_x, self.pos_y)) {
            ori_background_color = *obj2color_map.get(object).unwrap();
        }
        if map_datas.get(&(self.pos_x, self.pos_y)) == Some(&"Water".to_string()) {
            self.water_time += 1;
        }
        game.set_screen_char(
            self.pos_x,
            self.pos_y,
            Some(
                StyledCharacter::new('\u{265F}').style(
                    GameStyle::new()
                        .background_color(Some(ori_background_color))
                        .color(Some(GameColor::White)),
                ),
            ),
        );
        match map_datas.get(&(self.pos_x, self.pos_y)) {
            Some(_) => {
                self.request1
                    .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                self.request2
                    .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                self.request3
                    .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
            }
            _ => (),
        }
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        let args: Vec<String> = env::args().collect();
        let request_task = &args[2];
        let mut complete_or_not = HashMap::new();
        complete_or_not.insert(true, '✅');
        complete_or_not.insert(false, ' ');

        let viewport_x = game.get_viewport().x;
        let viewport_y = game.get_viewport().y;
        //get the ron file map
        let map_datas = get_map_file();
        //get the object to color map
        let obj2color_map = get_object_background_color();

        match event.into() {
            //turn left
            SimpleEvent::Just(KeyCode::Left) => {
                if self.water_time >= 10 {
                    game.end_game();
                }
                game.set_message(None);
                let old_pos_x = self.pos_x;
                let old_pos_y = self.pos_y;
                let mut new_pos_x = self.pos_x - 1;
                let new_pos_y = self.pos_y;

                let mut nextpoint_background_color: GameColor = GameColor::Black;
                if let Some(object) = map_datas.get(&(new_pos_x, new_pos_y)) {
                    nextpoint_background_color = *obj2color_map.get(object).unwrap();
                }
                let mut original_background_color: GameColor = GameColor::Black;
                if let Some(object) = map_datas.get(&(old_pos_x, old_pos_y)) {
                    original_background_color = *obj2color_map.get(object).unwrap();
                }

                if nextpoint_background_color == GameColor::White {
                    new_pos_x = self.pos_x;
                    game.set_screen_char(
                        new_pos_x,
                        new_pos_y,
                        Some(StyledCharacter::new('\u{265F}').style(
                            GameStyle::new().background_color(Some(original_background_color)),
                        )),
                    );
                } else {
                    game.set_screen_char(
                        new_pos_x,
                        new_pos_y,
                        Some(StyledCharacter::new('\u{265F}').style(
                            GameStyle::new().background_color(Some(nextpoint_background_color)),
                        )),
                    );
                    if let Some(check_sign) = map_datas.get(&(new_pos_x, new_pos_y)) {
                        if check_sign.starts_with("Sign") {
                            let breathe_message =
                                Message::new(String::from("Don't worry, you can breathe here."))
                                    .title(String::from("Breathe here"));
                            game.set_message(Some(breathe_message));
                        }
                    }

                    self.pos_x -= 1;
                    if let Some(check_sign_old) = map_datas.get(&(old_pos_x, old_pos_y)) {
                        if check_sign_old.starts_with("Sign") {
                            game.set_screen_char(
                                old_pos_x,
                                old_pos_y,
                                Some(StyledCharacter::new('💭').style(
                                    GameStyle::new().background_color(Some(GameColor::Black)),
                                )),
                            );
                        } else {
                            game.set_screen_char(
                                old_pos_x,
                                old_pos_y,
                                Some(
                                    StyledCharacter::new(' ').style(
                                        GameStyle::new()
                                            .background_color(Some(original_background_color)),
                                    ),
                                ),
                            );
                        }
                    } else {
                        game.set_screen_char(
                            old_pos_x,
                            old_pos_y,
                            Some(StyledCharacter::new(' ').style(
                                GameStyle::new().background_color(Some(original_background_color)),
                            )),
                        );
                    }
                }

                //get the water time
                if map_datas.get(&(self.pos_x, self.pos_y)) == Some(&"Water".to_string()) {
                    self.water_time += 1;
                    if self.water_time >= 10 {
                        let drown_message =
                            Message::new(String::from("You drown :(")).title(String::from("Drown"));
                        game.set_message(Some(drown_message));
                    }
                } else {
                    self.water_time = 0;
                }

                match map_datas.get(&(self.pos_x, self.pos_y)) {
                    Some(_) => {
                        self.request1
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                        self.request2
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                        self.request3
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                    }
                    _ => (),
                }

                if self.pos_x - viewport_x <= 3 && viewport_x != 0 {
                    game.set_viewport(termgame::ViewportLocation {
                        x: viewport_x - 1,
                        y: viewport_y,
                    });
                }
            }
            //turn right
            SimpleEvent::Just(KeyCode::Right) => {
                if self.water_time >= 10 {
                    game.end_game();
                }
                game.set_message(None);
                let old_pos_x = self.pos_x;
                let old_pos_y = self.pos_y;
                let mut new_pos_x = self.pos_x + 1;
                let new_pos_y = self.pos_y;

                let mut nextpoint_background_color: GameColor = GameColor::Black;
                if let Some(object) = map_datas.get(&(new_pos_x, new_pos_y)) {
                    nextpoint_background_color = *obj2color_map.get(object).unwrap();
                }
                let mut original_background_color: GameColor = GameColor::Black;
                if let Some(object) = map_datas.get(&(old_pos_x, old_pos_y)) {
                    original_background_color = *obj2color_map.get(object).unwrap();
                }

                if nextpoint_background_color == GameColor::White {
                    new_pos_x = self.pos_x;
                    game.set_screen_char(
                        new_pos_x,
                        new_pos_y,
                        Some(StyledCharacter::new('\u{265F}').style(
                            GameStyle::new().background_color(Some(original_background_color)),
                        )),
                    );
                } else {
                    game.set_screen_char(
                        new_pos_x,
                        new_pos_y,
                        Some(StyledCharacter::new('\u{265F}').style(
                            GameStyle::new().background_color(Some(nextpoint_background_color)),
                        )),
                    );
                    if let Some(check_sign) = map_datas.get(&(new_pos_x, new_pos_y)) {
                        if check_sign.starts_with("Sign") {
                            let breathe_message =
                                Message::new(String::from("Don't worry, you can breathe here."))
                                    .title(String::from("Breathe here"));
                            game.set_message(Some(breathe_message));
                        }
                    }

                    self.pos_x += 1;
                    if let Some(check_sign_old) = map_datas.get(&(old_pos_x, old_pos_y)) {
                        if check_sign_old.starts_with("Sign") {
                            game.set_screen_char(
                                old_pos_x,
                                old_pos_y,
                                Some(StyledCharacter::new('💭').style(
                                    GameStyle::new().background_color(Some(GameColor::Black)),
                                )),
                            );
                        } else {
                            game.set_screen_char(
                                old_pos_x,
                                old_pos_y,
                                Some(
                                    StyledCharacter::new(' ').style(
                                        GameStyle::new()
                                            .background_color(Some(original_background_color)),
                                    ),
                                ),
                            );
                        }
                    } else {
                        game.set_screen_char(
                            old_pos_x,
                            old_pos_y,
                            Some(StyledCharacter::new(' ').style(
                                GameStyle::new().background_color(Some(original_background_color)),
                            )),
                        );
                    }
                }

                //get the water time
                if map_datas.get(&(self.pos_x, self.pos_y)) == Some(&"Water".to_string()) {
                    self.water_time += 1;
                    if self.water_time >= 10 {
                        let drown_message =
                            Message::new(String::from("You drown :(")).title(String::from("Drown"));
                        game.set_message(Some(drown_message));
                    }
                } else {
                    self.water_time = 0;
                }

                match map_datas.get(&(self.pos_x, self.pos_y)) {
                    Some(_) => {
                        self.request1
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                        self.request2
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                        self.request3
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                    }
                    _ => (),
                }

                if self.pos_x - viewport_x >= 75 && viewport_x != 79 {
                    game.set_viewport(termgame::ViewportLocation {
                        x: viewport_x + 1,
                        y: viewport_y,
                    });
                }
            }
            //turn up
            SimpleEvent::Just(KeyCode::Up) => {
                if self.water_time >= 10 {
                    game.end_game();
                }
                game.set_message(None);
                let old_pos_x = self.pos_x;
                let old_pos_y = self.pos_y;
                let new_pos_x = self.pos_x;
                let mut new_pos_y = self.pos_y - 1;

                let mut nextpoint_background_color: GameColor = GameColor::Black;
                if let Some(object) = map_datas.get(&(new_pos_x, new_pos_y)) {
                    nextpoint_background_color = *obj2color_map.get(object).unwrap();
                }
                let mut original_background_color: GameColor = GameColor::Black;
                if let Some(object) = map_datas.get(&(old_pos_x, old_pos_y)) {
                    original_background_color = *obj2color_map.get(object).unwrap();
                }

                if nextpoint_background_color == GameColor::White {
                    new_pos_y = self.pos_y;
                    game.set_screen_char(
                        new_pos_x,
                        new_pos_y,
                        Some(StyledCharacter::new('\u{265F}').style(
                            GameStyle::new().background_color(Some(original_background_color)),
                        )),
                    );
                } else {
                    game.set_screen_char(
                        new_pos_x,
                        new_pos_y,
                        Some(StyledCharacter::new('\u{265F}').style(
                            GameStyle::new().background_color(Some(nextpoint_background_color)),
                        )),
                    );
                    if let Some(check_sign) = map_datas.get(&(new_pos_x, new_pos_y)) {
                        if check_sign.starts_with("Sign") {
                            let breathe_message =
                                Message::new(String::from("Don't worry, you can breathe here."))
                                    .title(String::from("Breathe here"));
                            game.set_message(Some(breathe_message));
                        }
                    }

                    self.pos_y -= 1;
                    if let Some(check_sign_old) = map_datas.get(&(old_pos_x, old_pos_y)) {
                        if check_sign_old.starts_with("Sign") {
                            game.set_screen_char(
                                old_pos_x,
                                old_pos_y,
                                Some(StyledCharacter::new('💭').style(
                                    GameStyle::new().background_color(Some(GameColor::Black)),
                                )),
                            );
                        } else {
                            game.set_screen_char(
                                old_pos_x,
                                old_pos_y,
                                Some(
                                    StyledCharacter::new(' ').style(
                                        GameStyle::new()
                                            .background_color(Some(original_background_color)),
                                    ),
                                ),
                            );
                        }
                    } else {
                        game.set_screen_char(
                            old_pos_x,
                            old_pos_y,
                            Some(StyledCharacter::new(' ').style(
                                GameStyle::new().background_color(Some(original_background_color)),
                            )),
                        );
                    }
                }

                //get the water time
                if map_datas.get(&(self.pos_x, self.pos_y)) == Some(&"Water".to_string()) {
                    self.water_time += 1;
                    if self.water_time >= 10 {
                        let drown_message =
                            Message::new(String::from("You drown :(")).title(String::from("Drown"));
                        game.set_message(Some(drown_message));
                    }
                } else {
                    self.water_time = 0;
                }
                match map_datas.get(&(self.pos_x, self.pos_y)) {
                    Some(_) => {
                        self.request1
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                        self.request2
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                        self.request3
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                    }
                    _ => (),
                }

                if self.pos_y - viewport_y <= 3 && viewport_y != 0 {
                    game.set_viewport(termgame::ViewportLocation {
                        x: viewport_x,
                        y: viewport_y - 1,
                    });
                }
            }
            //turn down
            SimpleEvent::Just(KeyCode::Down) => {
                if self.water_time >= 10 {
                    game.end_game();
                }
                game.set_message(None);
                let old_pos_x = self.pos_x;
                let old_pos_y = self.pos_y;
                let new_pos_x = self.pos_x;
                let mut new_pos_y = self.pos_y + 1;

                let mut nextpoint_background_color: GameColor = GameColor::Black;
                if let Some(object) = map_datas.get(&(new_pos_x, new_pos_y)) {
                    nextpoint_background_color = *obj2color_map.get(object).unwrap();
                }
                let mut original_background_color: GameColor = GameColor::Black;
                if let Some(object) = map_datas.get(&(old_pos_x, old_pos_y)) {
                    original_background_color = *obj2color_map.get(object).unwrap();
                }

                if nextpoint_background_color == GameColor::White {
                    new_pos_y = self.pos_y;
                    game.set_screen_char(
                        new_pos_x,
                        new_pos_y,
                        Some(StyledCharacter::new('\u{265F}').style(
                            GameStyle::new().background_color(Some(original_background_color)),
                        )),
                    );
                } else {
                    game.set_screen_char(
                        new_pos_x,
                        new_pos_y,
                        Some(StyledCharacter::new('\u{265F}').style(
                            GameStyle::new().background_color(Some(nextpoint_background_color)),
                        )),
                    );
                    if let Some(check_sign) = map_datas.get(&(new_pos_x, new_pos_y)) {
                        if check_sign.starts_with("Sign") {
                            let breathe_message =
                                Message::new(String::from("Don't worry, you can breathe here."))
                                    .title(String::from("Breathe here"));
                            game.set_message(Some(breathe_message));
                        }
                    }

                    self.pos_y += 1;
                    if let Some(check_sign_old) = map_datas.get(&(old_pos_x, old_pos_y)) {
                        if check_sign_old.starts_with("Sign") {
                            game.set_screen_char(
                                old_pos_x,
                                old_pos_y,
                                Some(StyledCharacter::new('💭').style(
                                    GameStyle::new().background_color(Some(GameColor::Black)),
                                )),
                            );
                        } else {
                            game.set_screen_char(
                                old_pos_x,
                                old_pos_y,
                                Some(
                                    StyledCharacter::new(' ').style(
                                        GameStyle::new()
                                            .background_color(Some(original_background_color)),
                                    ),
                                ),
                            );
                        }
                    } else {
                        game.set_screen_char(
                            old_pos_x,
                            old_pos_y,
                            Some(StyledCharacter::new(' ').style(
                                GameStyle::new().background_color(Some(original_background_color)),
                            )),
                        );
                    }
                }
                //get the water time
                if map_datas.get(&(self.pos_x, self.pos_y)) == Some(&"Water".to_string()) {
                    self.water_time += 1;
                    if self.water_time >= 10 {
                        let drown_message =
                            Message::new(String::from("You drown :(")).title(String::from("Drown"));
                        game.set_message(Some(drown_message));
                    }
                } else {
                    self.water_time = 0;
                }

                match map_datas.get(&(self.pos_x, self.pos_y)) {
                    Some(_) => {
                        self.request1
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                        self.request2
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                        self.request3
                            .update(map_datas.get(&(self.pos_x, self.pos_y)).unwrap().clone());
                    }
                    _ => (),
                }

                if self.pos_y - viewport_y >= 18 && viewport_y != 23 {
                    game.set_viewport(termgame::ViewportLocation {
                        x: viewport_x,
                        y: viewport_y + 1,
                    });
                }
            }

            SimpleEvent::Just(KeyCode::Char('q')) => {
                let quest_message = match request_task.as_str(){
                    "first_quest" => {
                        Message::new(String::from(format!("[{}] Walk on a sand block..\n ^  (Complete {} more time.)",complete_or_not.get(&self.request1.is_task_finished()).unwrap(),self.request1.get_howmany_task1_sand_need())))

                    }
                    "second_quest" =>{
                        Message::new(String::from(format!("[{}] You must, in order, complete each of these quests:\n   [{}] Collect a 'x'..\n   ^  (Complete {} more time.)\n   [{}] Collect a 'y'..\n   ^  (Complete {} more time.)",complete_or_not.get(&self.request2.is_task_finished()).unwrap(),
                        complete_or_not.get(&self.request2.is_sub_task1_x_completed()).unwrap(),self.request2.get_howmany_sub_task1_x_need(),complete_or_not.get(&self.request2.is_sub_task1_y_completed()).unwrap(),self.request2.get_howmany_sub_task1_y_need())))
                    }
                    "third_quest" =>{
                        Message::new(String::from(format!("[{}] You must complete at least 2 of these quests:\n  [{}] You must, in order, complete each of these quests:\n    [{}] Walk on a sand bolck..\n     ^  (Complete {} more time.)\n    [{}] Collect a 'x'..\n[{}] You must, in order, complete each of these quests:\n    [{}] Collect a 'y'..\n    [{}] Walk on a grass block..\n  [{}] Walk through exactly 9 blocks of water..\n   ^  (Complete {} more time.)",
                        complete_or_not.get(&self.request3.is_task_finished()).unwrap(),
                        complete_or_not.get(&self.request3.is_sub_task1_x_completed()).unwrap(),
                        complete_or_not.get(&self.request3.is_sub_task1_sand_completed()).unwrap(),
                        self.request3.get_howmany_sub_task1_sand_need(),
                        complete_or_not.get(&self.request3.is_sub_task1_x_completed()).unwrap(),
                        complete_or_not.get(&self.request3.is_sub_task2_grass_completed()).unwrap(),
                        complete_or_not.get(&self.request3.is_sub_task2_y_completed()).unwrap(),
                        complete_or_not.get(&self.request3.is_sub_task2_grass_completed()).unwrap(),
                        complete_or_not.get(&self.request3.is_sub_task3_completed()).unwrap(),
                        self.request3.get_howmany_sub_task3_water_need())))
                    }
                    _ => {
                        Message::new(String::from(""))
                    }
                };
                game.set_message(Some(quest_message));
            }
            SimpleEvent::Just(KeyCode::Char('r')) => {
                self.request1 = Task1Status::new();
                self.request2 = Task2Status::new();
                self.request3 = Task3Status::new();
            }

            _ => {}
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = MyGame::new();
    run_game(
        &mut controller,
        GameSettings::new()
            // The below are the defaults, but shown so you can edit them.
            .tick_duration(Duration::from_millis(50))
            .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into())),
    )?;

    println!("Game Ended!");
    Ok(())
}
