pub mod kfm;
pub trait Character {
    fn new() -> Self where Self: Sized;

    fn get_sff_name(&self) -> &'static str;
    fn get_air_name(&self) -> &'static str;
    fn get_cmd_name(&self) -> &'static str;

    fn set_action(&mut self, command: String);
    fn set_direction(&mut self, direction: u8);
    fn hit_handler(&mut self, char_target: &mut dyn Character);

    fn set_data(&mut self);
    fn update(&mut self);
    fn set_offset_x(&mut self, x: f64);
    fn set_offset_y(&mut self, y: f64);
    fn set_x(&mut self, x: f64);
    fn add_pos_x(&mut self, x: f64);
    fn set_y(&mut self, y: f64);
    fn set_vel_x(&mut self, x: f64);
    fn add_vel_x(&mut self, x: f64);
    fn add_vel_y(&mut self, y: f64);
    fn set_state_no(&mut self, number: i32);
    fn set_distance(&mut self, distance: f64);

    
    fn get_anim_elem(&self) -> i32;
    fn get_state_no(&self) -> i32;
    fn get_time(&self) -> i32;
    fn get_direction(&self) -> u8;
    fn get_state(&self) -> &State;
    fn get_anim(&self) -> &i32;
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn was_flipped(&self) -> bool;
    fn is_flipped(&self) -> bool;
    fn get_distance(&self) -> f64;
    fn get_offset_x(&self) -> f64;
    fn get_width(&self) -> u16;
    fn get_offset_y(&self) -> f64;
    fn get_vel(&self) -> f64;
    fn get_new_anim(&self) -> bool;
    fn get_anim_time(&self) -> i32;
    fn get_name(&self) -> &String;
    fn has_control(&self) -> bool;
    fn get_flip(&self) -> bool;
    fn get_flip_x(&self) -> bool;
    fn get_hit(&self) -> i32;
    fn get_life(&self) -> i32;
    fn get_win(&self) -> bool;
    fn get_lose(&self) -> bool;
    fn get_life_as_percentage(&self) -> f64;
    fn get_power_as_percentage(&self) -> f64;
    
    fn add_life(&mut self, life: i32);
    fn set_time(&mut self, time: i32);
    fn set_previous_flip(&mut self);
    fn set_current_flip(&mut self, flip: bool);
    fn set_fall(&mut self, fall: bool);
    fn set_win(&mut self, win: bool);
    fn set_lose(&mut self, lose: bool);
    fn set_anim_time(&mut self, time: i32);
    fn set_anim_element(&mut self, element: i32);
    fn set_new_anim(&mut self, bool: bool);
    fn set_hit(&mut self, hit_no: i32);
    fn set_width(&mut self, width: u16);
    fn set_vel_y(&mut self, y: f64);
    fn add_power(&mut self, power: i32);
    fn set_state(&mut self, state: State);
    fn set_def(&mut self, def_state: bool);
}

pub fn get_char(char_name: &str) -> Option<Box<dyn Character>> {
    match char_name {
        "kfm" => {
            let kfm: kfm::CharData = Character::new();
            Some(Box::new(kfm))
        }
        _ => {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    S, C, A, L
}

// #[derive(Debug, PartialEq)]
// pub enum Attack {
//     N,
//     NA, SA, HA,
//     NT, ST, HT
// }