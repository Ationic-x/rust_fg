use std::sync::MutexGuard;

use gfx_device_gl::Resources;
use piston::Key;
use piston_window::{G2dTextureContext, Texture};
use sprite::Sprite;

use super::{character::{air::manager::Clsn, character::Character}, input::manager::InputManager};

const PAUSE_DURATION: i32 = 3;

/// Estructura que representa el jugador en el juego.
pub struct Player {
    /// Indica si es el primer o segundo jugador
    first_player: bool,
    /// Gestiona los inputs del jugador
    input_manager: InputManager,
    /// Mapeo de teclas personalizado del jugador
    key_map: KeyMap,
    /// Personaje elegido por el jugador
    character: Option<Character>,
    /// Estado donde se puede modificar ciertos inputs (para casos de teclas dobles)
    replace_action: bool,
    /// Franja de tiempo donde permite modificar ciertos input (para casos de teclas dobles)
    replace_timer: i32,
}

/// Estructura que represental el mapeo de teclas para el jugador.
pub struct KeyMap {
    /// Puño débil
    lp: Key,
    /// Puño medio
    mp: Key,
    /// Puño fuerte
    hp: Key,
    /// Patada baja
    lk: Key,
    /// Patada media
    mk: Key,
    /// Patada fuerte
    hk: Key,
    /// Adelante
    f: Key,
    /// Arriba
    u: Key,
    /// Atrás
    b: Key,
    /// Abajo
    d: Key,
    /// Inicio/Taunt
    start: Key,
}

impl KeyMap {
    fn new(first_player: bool) -> Self {
        if first_player {
            return Self {
                lp: Key::A,
                mp: Key::S,
                hp: Key::D,
                lk: Key::Z,
                mk: Key::X,
                hk: Key::C,
                f: Key::Right,
                u: Key::Up,
                b: Key::Left,
                d: Key::Down,
                start: Key::RShift,
            };
        }
        Self {
            lp: Key::R,
            mp: Key::T,
            hp: Key::Y,
            lk: Key::F,
            mk: Key::G,
            hk: Key::H,
            f: Key::L,
            u: Key::I,
            b: Key::J,
            d: Key::K,
            start: Key::RCtrl,
        }
    }

    fn translate(&self, key: &Key, flip: bool) -> &str {
        match *key {
            k if k == self.lp => "lp",
            k if k == self.mp => "mp",
            k if k == self.hp => "hp",
            k if k == self.lk => "lk",
            k if k == self.mk => "mk",
            k if k == self.hk => "hk",
            k if k == self.f => if flip {"b"} else {"f"},
            k if k == self.u => "u",
            k if k == self.b => if flip {"f"} else {"b"},
            k if k == self.d => "d",
            k if k == self.start => "start",
            _ => "",
        }
    }
}

impl Player {
     /// Crea una nueva instancia de `Player`.
    ///
    /// # Argumentos
    ///
    /// * `first_player` - Un booleano que indica si el jugador es el primer jugador o no.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `Player`.
    pub fn new(first_player: bool) -> Self {
        Self {
            first_player,
            input_manager: InputManager::new(),
            key_map: KeyMap::new(first_player),
            character: None,
            replace_action: false,
            replace_timer: 0,
        }
    }

    /// Indica si el jugador es el primer jugador.
    ///
    /// # Retorna
    ///
    /// `true` si el jugador es el primer jugador, de lo contrario, `false`.
    pub fn is_first_player(&self) -> bool {
        self.first_player
    }

    /// Obtiene la vida del jugador.
    ///
    /// # Retorna
    ///
    /// La vida del jugador.
    pub fn get_life(&self) -> i32 {
        self.character.as_ref().unwrap().get_life()
    }

    /// Establece si el jugador ganó.
    ///
    /// # Argumentos
    ///
    /// * `win` - Un booleano que indica si el jugador ganó o no.
    pub fn set_win(&mut self, win: bool) {
        self.character.as_mut().unwrap().set_win(win);
    }

    /// Establece si el jugador perdió.
    ///
    /// # Argumentos
    ///
    /// * `lose` - Un booleano que indica si el jugador perdió o no.
    pub fn set_lose(&mut self, lose: bool) {
        self.character.as_mut().unwrap().set_lose(lose);
    }

    /// Obtiene el poder del jugador como un porcentaje.
    ///
    /// # Retorna
    ///
    /// El poder del jugador como un porcentaje.
    pub fn get_power_as_percentage(&self) -> f64 {
        self.character.as_ref().unwrap().get_power_as_percentage()
    }

    /// Obtiene la vida del jugador como un porcentaje.
    ///
    /// # Retorna
    ///
    /// La vida del jugador como un porcentaje.
    pub fn get_life_as_percentage(&self) -> f64 {
        self.character.as_ref().unwrap().get_life_as_percentage()
    }

    /// Maneja la colisión entre dos jugadores.
    ///
    /// # Argumentos
    ///
    /// * `p1` - Una referencia mutable al primer jugador.
    /// * `p2` - Una referencia mutable al segundo jugador.
    pub fn check_collision(p1: &mut Player, p2: &mut Player) {
        let mut p1_hit = false;
        let mut p2_hit = false;
        let mut collision = false;
        for clsn_p1 in p1.get_clsns() {
            for clsn_p2 in p2.get_clsns() {
                if clsn_p1.collides(clsn_p2) {
                    if !clsn_p1.is_hitbox() {
                        p1_hit = true;
                    }
                    if !clsn_p2.is_hitbox() {
                        p2_hit = true;
                    }
                    if p1_hit && p2_hit {
                        break;
                    }
                    collision = true;
                }
            }
        }
        let character_p1 = p1.character.as_mut().unwrap();
        let character_p2 = p2.character.as_mut().unwrap();
        
        Character::set_distance(character_p1, character_p2);
        if collision {
            Character::normal_collision_handler(character_p1, character_p2);
        }
        if p1_hit {
            character_p1.collision_handler(character_p2);
        }
        if p2_hit {
            character_p2.collision_handler(character_p1);
        }
    }

    /// Establece el estado de una tecla del jugador.
    ///
    /// # Argumentos
    ///
    /// * `key` - Una referencia a la tecla que se está estableciendo.
    /// * `value` - El valor booleano que indica si la tecla está presionada (`true`) o liberada (`false`).
    ///
    /// # Retorna
    ///
    /// El estado anterior de la tecla antes de que se estableciera.
    pub fn set_player_input(&mut self, key: &Key, value: bool) -> bool {
        let symbol = self.key_map.translate(key, self.character.as_ref().unwrap().is_flipped());
        if symbol == "" {
            return value;
        }
        self.input_manager.player_input.set_state(symbol, value)
    }

    /// Elige un personaje para el jugador.
    ///
    /// # Argumentos
    ///
    /// * `char_name` - El nombre del personaje.
    /// * `context` - El contexto de textura de G2d.
    pub fn choose_char(&mut self, char_name: &str, context: G2dTextureContext) {
        let mut character = Character::new(char_name, context);
        if !self.first_player {
            character.set_as_second_player();
        }
        self.character = Some(character);
    }

    /// Establece la paleta del personaje.
    ///
    /// # Argumentos
    ///
    /// * `palette_index` - El índice de la paleta.
    pub fn set_palette(&mut self, palette_index: usize) {
        self.character.as_mut().unwrap().set_palette(palette_index);
    }

    /// Actualiza el estado del jugador.
    pub fn update(&mut self) {
        self.input_manager.update_hold_key();
        if self.character.as_mut().unwrap().is_flipping() {
            self.input_manager.flip();
        }

        if self.replace_timer < PAUSE_DURATION {
            self.replace_timer += 1;
        } else if self.replace_action {
            let action = self
                .input_manager
                .walk_input_buffer(self.character.as_ref().unwrap().get_cmd());
            self.character.as_mut().unwrap().set_action(action);
            self.input_manager.clear();
            self.replace_action = false;
        }

        self.character
            .as_mut()
            .unwrap()
            .set_direction(self.input_manager.get_active_direction());
        self.character.as_mut().unwrap().update();
    }

     /// Maneja la entrada de teclado del jugador.
    ///
    /// # Argumentos
    ///
    /// * `ticks` - Una referencia mutable al contador de ticks que representa el tiempo actual del juego.
    /// * `replace` - Un booleano que indica si reemplazar la última entrada en el búfer si es necesario.
    pub fn handle_key_input(&mut self, ticks: &mut u16, replace: bool) {
        self.input_manager
            .handle_key_input(ticks, if replace { self.replace_action } else { false });
    }

    /// Indica si el jugador está reemplazando una acción.
    ///
    /// # Retorna
    ///
    /// `true` si el jugador está reemplazando una acción, de lo contrario, `false`.
    pub fn is_replacing_action(&self) -> bool {
        self.replace_action
    }

    /// Indica si el jugador está en movimiento.
    ///
    /// # Argumentos
    ///
    /// * `key` - La tecla que se está verificando.
    ///
    /// # Retorna
    ///
    /// `true` si el jugador está en movimiento con la tecla especificada, de lo contrario, `false`.
    pub fn is_moving(&self, key: Key) -> bool {
        if key == self.key_map.f || key == self.key_map.b || key == self.key_map.u || key == self.key_map.d {
            return true;
        }
        false
    }

    /// Establece que el jugador está reemplazando una acción.
    pub fn set_replacing(&mut self) {
        self.replace_action = true;
        self.replace_timer = 0;
    }

    /// Obtiene una referencia mutable al sprite del jugador.
    ///
    /// # Retorna
    ///
    /// Una referencia mutable al sprite del jugador.
    pub fn get_mut_sprite(&mut self) -> MutexGuard<Sprite<Texture<Resources>>> {
        self.character.as_mut().unwrap().get_sprite()
    }

    /// Obtiene una referencia a los colisionadores del jugador.
    ///
    /// # Retorna
    ///
    /// Una referencia a los colisionadores del jugador.
    pub fn get_clsns(&self) -> &Vec<Clsn> {
        self.character.as_ref().unwrap().get_animation_table().get_clsns()
    }

    /// Indica si el jugador está realizando una acción.
    ///
    /// # Retorna
    ///
    /// `true` si el jugador está realizando una acción, de lo contrario, `false`.
    pub fn is_doing_action(&self) -> bool {
        self.character.as_ref().unwrap().has_control()
    }
}
