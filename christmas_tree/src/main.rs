use serde::Deserialize;
use std::collections::VecDeque;
use std::io;
use ron;

#[derive(Debug, Deserialize)]
enum Instruction {
    Set(i32),
    Left,
    Right,
    Reset,
}

#[derive(Debug)]
struct Light {
    left: Option<Box<Light>>,
    right: Option<Box<Light>>,
    brightness: i32,
}



fn clone_light(light: &Light) -> Light {
    Light {
        left: light.left.as_ref().map(|l| Box::new(clone_light(l))),
        right: light.right.as_ref().map(|r| Box::new(clone_light(r))),
        brightness: light.brightness,
    }
}

impl Clone for Light {
    fn clone(&self) -> Self {
        clone_light(self)
    }
}


fn get_instructions_from_stdin() -> VecDeque<Instruction> {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();
    ron::from_str(&instructions).unwrap()
}


fn new() -> Light {
    Light { left: None, right: None, brightness: 0 }
}


fn counting(light: &Light) -> (i32, i32) {
    let mut count = 1;
    let mut total_brightless=light.brightness;
    if let Some(left) = & light.left {
        count += counting(&**left).0;
        total_brightless += counting(&**left).1;
    }
    if let Some(right) = &light.right {
        count += counting(&**right).0;
        total_brightless += counting(&**right).1;
    }
    let data=(count,total_brightless);
    data
}

fn main() {
    let instructions = get_instructions_from_stdin();
    let mut root = new();
    let mut current_light=&mut root;
    
    for instruction in instructions {
        match instruction {
            Instruction::Set(brightness) => current_light.brightness = brightness,
            Instruction::Left => {
                if let None = current_light.left {
                    let left_light=new();
                    current_light.left = Some(Box::new(left_light));
                    
                } 
                current_light=current_light.left.as_mut().unwrap();
                
            },
            Instruction::Right => {
                if let None = current_light.right {
                    let right_light=new();
                    current_light.right = Some(Box::new(right_light));
                    
                } 
                current_light=current_light.right.as_mut().unwrap();
                
            },
            Instruction::Reset => {
                current_light=&mut root;
            },
        }
    }
    // println!("{:?}",current_light);
    current_light=&mut root;
    let light_copy=current_light.clone();
    let (num,total_brightness)=counting(&light_copy);
    let average:f64;
    average=((total_brightness as f64)/(num as f64)).floor();
    
    println!("{:.0}", average);

}
//[Set(75), Left, Set(33), Reset, Right, Set(67), Left, Set(25)]
//50

//[Left, Left, Reset, Left, Right, Reset, Right, Left, Reset, Right, Right]
//0

//[Left, Set(100), Reset, Right, Set(100)]