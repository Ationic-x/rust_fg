use constants::constants::*;

use crate::chars::Character;

use super::State;
const WINDOW_SIZE: f64 = 512.0;

impl Character for CharData {
    fn new() -> Self {
        Self {
            name: "KFM".to_string(),
            life: LIFE,
            power: 3000,
            state: State::S,
            attack: State::S,
            ctrl: true,
            state_no: -1,
            air_time: 0,
            double_jump: false,
            move_contact: false,
            run: false,
            anim: 0,
            //fx: 0,
            action: "".to_string(),
            time: 0,
            anim_time: 0,
            anim_elem: 0,
            new_anim: true,
            offset_x: 0.0,
            offset_y: 0.0,
            width: 0,
            x: 50.0,
            y: 500.0,
            vel_x: 0.0,
            vel_y: 0.0,
            def: true,
            direction: 5,
            jumps: 1,
            previous_flip: false,
            current_flip: false,
            flip_x: false,
            distance: -1.0,
            hit: 0,
            fall: false,
            win: false,
            lose: false,
        }
    }

    fn get_hit(&self) -> i32 {
        self.hit
    }

    fn get_life_as_percentage(&self) -> f64 {
        self.life as f64 / LIFE as f64 * 100.0
    }

    fn get_power_as_percentage(&self) -> f64 {
        self.power as f64 / POWER as f64 * 100.0
    }

    fn get_air_name(&self) -> &'static str {
        AIR
    }
    fn get_sff_name(&self) -> &'static str {
        SFF
    }
    fn get_cmd_name(&self) -> &'static str {
        CMD
    }

    fn set_action(&mut self, action: String) {
        self.action = action;
    }

    fn set_direction(&mut self, direction: u8) {
        self.direction = direction;
    }

    fn get_direction(&self) -> u8 {
        self.direction
    }

    fn set_data(&mut self) {
        char::trigger(self);
    }

    fn set_distance(&mut self, distance: f64) {
        self.distance = distance;
    }

    fn set_state_no(&mut self, number: i32) {
        self.state_no = number;
    }

    fn get_state_no(&self) -> i32 {
        self.state_no
    }

    fn get_anim(&self) -> &i32 {
        &self.anim
    }

    fn get_time(&self) -> i32 {
        self.time
    }

    fn get_anim_time(&self) -> i32 {
        self.anim_time
    }

    fn set_time(&mut self, time: i32) {
        self.time = time
    }

    fn set_anim_time(&mut self, time: i32) {
        self.anim_time = time;
    }

    fn set_anim_element(&mut self, element: i32) {
        self.anim_elem = element;
    }

    fn get_new_anim(&self) -> bool {
        self.new_anim
    }

    fn set_new_anim(&mut self, bool: bool) {
        self.new_anim = bool;
    }

    fn has_control(&self) -> bool {
        self.ctrl
    }

    fn get_distance(&self) -> f64 {
        self.distance
    }

    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn get_width(&self) -> u16 {
        self.width
    }

    fn get_offset_x(&self) -> f64 {
        self.offset_x
    }

    fn get_offset_y(&self) -> f64 {
        self.offset_y
    }

    fn was_flipped(&self) -> bool {
        self.previous_flip
    }

    fn is_flipped(&self) -> bool {
        self.current_flip
    }

    fn set_offset_x(&mut self, x: f64) {
        self.offset_x = x;
    }

    fn set_offset_y(&mut self, y: f64) {
        self.offset_y = y;
    }

    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    fn get_flip(&self) -> bool{
        self.current_flip
    }

    fn set_previous_flip(&mut self) {
        self.previous_flip = self.current_flip;
    }

    fn set_current_flip(&mut self, flip: bool) {
        self.current_flip = flip;
    }

    fn add_power(&mut self, power: i32) {
        let mut result = self.power as i32 + power;
        if result < 0 {
            result = 0;
        }
        if result > POWER as i32 {
            result = POWER as i32;
        }
        self.power = result as u32;
    }

    fn add_pos_x(&mut self, x: f64) {
        if self.current_flip {
            self.x -= x;
            if self.x < self.offset_x {
                self.x += x;
            }
        } else {
            self.x += x;
            if self.x > WINDOW_SIZE - self.width as f64 + self.offset_x {
                self.vel_x -= x;
            }
        }
    }

    fn set_vel_x(&mut self, x: f64) {
        if self.current_flip {
            self.vel_x = -x;
        } else {
            self.vel_x = x;
        }
    }

    fn set_hit(&mut self, hit_no: i32) {
        self.hit = hit_no;
    }

    fn update(&mut self) {
        if self.state != State::A {
            self.air_time = 0;
            self.vel_x *= CROUCH_FRICTION;
            if self.vel_x.abs() < CROUCH_FRICTION_THRESHOLD {
                self.vel_x = 0.0;
            }
        } else {
            self.air_time += 1;
        }
        self.x += self.vel_x;
        if self.vel_x < 0.0 && self.x < self.offset_x
            || self.vel_x > 0.0 && self.x > WINDOW_SIZE - self.offset_x
        {
            self.x -= self.vel_x;
            if self.state_no >= 5000 {
                self.vel_x = -self.vel_x;
            }
        }
        self.y += self.vel_y;
    }

    fn hit_handler(&mut self, char_target: &mut dyn Character) {
        char::hit_handler(self, char_target);
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_state(&self) -> &State {
        &self.state
    }

    fn add_vel_x(&mut self, x: f64) {
        if self.current_flip {
            self.vel_x += -x;
        } else {
            self.vel_x += x;
        }
    }

    fn add_vel_y(&mut self, y: f64) {
        self.vel_y += y;
    }

    fn get_vel(&self) -> f64 {
        (self.vel_x.powi(2) + self.vel_y.powi(2)).sqrt()
    }

    fn set_vel_y(&mut self, y: f64) {
        self.vel_y = y;
    }

    fn get_anim_elem(&self) -> i32 {
        self.anim_elem
    }

    fn get_flip_x(&self) -> bool {
        self.flip_x
    }

    fn set_fall(&mut self, fall: bool) {
        self.fall = fall;
    }

    fn set_state(&mut self, state: State) {
        self.state = state;
    }

    fn get_life(&self) -> i32 {
        self.life
    }

    fn add_life(&mut self, life: i32) {
        let tmp_life = self.life + life;
        if tmp_life < LIFE && tmp_life > 0 {
            self.life = tmp_life;
            return;
        }
        if tmp_life > LIFE {
            self.life = LIFE;
            return;
        }
        if tmp_life < 1 {
            if self.def {
                self.life = 1;
            } else {
                self.lose = true;
                self.life = 0;
            }
        }
    }

    fn set_def(&mut self, def_state: bool) {
        self.def = def_state;
    }

    fn get_lose(&self) -> bool {
        self.win
    }

    fn get_win(&self) -> bool {
        self.lose
    }

    fn set_win(&mut self, win: bool) {
        self.win = win;
    }

    fn set_lose(&mut self, lose: bool) {
        self.lose = lose;
    }

    fn set_width(&mut self, width: u16) {
        self.width = width;
    }
}

pub struct CharData {
    name: String,
    pub life: i32,
    power: u32,
    state: State,
    ctrl: bool,
    state_no: i32,
    air_time: i32,
    double_jump: bool,
    move_contact: bool,
    run: bool,
    attack: State,
    anim: i32,
    //fx: i32,
    action: String,
    time: i32,
    anim_time: i32,
    anim_elem: i32,
    new_anim: bool,
    offset_x: f64,
    offset_y: f64,
    x: f64,
    y: f64,
    vel_x: f64,
    vel_y: f64,
    def: bool,
    direction: u8,
    jumps: i32,
    previous_flip: bool,
    current_flip: bool,
    flip_x: bool,
    distance: f64,
    hit: i32,
    fall: bool,
    win: bool,
    lose: bool,
    width: u16,
}

pub mod char;
pub mod constants;
