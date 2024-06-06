use constants::constants::*;

use crate::chars::Character;

use super::State;
const WINDOW_SIZE: f64 = 512.0;
/// Implementación del rasgo `Character` para la estructura `CharData`.
impl Character for CharData {
    /// Crea un nuevo `CharData` con valores predeterminados.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `CharData` con valores iniciales predeterminados.
    fn new() -> Self {
        Self {
            //name: "KFM".to_string(),
            life: LIFE,
            power: 3000,
            state: State::S,
            attack: State::S,
            ctrl: true,
            state_no: -1,
            air_time: 0,
            double_jump: false,
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
            wall: false,
        }
    }

    /// Obtiene el número del estado que le ha impactado.
    ///
    /// # Retorna
    ///
    /// El número del estado que ha impactado al personaje.
    fn get_hit(&self) -> i32 {
        self.hit
    }

    /// Obtiene la vida del personaje como un porcentaje.
    ///
    /// # Retorna
    ///
    /// El porcentaje de vida del personaje.
    fn get_life_as_percentage(&self) -> f64 {
        self.life as f64 / LIFE as f64 * 100.0
    }

    /// Obtiene el poder del personaje como un porcentaje.
    ///
    /// # Retorna
    ///
    /// El porcentaje de poder del personaje.
    fn get_power_as_percentage(&self) -> f64 {
        self.power as f64 / POWER as f64 * 100.0
    }

    /// Obtiene el nombre del archivo AIR.
    ///
    /// # Retorna
    ///
    /// El nombre del archivo AIR.
    fn get_air_name(&self) -> &'static str {
        AIR
    }

    /// Obtiene el nombre del archivo SFF.
    ///
    /// # Retorna
    ///
    /// El nombre del archivo SFF.
    fn get_sff_name(&self) -> &'static str {
        SFF
    }

    /// Obtiene el nombre del archivo CMD.
    ///
    /// # Retorna
    ///
    /// El nombre del archivo CMD.
    fn get_cmd_name(&self) -> &'static str {
        CMD
    }

    /// Establece la acción actual del personaje.
    ///
    /// # Argumentos
    ///
    /// * `action` - La acción actual del personaje.
    fn set_action(&mut self, action: String) {
        self.action = action;
    }

    /// Establece la dirección del personaje.
    ///
    /// # Argumentos
    ///
    /// * `direction` - La dirección del personaje.
    fn set_direction(&mut self, direction: u8) {
        self.direction = direction;
    }

    /// Obtiene la dirección actual del personaje.
    ///
    /// # Retorna
    ///
    /// La dirección actual del personaje.
    fn get_direction(&self) -> u8 {
        self.direction
    }

    /// Actualiza los datos del personaje en base a unas condiciones.
    ///
    /// Esta función actualiza los datos del personaje según las condiciones actuales.
    fn update_data(&mut self) {
        char::trigger(self);
    }

    /// Establece la distancia del personaje respecto a otro.
    ///
    /// # Argumentos
    ///
    /// * `distance` - La distancia del personaje respecto a otro.
    fn set_distance(&mut self, distance: f64) {
        self.distance = distance;
    }

    /// Establece el número de estado del personaje.
    ///
    /// # Argumentos
    ///
    /// * `number` - El número de estado del personaje.
    fn set_state_no(&mut self, number: i32) {
        self.state_no = number;
    }

    /// Obtiene el número de estado actual del personaje.
    ///
    /// # Retorna
    ///
    /// El número de estado actual del personaje.
    fn get_state_no(&self) -> i32 {
        self.state_no
    }

    /// Obtiene la animación actual del personaje.
    ///
    /// # Retorna
    ///
    /// La animación actual del personaje.
    fn get_anim(&self) -> &i32 {
        &self.anim
    }

    /// Obtiene el tiempo de la animación del personaje.
    ///
    /// # Retorna
    ///
    /// El tiempo de la animación del personaje.
    fn get_time(&self) -> i32 {
        self.time
    }

    /// Establece el tiempo que lleva en la animación el personaje.
    ///
    /// # Argumentos
    ///
    /// * `time` - El tiempo que lleva en la animación el personaje.
    fn set_time(&mut self, time: i32) {
        self.time = time;
    }

    /// Establece el tiempo que lleva la animación del personaje.
    ///
    /// # Argumentos
    ///
    /// * `time` - El tiempo que lleva la animación del personaje.
    fn set_anim_time(&mut self, time: i32) {
        self.anim_time = time;
    }

    /// Establece el elemento de la animación del personaje.
    ///
    /// # Argumentos
    ///
    /// * `element` - El elemento de la animación del personaje.
    fn set_anim_element(&mut self, element: i32) {
        self.anim_elem = element;
    }

    /// Obtiene si la animación es nueva.
    ///
    /// # Retorna
    ///
    /// `true` si la animación es nueva, `false` en caso contrario.
    fn get_new_anim(&self) -> bool {
        self.new_anim
    }

    /// Establece si la animación es nueva.
    ///
    /// # Argumentos
    ///
    /// * `bool` - Booleano que indica si la animación es nueva.
    fn set_new_anim(&mut self, bool: bool) {
        self.new_anim = bool;
    }

    /// Verifica si el personaje tiene control.
    ///
    /// # Retorna
    ///
    /// `true` si el personaje tiene control, `false` en caso contrario.
    fn has_control(&self) -> bool {
        self.ctrl
    }

    /// Obtiene la distancia del personaje.
    ///
    /// # Retorna
    ///
    /// La distancia del personaje.
    fn get_distance(&self) -> f64 {
        self.distance
    }

    /// Obtiene la posición `x` del personaje.
    ///
    /// # Retorna
    ///
    /// La posición `x` del personaje.
    fn get_x(&self) -> f64 {
        self.x
    }

    /// Obtiene la posición `y` del personaje.
    ///
    /// # Retorna
    ///
    /// La posición `y` del personaje.
    fn get_y(&self) -> f64 {
        self.y
    }

    /// Obtiene el desplazamiento `x` del personaje.
    ///
    /// # Retorna
    ///
    /// El desplazamiento `x` del personaje.
    fn get_offset_x(&self) -> f64 {
        self.offset_x
    }

    /// Obtiene el desplazamiento `y` del personaje.
    ///
    /// # Retorna
    ///
    /// El desplazamiento `y` del personaje.
    fn get_offset_y(&self) -> f64 {
        self.offset_y
    }

    /// Verifica si el personaje estaba volteado horizontalmente.
    ///
    /// # Retorna
    ///
    /// `true` si el personaje estaba volteado horizontalmente, `false` en caso contrario.
    fn was_flipped(&self) -> bool {
        self.previous_flip
    }

    /// Verifica si el personaje está volteado horizontalmente.
    ///
    /// # Retorna
    ///
    /// `true` si el personaje está volteado horizontalmente, `false` en caso contrario.
    fn is_flipped(&self) -> bool {
        self.current_flip
    }

    /// Establece el desplazamiento `x` del sprite.
    ///
    /// # Argumentos
    ///
    /// * `x` - El desplazamiento `x` del sprite.
    fn set_offset_x(&mut self, x: f64) {
        self.offset_x = x;
    }

    /// Establece el desplazamiento `y` del sprite.
    ///
    /// # Argumentos
    ///
    /// * `y` - El desplazamiento `y` del sprite.
    fn set_offset_y(&mut self, y: f64) {
        self.offset_y = y;
    }

    /// Establece la posición `x` del personaje.
    ///
    /// # Argumentos
    ///
    /// * `x` - La posición `x` del personaje.
    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Establece la posición `y` del personaje.
    ///
    /// # Argumentos
    ///
    /// * `y` - La posición `y` del personaje.
    fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Obtiene si el personaje está volteado horizontalmente.
    ///
    /// # Retorna
    ///
    /// `true` si el personaje está volteado horizontalmente, `false` en caso contrario.
    fn get_flip(&self) -> bool {
        self.current_flip
    }

    /// Establece el estado previo de volteo horizontal del personaje.
    fn set_previous_flip(&mut self) {
        self.previous_flip = self.current_flip;
    }

    /// Establece el estado actual de volteo horizontal del personaje.
    ///
    /// # Argumentos
    ///
    /// * `flip` - El estado actual de volteo horizontal del personaje.
    fn set_current_flip(&mut self, flip: bool) {
        self.current_flip = flip;
    }

    /// Suma poder al personaje (puede ser negativo).
    ///
    /// # Argumentos
    ///
    /// * `power` - La cantidad de poder a sumar al personaje.
    ///
    /// Los límites del poder van del 0 a la definida por el personaje.
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

    /// Añade posición en el eje `x` al personaje.
    ///
    /// # Argumentos
    ///
    /// * `x` - La cantidad de posición en el eje `x` a añadir al personaje.
    ///
    /// Se tiene en cuenta las colisiones con las paredes.
    fn add_pos_x(&mut self, x: f64) {
        if self.current_flip {
            self.x -= x;
            if x > 0.0 && self.x < self.offset_x || x < 0.0 && self.x > WINDOW_SIZE - self.offset_x
            {
                self.wall = true;
                self.x += x;
            } else {
                self.wall = false;
            }
        } else {
            self.x += x;
            if x < 0.0 && self.x < self.offset_x || x > 0.0 && self.x > WINDOW_SIZE - self.offset_x
            {
                self.wall = true;
                self.x -= x;
            } else {
                self.wall = false;
            }
        }
    }

    /// Establece la velocidad en el eje `x` del personaje.
    ///
    /// # Argumentos
    ///
    /// * `x` - La velocidad en el eje `x` del personaje.
    fn set_vel_x(&mut self, x: f64) {
        self.vel_x = x;
    }

    /// Establece el número del estado que impactó.
    ///
    /// # Argumentos
    ///
    /// * `hit_no` - El número del estado que impactó.
    fn set_hit(&mut self, hit_no: i32) {
        self.hit = hit_no;
    }

    /// Actualiza las posición del personaje. <br>
    /// En base a la posición actual y estado, se le aplica un coeficiente de fricción con el suelo
    /// a la vez que se detecta cuando choca contra una pared o cuanto tiempo está en el aire.
    fn update_pos(&mut self) {
        if self.state != State::A {
            self.air_time = 0;
            self.vel_x *= CROUCH_FRICTION;
            if self.vel_x.abs() < CROUCH_FRICTION_THRESHOLD {
                self.vel_x = 0.0;
            }
        } else {
            self.air_time += 1;
        }
        if self.wall && self.vel_x < 0.0 && self.state_no > 4999 {
            self.vel_x = -self.vel_x;
        }
        self.add_pos_x(self.vel_x);
        self.y += self.vel_y;
    }

    /// Maneja el impacto entre personajes.
    ///
    /// # Argumentos
    ///
    /// * `char_target` - El personaje objetivo del impacto.
    fn hit_handler(&mut self, char_target: &mut dyn Character) {
        char::hit_handler(self, char_target);
    }

    /// Obtiene el estado actual del personaje.
    ///
    /// # Retorna
    ///
    /// Una referencia al estado actual del personaje.
    fn get_state(&self) -> &State {
        &self.state
    }

    /// Suma velocidad en el eje `x` al personaje (puede ser negativo).
    ///
    /// # Argumentos
    ///
    /// * `x` - La cantidad de velocidad en el eje `x` a sumar al personaje.
    fn add_vel_x(&mut self, x: f64) {
        self.vel_x += x;
    }

    /// Suma velocidad en el eje `y` al personaje (puede ser negativo).
    ///
    /// # Argumentos
    ///
    /// * `y` - La cantidad de velocidad en el eje `y` a sumar al personaje.
    fn add_vel_y(&mut self, y: f64) {
        self.vel_y += y;
    }

    /// Obtiene la velocidad total (contabiliza `x` e `y`) del personaje.
    ///
    /// # Retorna
    ///
    /// La velocidad total del personaje.
    fn get_vel(&self) -> f64 {
        (self.vel_x.powi(2) + self.vel_y.powi(2)).sqrt()
    }

    /// Establece la velocidad en el eje `y` del personaje.
    ///
    /// # Argumentos
    ///
    /// * `y` - La velocidad en el eje `y` del personaje.
    fn set_vel_y(&mut self, y: f64) {
        self.vel_y = y;
    }

    /// Obtiene el elemento de la animación del personaje.
    ///
    /// # Retorna
    ///
    /// El elemento de la animación del personaje.
    fn get_anim_elem(&self) -> i32 {
        self.anim_elem
    }

    /// Obtiene si el personaje está volteado horizontalmente.
    ///
    /// # Retorna
    ///
    /// `true` si el personaje está volteado horizontalmente, `false` en caso contrario.
    fn get_flip_x(&self) -> bool {
        self.flip_x
    }

    /// Establece si el personaje está en estado de caída.
    ///
    /// # Argumentos
    ///
    /// * `fall` - Indica si el personaje está en estado de caída.
    fn set_fall(&mut self, fall: bool) {
        self.fall = fall;
    }

    /// Establece el estado actual del personaje.
    ///
    /// # Argumentos
    ///
    /// * `state` - El estado actual del personaje.
    fn set_state(&mut self, state: State) {
        self.state = state;
    }

    /// Obtiene la vida actual del personaje.
    ///
    /// # Retorna
    ///
    /// La vida actual del personaje.
    fn get_life(&self) -> i32 {
        self.life
    }

    /// Suma vida al personaje (puede ser un valor negativo).
    ///
    /// # Argumentos
    ///
    /// * `life` - La cantidad de vida a sumar al personaje.
    ///
    /// Los límites de la vida van del 0 a la definida por el personaje. Si llega a 0 el personaje a perdido.
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

    /// Establece si el personaje está en estado defensivo.
    ///
    /// # Argumentos
    ///
    /// * `def_state` - Indica si el personaje está en estado defensivo.
    fn set_def(&mut self, def_state: bool) {
        self.def = def_state;
    }

    /// Establece si el personaje ha ganado.
    ///
    /// # Argumentos
    ///
    /// * `win` - Indica si el personaje ha ganado.
    fn set_win(&mut self, win: bool) {
        self.win = win;
    }

    /// Establece si el personaje ha perdido.
    ///
    /// # Argumentos
    ///
    /// * `lose` - Indica si el personaje ha perdido.
    fn set_lose(&mut self, lose: bool) {
        self.lose = lose;
    }

    /// Establece el ancho del personaje.
    ///
    /// # Argumentos
    ///
    /// * `width` - El ancho del personaje.
    fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    /// Verifica si el personaje está en contacto con una pared.
    ///
    /// # Retorna
    ///
    /// `true` si el personaje está en contacto con una pared, `false` en caso contrario.
    fn get_wall(&self) -> bool {
        self.wall
    }
}

/// Estructura que representa los datos de un personaje.
pub struct CharData {
    //name: String,
    life: i32,
    power: u32,
    state: State,
    attack: State,
    ctrl: bool,
    state_no: i32,
    air_time: i32,
    double_jump: bool,
    run: bool,
    anim: i32,
    //fx: u32,
    action: String,
    time: i32,
    anim_time: i32,
    anim_elem: i32,
    new_anim: bool,
    offset_x: f64,
    offset_y: f64,
    width: u16,
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
    wall: bool,
}

pub mod char;
pub mod constants;
