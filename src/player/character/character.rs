use std::{rc::Rc, sync::MutexGuard};

use gfx_device_gl::Resources;
use image::RgbaImage;
use piston_window::{G2dTextureContext, Texture, TextureSettings};
use sprite::Sprite;

use crate::{
    chars::{self, State},
    error::pop_up::show_error_popup,
};

use super::{
    air::{self, manager::AnimationTable},
    cmd::{self, manager::CommandNode},
};

/// Estructura que representa un personaje en el juego.
pub struct Character {
    char: Box<dyn chars::Character>,
    at: AnimationTable,
    cmd: CommandNode,
}

impl Character {
    /// Crea un nuevo personaje.
    ///
    /// # Argumentos
    ///
    /// * `char_name` - Nombre del personaje.
    /// * `context` - Contexto de textura G2d.
    ///
    /// # Retorna
    ///
    /// Devuelve una nueva instancia de `Character` del nombre dado.
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
        let mut at =
            match air::manager::parse_air(&(char_path.clone() + char.get_air_name() + ".air")) {
                Ok(at) => at,
                Err(err) => {
                    show_error_popup(&err);
                    std::process::exit(1);
                }
            };

        let cmd =
            match cmd::manager::create_command_tree(&(char_path + char.get_cmd_name() + ".cmd")) {
                Ok(at) => at,
                Err(err) => {
                    show_error_popup(&err);
                    std::process::exit(1);
                }
            };

        at.set_sff(
            char_name,
            char.get_sff_name().to_string() + ".sff",
            true,
            context,
        );
        at.set_sprite(empty_sprite);

        Self { char, at, cmd }
    }

    /// Establece el índice de la paleta del personaje.
    ///
    /// # Argumentos
    ///
    /// * `palette_index` - Índice de la paleta.
    pub fn set_palette(&mut self, palette_index: usize) {
        self.at.set_palette(palette_index);
    }

    /// Manejador de colisiones normales entre dos personajes y sus hitboxes (no hurtbox).
    ///
    /// # Argumentos
    ///
    /// * `p1` - Primer personaje.
    /// * `p2` - Segundo personaje.
    pub fn normal_collision_handler(p1: &mut Character, p2: &mut Character) {
        let p1_vel = p1.char.get_vel();
        let p2_vel = p2.char.get_vel();
        let sum_vel = p1_vel + p2_vel;
        if p2.char.get_state() == &State::L {
            p1.char.add_pos_x(-sum_vel);
            return;
        }
        if p1.char.get_state() == &State::L {
            p2.char.add_pos_x(-sum_vel);
            return;
        }
        if p1_vel > p2_vel {
            p2.char.add_pos_x(-sum_vel);
            if p2.char.get_wall() {
                p1.char.add_pos_x(-sum_vel);
            } else {
                p1.char.add_pos_x(-p2_vel);
            }
        } else if p2_vel > p1_vel {
            p1.char.add_pos_x(-sum_vel);
            if p1.char.get_wall() {
                p2.char.add_pos_x(-sum_vel);
            } else {
                p2.char.add_pos_x(-p1_vel);
            }
        } else {
            p1.char.add_pos_x(-p1_vel);
            p2.char.add_pos_x(-p2_vel);
        }
    }

    /// Calcula y establece la distancia entre dos personajes.
    ///
    /// # Argumentos
    ///
    /// * `p1` - Primer personaje.
    /// * `p2` - Segundo personaje.
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

    /// Establece la acción que está realizando el personaje.
    ///
    /// # Argumentos
    ///
    /// * `action` - Nombre de la acción a establecer.
    pub fn set_action(&mut self, action: String) {
        self.char.set_action(action);
    }

    /// Establece la dirección del personaje.
    ///
    /// # Argumentos
    ///
    /// * `direction` - Dirección a establecer.
    pub fn set_direction(&mut self, direction: u8) {
        self.char.set_direction(direction);
    }

    /// Obtiene la tabla de animación del personaje.
    /// 
    /// # Retorna
    /// 
    /// Devuelve la tabla de aniamción.
    pub fn get_animation_table(&self) -> &AnimationTable {
        &self.at
    }

    /// Obtiene el sprite del personaje.
    /// 
    /// # Retorna
    /// 
    /// Retorna una referencia mutable del sprite del personaje.
    pub fn get_sprite(&mut self) -> MutexGuard<Sprite<Texture<Resources>>> {
        self.at.get_sprite()
    }

    /// Verifica si el personaje está volteado.
    /// 
    /// # Retorna
    /// 
    /// True si está girado y false si no lo está.
    pub fn is_flipped(&self) -> bool {
        self.char.is_flipped()
    }

    /// Verifica si el personaje está girando y actualizado su estado previo de giro.
    /// 
    /// # Retorna
    /// 
    /// True si está girando y false si no lo está.
    pub fn is_flipping(&mut self) -> bool {
        if self.char.was_flipped() != self.char.is_flipped() {
            self.char.set_previous_flip();
            return true;
        }
        false
    }

    /// Verifica si el personaje tiene control.
    /// 
    /// # Retorna
    /// 
    /// True si tiene control y false si no tiene control.
    pub fn has_control(&self) -> bool {
        self.char.has_control()
    }

    /// Obtiene el árbol de comandos del personaje.
    /// 
    /// # Retorna
    /// 
    /// El nodo de comandos desde el nivel 0.
    pub fn get_cmd(&self) -> &CommandNode {
        &self.cmd
    }

    /// Configura el personaje como segundo jugador.
    pub fn set_as_second_player(&mut self) {
        self.char.set_x(450.0);
        self.at.get_sprite().set_anchor(1.0, 0.0);
        self.char.set_current_flip(true);
        self.char.set_previous_flip();
        self.char.set_distance(1.0);
    }

    /// Obtiene la vida del personaje.
    /// 
    /// # Retorna
    /// 
    /// Retorna la vida actual del personaje.
    pub fn get_life(&self) -> i32 {
        self.char.get_life()
    }

    /// Establece si el personaje ganó.
    ///
    /// # Argumentos
    ///
    /// * `win` - Indica si el personaje ganó.
    pub fn set_win(&mut self, win: bool) {
        self.char.set_win(win);
    }

    /// Establece si el personaje perdió.
    ///
    /// # Argumentos
    ///
    /// * `lose` - Indica si el personaje perdió.
    pub fn set_lose(&mut self, lose: bool) {
        self.char.set_lose(lose);
    }

    /// Obtiene el poder del personaje como porcentaje.
    /// 
    /// # Retorna
    /// 
    /// Retorna el poder del personaje como porcentaje del (0.0 - 100.0).
    pub fn get_power_as_percentage(&self) -> f64 {
        self.char.get_power_as_percentage()
    }

    /// Obtiene la vida del personaje como porcentaje.
    /// 
    /// # Retorna
    /// 
    /// Retorna la vida del personaje como porcentaje del (0.0 - 100.0).
    pub fn get_life_as_percentage(&self) -> f64 {
        self.char.get_life_as_percentage()
    }

    /// Actualiza el estado del personaje.
    pub fn update(&mut self) {
        self.char.update_data();
        self.char.update_pos();
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

    /// Maneja las colisiones entre personajes donde haya una hurtbox.
    ///
    /// # Argumentos
    ///
    /// * `player_target` - Personaje objetivo para la colisión.
    pub fn collision_handler(&mut self, player_target: &mut Character) {
        let current_frame = self.char.get_anim_elem();
        if self.char.get_hit() != current_frame {
            self.char.set_hit(current_frame);
            self.char.hit_handler(&mut *player_target.char);
        }
    }
}
