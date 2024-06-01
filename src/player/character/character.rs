use std::{rc::Rc, sync::MutexGuard};

use gfx_device_gl::Resources;
use image::RgbaImage;
use piston_window::{G2dTextureContext, Texture, TextureSettings};
use sprite::Sprite;

use crate::chars;

use super::{
    air::{self, manager::AnimationTable},
    cmd::{self, manager::CommandNode},
};

pub struct Character {
    char: Box<dyn chars::Character>,
    at: AnimationTable,
    cmd: CommandNode,
}

impl Character {
    pub fn new(char_name: &str, mut context: G2dTextureContext) -> Self {
        let img_buffer = &RgbaImage::from_raw(1, 1, vec![0, 0, 0, 0]).unwrap();
        let pw = piston_window::Texture::from_image(
            &mut context,
            &img_buffer,
            &TextureSettings::new().filter(piston_window::Filter::Nearest),
        )
        .unwrap();
        let mut empty_sprite = Sprite::from_texture(Rc::new(pw));
        empty_sprite.set_anchor(0.0, 0.0);
        empty_sprite.set_scale(1.0, 1.0);

        let char = chars::get_char(char_name).unwrap();
        let char_path = "src/chars/".to_string() + char_name + "/";
        let mut at = air::manager::parse_air(&(char_path.clone() + char.get_air_name() + ".air"));
        let cmd = cmd::manager::create_command_tree(&(char_path + char.get_cmd_name() + ".cmd"));

        at.set_sff(
            char_name,
            char.get_sff_name().to_string() + ".sff",
            true,
            context,
        );
        at.set_sprite(empty_sprite);

        Self { char, at, cmd }
    }

    pub fn set_palette(&mut self, palette_index: usize) {
        self.at.set_palette(palette_index);
    }

    pub fn normal_collision_handler(p1: &mut Character, p2: &mut Character) {
        let p1_vel = p1.char.get_vel();
        let p1_x = p1.char.get_x();
        let p2_vel = p2.char.get_vel();
        let p2_x = p2.char.get_x();
        let sum_vel = p1_vel + p2_vel;
        if p1_vel > p2_vel {
            if p2_x > p2.char.get_offset_x() && p2_x < 512.0 - p2.char.get_offset_x() {
                p1.char.add_pos_x(-p2_vel);
                p2.char.add_pos_x(-sum_vel);
            } else {
                p1.char.add_pos_x(-sum_vel);
            }
        }
        if p2_vel > p1_vel {
            if p1_x > p1.char.get_offset_x() && p1_x < 512.0 - p1.char.get_offset_x() {
                p1.char.add_pos_x(-sum_vel);
                p2.char.add_pos_x(-p1_vel);
            } else {
                p2.char.add_pos_x(-sum_vel);
            }
        }
        if p1_vel == p2_vel {
            p1.char.add_pos_x(-p1_vel);
            p2.char.add_pos_x(-p2_vel);
        }
    }

    pub fn set_distance(p1: &mut Character, p2: &mut Character) {
        let p1_x = p1.char.get_x();
        let p1_y = p1.char.get_y();
        let p2_x = p2.char.get_x();
        let p2_y = p2.char.get_y();
        let mut distance = ((p1_x - p2_x).powi(2) + (p1_y - p2_y).powi(2)).sqrt();
        if p1_x < p2_x {
            distance = -distance;
        };
        p1.char.set_distance(distance);
        p2.char.set_distance(-distance);
    }

    pub fn set_action(&mut self, action: String) {
        self.char.set_action(action);
    }

    pub fn set_direction(&mut self, direction: u8) {
        self.char.set_direction(direction);
    }

    pub fn get_animation_table(&self) -> &AnimationTable {
        &self.at
    }

    pub fn get_sprite(&mut self) -> MutexGuard<Sprite<Texture<Resources>>> {
        self.at.get_sprite()
    }

    pub fn is_flipped(&self) -> bool {
        self.char.is_flipped()
    }

    pub fn is_flipping(&mut self) -> bool {
        if self.char.was_flipped() != self.char.is_flipped() {
            self.char.set_previous_flip();
            return true;
        }
        false
    }

    pub fn has_control(&self) -> bool {
        self.char.has_control()
    }

    pub fn get_cmd(&self) -> &CommandNode {
        &self.cmd
    }

    pub fn set_as_second_player(&mut self) {
        self.char.set_x(450.0);
        self.at.get_sprite().set_anchor(1.0, 0.0);
        self.char.set_current_flip(true);
        self.char.set_previous_flip();
        self.char.set_distance(1.0);
    }

    pub fn get_life(&self) -> i32 {
        self.char.get_life()
    }

    pub fn set_win(&mut self, win: bool) {
        self.char.set_win(win);
    }

    pub fn set_lose(&mut self, lose: bool) {
        self.char.set_lose(lose);
    }

    pub fn get_power_as_percentage(&self) -> f64 {
        self.char.get_power_as_percentage()
    }

    pub fn get_life_as_percentage(&self) -> f64 {
        self.char.get_life_as_percentage()
    }

    pub fn update(&mut self) {
        self.char.set_data();
        self.char.update();
        self.at.update_sprite(&mut self.char);
        let x;
        if self.char.is_flipped() {
            x = self.char.get_x() + self.char.get_offset_x();
        } else {
            x = self.char.get_x() - self.char.get_offset_x();
        }
        self.at
            .get_sprite()
            .set_position(x, self.char.get_y() - self.char.get_offset_y());
    }

    pub fn collision_handler(&mut self, player_target: &mut Character) {
        let current_frame = self.char.get_anim_elem();
        if self.char.get_hit() != current_frame {
            self.char.set_hit(current_frame);
            self.char.hit_handler(&mut *player_target.char);
        }
    }
}
