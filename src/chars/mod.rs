pub mod kfm;

/// Rasgo que representa las operaciones básicas de un personaje.
pub trait Character {
    /// Crea un nuevo `CharData` con valores predeterminados.
    ///
    /// # Retorna
    ///
    /// Devuelve una instancia de `CharData`.
    fn new() -> Self where Self: Sized;

    /// Obtiene el número del estado que le ha impactado.
    ///
    /// # Retorna
    ///
    /// Retorna el número del estado.
    fn get_hit(&self) -> i32;

    /// Obtiene la vida del personaje como un porcentaje.
    ///
    /// # Retorna
    ///
    /// Retorna la vida del personaje como un porcentaje (0.0 - 100.0).
    fn get_life_as_percentage(&self) -> f64;

    /// Obtiene el poder del personaje como un porcentaje.
    ///
    /// # Retorna
    ///
    /// Retorna el poder del personaje como un porcentaje (0.0 - 100.0).
    fn get_power_as_percentage(&self) -> f64;

    /// Obtiene el nombre del archivo AIR.
    ///
    /// # Retorna
    ///
    /// Retorna el nombre del archivo AIR.
    fn get_air_name(&self) -> &'static str;

    /// Obtiene el nombre del archivo SFF.
    ///
    /// # Retorna
    ///
    /// Retorna el nombre del archivo SFF.
    fn get_sff_name(&self) -> &'static str;

    /// Obtiene el nombre del archivo CMD.
    ///
    /// # Retorna
    ///
    /// Retorna el nombre del archivo CMD.
    fn get_cmd_name(&self) -> &'static str;

    /// Establece la acción actual del personaje.
    ///
    /// # Argumentos
    ///
    /// * `action` - La acción que se establecerá.
    fn set_action(&mut self, action: String);

    /// Establece la dirección del personaje.
    ///
    /// # Argumentos
    ///
    /// * `direction` - La dirección del personaje (0-255).
    fn set_direction(&mut self, direction: u8);

    /// Obtiene la dirección actual del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna la dirección actual del personaje.
    fn get_direction(&self) -> u8;

    /// Actualiza los datos del personaje en base a unas condiciones.
    fn update_data(&mut self);

    /// Establece la distancia del personaje respecto a otro.
    ///
    /// # Argumentos
    ///
    /// * `distance` - La distancia entre el personaje y otro.
    fn set_distance(&mut self, distance: f64);

    /// Establece el número de estado del personaje.
    ///
    /// # Argumentos
    ///
    /// * `number` - El número de estado que se establecerá.
    fn set_state_no(&mut self, number: i32);

    /// Obtiene el número de estado actual del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna el número de estado actual del personaje.
    fn get_state_no(&self) -> i32;

    /// Obtiene la animación actual del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna una referencia a la animación actual del personaje.
    fn get_anim(&self) -> &i32;

    /// Obtiene el tiempo de la animación del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna el tiempo de la animación del personaje.
    fn get_time(&self) -> i32;

    /// Establece el tiempo que lleva en la animación el personaje.
    ///
    /// # Argumentos
    ///
    /// * `time` - El tiempo de la animación.
    fn set_time(&mut self, time: i32);

    /// Establece el tiempo que lleva la animación del personaje.
    ///
    /// # Argumentos
    ///
    /// * `time` - El tiempo de la animación.
    fn set_anim_time(&mut self, time: i32);

    /// Establece el elemento de la animación del personaje.
    ///
    /// # Argumentos
    ///
    /// * `element` - El elemento de la animación.
    fn set_anim_element(&mut self, element: i32);

    /// Obtiene si la animación es nueva.
    ///
    /// # Retorna
    ///
    /// Retorna true si la animación es nueva, de lo contrario false.
    fn get_new_anim(&self) -> bool;

    /// Establece si la animación es nueva.
    ///
    /// # Argumentos
    ///
    /// * `new` - Valor booleano que indica si la animación es nueva o no.
    fn set_new_anim(&mut self, new: bool);

    /// Verifica si el personaje tiene control.
    ///
    /// # Retorna
    ///
    /// Retorna true si el personaje tiene control, de lo contrario false.
    fn has_control(&self) -> bool;

    /// Obtiene la distancia del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna la distancia del personaje.
    fn get_distance(&self) -> f64;

    /// Obtiene la posición `x` del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna la posición `x` del personaje.
    fn get_x(&self) -> f64;

    /// Obtiene la posición `y` del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna la posición `y` del personaje.
    fn get_y(&self) -> f64;

    /// Obtiene el desplazamiento `x` del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna el desplazamiento `x` del personaje.
    fn get_offset_x(&self) -> f64;

    /// Obtiene el desplazamiento `y` del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna el desplazamiento `y` del personaje.
    fn get_offset_y(&self) -> f64;

    /// Verifica si el personaje estaba volteado horizontalmente.
    ///
    /// # Retorna
    ///
    /// Retorna true si el personaje estaba volteado horizontalmente, de lo contrario false.
    fn was_flipped(&self) -> bool;

    /// Verifica si el personaje está volteado horizontalmente.
    ///
    /// # Retorna
    ///
    /// Retorna true si el personaje está volteado horizontalmente, de lo contrario false.
    fn is_flipped(&self) -> bool;

     /// Establece el desplazamiento `x` del sprite.
    ///
    /// # Argumentos
    ///
    /// * `x` - El desplazamiento `x` del sprite.
    fn set_offset_x(&mut self, x: f64);

    /// Establece el desplazamiento `y` del sprite.
    ///
    /// # Argumentos
    ///
    /// * `y` - El desplazamiento `y` del sprite.
    fn set_offset_y(&mut self, y: f64);

    /// Establece la posición `x` del personaje.
    ///
    /// # Argumentos
    ///
    /// * `x` - La posición `x` del personaje.
    fn set_x(&mut self, x: f64);

    /// Establece la posición `y` del personaje.
    ///
    /// # Argumentos
    ///
    /// * `y` - La posición `y` del personaje.
    fn set_y(&mut self, y: f64);

    /// Obtiene si el personaje está volteado horizontalmente.
    ///
    /// # Retorna
    ///
    /// Retorna true si el personaje está volteado horizontalmente, de lo contrario false.
    fn get_flip(&self) -> bool;

    /// Establece el estado previo de volteo horizontal del personaje.
    fn set_previous_flip(&mut self);

    /// Establece el estado actual de volteo horizontal del personaje.
    ///
    /// # Argumentos
    ///
    /// * `flip` - El estado de volteo horizontal del personaje.
    fn set_current_flip(&mut self, flip: bool);

    /// Suma poder al personaje (puede ser negativo).
    ///
    /// # Argumentos
    ///
    /// * `power` - El poder que se agregará al personaje.
    fn add_power(&mut self, power: i32);

    /// Añade posición en el eje `x` al personaje.
    ///
    /// # Argumentos
    ///
    /// * `x` - La cantidad de posición que se añadirá al eje `x`.
    fn add_pos_x(&mut self, x: f64);

    /// Establece la velocidad en el eje `x` del personaje.
    ///
    /// # Argumentos
    ///
    /// * `x` - La velocidad en el eje `x` del personaje.
    fn set_vel_x(&mut self, x: f64);

    /// Establece el número del estado que impactó.
    ///
    /// # Argumentos
    ///
    /// * `hit_no` - El número del estado que impactó.
    fn set_hit(&mut self, hit_no: i32);

    /// Actualiza las posición del personaje.
    fn update_pos(&mut self);

    /// Maneja el impacto entre personajes.
    ///
    /// # Argumentos
    ///
    /// * `char_target` - El personaje objetivo del impacto.
    fn hit_handler(&mut self, char_target: &mut dyn Character);

    /// Obtiene el estado actual del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna una referencia al estado actual del personaje.
    fn get_state(&self) -> &State;

    /// Suma velocidad en el eje `x` al personaje (puede ser negativo).
    ///
    /// # Argumentos
    ///
    /// * `x` - La cantidad de velocidad que se sumará al eje `x`.
    fn add_vel_x(&mut self, x: f64);

    /// Suma velocidad en el eje `y` al personaje (puede ser negativo).
    ///
    /// # Argumentos
    ///
    /// * `y` - La cantidad de velocidad que se sumará al eje `y`.
    fn add_vel_y(&mut self, y: f64);

    /// Obtiene la velocidad total (contabiliza `x` e `y`) del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna la velocidad total del personaje.
    fn get_vel(&self) -> f64;

    /// Establece la velocidad en el eje `y` del personaje.
    ///
    /// # Argumentos
    ///
    /// * `y` - La velocidad en el eje `y` del personaje.
    fn set_vel_y(&mut self, y: f64);

    /// Obtiene el elemento de la animación del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna el elemento de la animación del personaje.
    fn get_anim_elem(&self) -> i32;

    /// Obtiene si el personaje está volteado horizontalmente.
    ///
    /// # Retorna
    ///
    /// Retorna true si el personaje está volteado horizontalmente, de lo contrario false.
    fn get_flip_x(&self) -> bool;

    /// Establece si el personaje está en estado de caída.
    ///
    /// # Argumentos
    ///
    /// * `fall` - Valor booleano que indica si el personaje está en estado de caída o no.
    fn set_fall(&mut self, fall: bool);

    /// Establece el estado actual del personaje.</br>
    /// Puede ser de pie, en el aire, tirado y agachado.
    ///
    /// # Argumentos
    ///
    /// * `state` - El estado actual del personaje.
    fn set_state(&mut self, state: State);

    /// Obtiene la vida actual del personaje.
    ///
    /// # Retorna
    ///
    /// Retorna la vida actual del personaje.
    fn get_life(&self) -> i32;

    /// Suma vida al personaje (puede ser un valor negativo).
    ///
    /// # Argumentos
    ///
    /// * `life` - La cantidad de vida que se sumará al personaje.
    fn add_life(&mut self, life: i32);

/// Establece si el personaje está en estado defensivo.
    ///
    /// # Argumentos
    ///
    /// * `def_state` - Un booleano que indica si el personaje está en estado defensivo (`true`) o no (`false`).
    fn set_def(&mut self, def_state: bool);

    /// Establece si el personaje ha ganado.
    ///
    /// # Argumentos
    ///
    /// * `win` - Un booleano que indica si el personaje ha ganado (`true`) o no (`false`).
    fn set_win(&mut self, win: bool);

    /// Establece si el personaje ha perdido.
    ///
    /// # Argumentos
    ///
    /// * `lose` - Un booleano que indica si el personaje ha perdido (`true`) o no (`false`).
    fn set_lose(&mut self, lose: bool);

    /// Establece el ancho del personaje.
    ///
    /// # Argumentos
    ///
    /// * `width` - El ancho del personaje como un entero sin signo de 16 bits.
    fn set_width(&mut self, width: u16);

    /// Verifica si el personaje está en contacto con una pared.
    ///
    /// # Retorna
    ///
    /// Un booleano que indica si el personaje está en contacto con una pared (`true`) o no (`false`).
    fn get_wall(&self) -> bool;
}

/// Obtiene un personaje basado en su nombre.
///
/// # Argumentos
///
/// * `char_name` - El nombre del personaje como una cadena de texto.
///
/// # Retorna
///
/// Un `Option` que contiene una caja (`Box`) que envuelve a un rasgo (`trait`) de personaje (`Character`)
/// si se encuentra un personaje con el nombre especificado, o `None` si no se encuentra ningún personaje.
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

/// Estados posibles de un personaje.
#[derive(Debug, PartialEq)]
pub enum State {
    /// Stand (de pie)
    S, 
    /// Crouch (agachado)
    C, 
    /// Air (en el aire)
    A, 
    /// Lie (tirado en el suelo)
    L
}

// #[derive(Debug, PartialEq)]
// pub enum Attack {
//     N,
//     NA, SA, HA,
//     NT, ST, HT
// }