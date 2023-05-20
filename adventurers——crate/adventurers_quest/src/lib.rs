// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
use std::string::String;

// pub trait MyTrait {
//     pub fn new() -> Self;
//     fn update(&mut self, object_type: String);
//     fn set_task_finish(&mut self);
    

// }

pub struct Task1Status {
    task_finish: bool,
    // sub task1
    sub_task1_sand_count: i32,
}

impl Task1Status {
    pub fn new() -> Self {
        Task1Status {
            task_finish: false,
            sub_task1_sand_count: 0,
        }
    }

    pub fn update(&mut self, object_type: String) {
        match object_type.as_str() {           
            "Sand" => {
                self.sub_task1_sand_count+=1;
                if self.sub_task1_sand_count>=5{
                    self.set_task_finish();
                }
            }
            _ => ()
        } 
    }

    pub fn set_task_finish(&mut self){
        self.task_finish=true;
    }

    pub fn is_task_finished(&self) -> bool {
        self.task_finish
    }

    pub fn get_howmany_task1_sand_need(&self) -> i32 {
        if 5-self.sub_task1_sand_count>=0{
            5-self.sub_task1_sand_count
        }else{
            0
        }
    }
    
}


pub struct Task2Status {
    task_finish: bool,
    sub_task1_x_completed: bool,
    sub_task1_x_count: i32,
    sub_task1_y_completed: bool,
    sub_task1_y_count: i32,
}

impl  Task2Status {
    pub fn new() -> Self {
        Task2Status {
            task_finish: false,
            sub_task1_y_completed: false,
            sub_task1_y_count: 0,
            sub_task1_x_completed: false,
            sub_task1_x_count: 0,
        }
    }

    pub fn update(&mut self, object_type: String) {
        match object_type.as_str() {
            object if object.starts_with("Object") => {
                let character_char= object.chars().nth(8).unwrap();
                match character_char{
                    'x' => {
                        self.sub_task1_x_count+=1;
                        if self.sub_task1_x_count>=5{
                            self.sub_task1_x_completed=true;
                        }
                        
                    }
                    'y' => {
                        if self.sub_task1_x_completed{
                            self.sub_task1_y_count+=1;
                            if self.sub_task1_y_count==3{
                                self.set_task_finish();
                            } 
                        }   
                    }
                    _ => ()
                }
            }
            _ => ()
        } 
    }

    pub fn set_task_finish(&mut self){
        self.task_finish=true;
        self.sub_task1_x_completed=true;
        self.sub_task1_y_completed=true;
    }

    pub fn is_task_finished(&self) -> bool {
        self.task_finish
    }

    pub fn is_sub_task1_x_completed(&self) -> bool {
        self.sub_task1_x_completed
    }

    pub fn is_sub_task1_y_completed(&self) -> bool {
        self.sub_task1_y_completed
    }

    pub fn get_howmany_sub_task1_y_need(&self) -> i32 {
        if self.sub_task1_x_completed{
            if 3-self.sub_task1_y_count>=0{
                3-self.sub_task1_y_count
            } else{
                0
            }
        }else {
            3
        }
    }

    pub fn get_howmany_sub_task1_x_need(&self) -> i32 {
        if 5-self.sub_task1_x_count>=0{
            5-self.sub_task1_x_count
        } else{
            0
        }
    }
}


pub struct Task3Status {
    task_finish: bool,
    // sub task1
    finish_sub_task: i32,
    sub_task1_sand_completed: bool,
    sub_task1_sand_count: i32,
    sub_task1_x_completed: bool,
    // sub task2
    sub_task2_y_completed: bool,
    sub_task2_y_count: i32,
    sub_task2_grass_completed: bool,
    sub_task2_grass_count: i32,
    // sub task3
    sub_task3_completed: bool,
    sub_task3_water_completed_time: i32,
    sub_task3_water_count: i32,
}

impl  Task3Status {
    pub fn new() -> Self {
        Task3Status {
            task_finish: false,
            finish_sub_task:0,
            sub_task1_sand_completed: false,
            sub_task1_sand_count: 0,
            sub_task1_x_completed: false,

            // sub task2
            sub_task2_grass_completed: false,
            sub_task2_grass_count: 0,
            sub_task2_y_completed: false,
            sub_task2_y_count: 0,
            // sub task3
            sub_task3_completed: false,
            sub_task3_water_completed_time: 0,
            sub_task3_water_count: 0,
        }
    }

    pub fn update(&mut self, object_type: String) {
        match object_type.as_str() {
            "Grass" => {
                if self.sub_task2_y_completed && !self.is_sub_task2_grass_completed(){
                    self.sub_task2_grass_count=1;
                    self.sub_task2_grass_completed=true;
                    self.finish_sub_task+=1;
                    if self.finish_sub_task>=2{
                        self.set_task_finish();
                    }
                }
                self.sub_task3_water_count=0;
            }
            "Sand" => {
                self.sub_task1_sand_count+=1;
                if self.sub_task1_sand_count>=5{
                    self.sub_task1_sand_completed=true;
                }
                self.sub_task3_water_count=0;
            }
            "Water" => {
                if self.sub_task3_water_completed_time<3{
                    self.sub_task3_water_count+=1;
                    if self.sub_task3_water_count==9{
                        self.sub_task3_water_count=0;
                        self.sub_task3_water_completed_time+=1;
                        if self.sub_task3_water_completed_time>=3{
                            self.sub_task3_completed=true;
                            self.finish_sub_task+=1;
                            if self.finish_sub_task>=2{
                                self.set_task_finish();
                            }
                        }
                    }
                }
                
            }
            object if object.starts_with("Object") => {
                let character_char= object.chars().nth(8).unwrap();
                match character_char{
                    'x' => {
                        if self.sub_task1_sand_completed && !self.sub_task1_x_completed{
                            self.sub_task1_x_completed=true;
                            self.finish_sub_task+=1;
                            if self.finish_sub_task>=2{
                                self.set_task_finish();
                            }
                        }
                    }
                    'y' => {
                        self.sub_task2_y_count+=1;
                        self.sub_task2_y_completed=true;
                    }
                    _ => ()
                }
                self.sub_task3_water_count=0;
            }
            _ => ()
        } 
    }

    pub fn set_task_finish(&mut self){
        self.task_finish=true;
        self.sub_task1_sand_completed=true;
        self.sub_task1_x_completed=true;
        self.sub_task2_grass_completed=true;
        self.sub_task2_y_completed=true;
        self.sub_task3_completed=true;
    }

    pub fn is_task_finished(&self) -> bool {
        self.task_finish
    }

    pub fn is_sub_task1_x_completed(&self) -> bool {
        self.sub_task1_x_completed
    }

    pub fn is_sub_task1_sand_completed(&self) -> bool {
        self.sub_task1_sand_completed
    }

    pub fn is_sub_task2_grass_completed(&self) -> bool {
        self.sub_task2_grass_completed
    }

    pub fn is_sub_task2_y_completed(&self) -> bool {
        self.sub_task2_y_completed
    }

    pub fn is_sub_task3_completed(&self) -> bool {
        self.sub_task3_completed
    }

    pub fn get_howmany_sub_task1_sand_need(&self) -> i32 {
        if 5-self.sub_task1_sand_count>=0{
            5-self.sub_task1_sand_count
        } else{
            0
        }
    }

    pub fn get_howmany_sub_task3_water_need(&self) -> i32 {
        if 3-self.sub_task3_water_completed_time>=0{
            3-self.sub_task3_water_completed_time
        } else{
            0
        }
    }

}
