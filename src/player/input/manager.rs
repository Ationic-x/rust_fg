use crate::{player::character::cmd, CK};

use cmd::manager::CommandNode;
use std::mem;

/// Representa diferentes comandos de ataque y sus valores asociados.
///
/// Esta enumeración define los diversos comandos de ataque que pueden ejecutarse en el juego,
/// junto con sus valores correspondientes. Estos comandos incluyen LP (Puño Ligero),
/// MP (Puño Medio), HP (Puño Pesado), LK (Patada Ligera), MK (Patada Mediana) y HK (Patada Pesada).
#[derive(Copy, Clone, Debug)]
enum Action {
    /// Acción de Puño Ligero
    LP = 1 << 4,
    /// Acción de Puño Medio
    MP = 1 << 5,
    /// Acción de Puño Pesado
    HP = 1 << 6,
    /// Acción de Patada Ligera
    LK = 1 << 7,
    /// Acción de Patada Media
    MK = 1 << 8,
    /// Acción de Patada Pesada
    HK = 1 << 9,
    /// Acción de Provocar o Inicio
    Start = 1 << 10,
}

/// Representa el movimiento direccional en el espacio 2D.
///
/// Esta enumeración define los posibles movimientos direccionales en el espacio 2D,
/// incluyendo adelante, atrás, arriba, abajo y varias combinaciones diagonales.
#[derive(Copy, Clone, Debug)]
enum Direction {
    /// Dirección hacia adelante.
    F = 5,
    /// Dirección hacia arriba.
    U = 2,
    /// Dirección hacia atrás.
    B = 6,
    /// Dirección hacia abajo.
    D = 9,
    /// Dirección diagonal arriba-atrás.
    UB = 8,
    /// Dirección diagonal arriba-adelante.
    UF = 7,
    /// Dirección diagonal abajo-adelante.
    DF = 14,
    /// Dirección diagonal abajo-atrás.
    DB = 15,
}

/// Almacena el estado de entrada de un jugador.
///
/// Esta estructura contiene el estado de entrada actual de un jugador, indicando si
/// se están presionando teclas específicas o entradas direccionales.
pub struct PlayerInput {
    /// Estado de la tecla Puño Ligero
    pub lp: bool,
    /// Estado de la tecla Puño Medio
    pub mp: bool,
    /// Estado de la tecla Puño Pesado
    pub hp: bool,
    /// Estado de la tecla Patada Ligera
    pub lk: bool,
    /// Estado de la tecla Patada Mediana
    pub mk: bool,
    /// Estado de la tecla Patada Pesada
    pub hk: bool,
    /// Estado de la tecla Adelante
    pub f: bool,
    /// Estado de la tecla Arriba
    pub u: bool,
    /// Estado de la tecla Atrás
    pub b: bool,
    /// Estado de la tecla Abajo
    pub d: bool,
    /// Estado de la tecla Arriba-Atrás
    pub ub: bool,
    /// Estado de la tecla Arriba-Adelante
    pub uf: bool,
    /// Estado de la tecla Abajo-Adelante
    pub df: bool,
    /// Estado de la tecla Abajo-Atrás
    pub db: bool,
    /// Estado de la tecla de Inicio o Provocar
    pub start: bool,
}

/// Representa una tecla de entrada junto con su tiempo de almacenamiento en búfer.
///
/// Esta estructura contiene una tecla de entrada junto con el tiempo que ha estado almacenada en búfer.
/// Se utiliza para rastrear la duración durante la cual se ha mantenido presionada una tecla particular.
#[derive(Debug, Clone, Copy)]
pub struct InputKey {
    /// Tecla de comando asociada con la entrada
    cmd_key: CK,
    /// Tiempo por el cual la entrada ha estado almacenada en búfer
    buff_time: u128,
}

/// Representa una secuencia de comandos de entrada.
///
/// Esta estructura representa una secuencia de comandos de entrada que se han almacenado en búfer,
/// formando una entrada de comando. Contiene una lista de teclas de entrada, una ventana de entrada
/// que indica la duración máxima durante la cual la entrada es válida, y una bandera para
/// rastrear si la entrada ha sido procesada.
#[derive(Clone, Debug)]
pub struct CommandInput {
    /// Lista de teclas de entrada en la secuencia
    keys: Vec<InputKey>,
    /// Duración máxima durante la cual la entrada es válida
    input_window: u16,
    // Bandera que indica si la entrada ha sido procesada
    walked: bool,
    /// Indica si se ha encontrado la entrada
    found: bool,
}

/// Gestiona la entrada del jugador y el almacenamiento de comandos.
///
/// Esta estructura maneja la entrada del jugador y administra el almacenamiento de comandos de entrada.
/// Mantiene el estado de entrada actual del jugador y almacena un búfer de
/// comandos de entrada para su procesamiento.
pub struct InputManager {
    /// Estado de entrada del jugador
    pub player_input: PlayerInput,
    /// Búfer para almacenar los comandos de entrada
    input_buffer: Vec<CommandInput>,
}

impl PlayerInput {
    /// Crea una nueva instancia de `PlayerInput` con todas las teclas inicialmente liberadas.
    ///
    /// # Retorna
    ///
    ///  Una nueva instancia de `PlayerInput` con todos los estados de tecla establecidos en `false`.
    fn new() -> Self {
        PlayerInput {
            lp: false,
            mp: false,
            hp: false,
            lk: false,
            mk: false,
            hk: false,
            f: false,
            u: false,
            b: false,
            d: false,
            ub: false,
            uf: false,
            df: false,
            db: false,
            start: false,
        }
    }

    /// Limpia todas las acciones del jugador.
    ///
    /// Restablece todos los estados de teclas de acción a `false`.
    fn clear_action(&mut self) {
        self.lp = false;
        self.mp = false;
        self.hp = false;
        self.lk = false;
        self.mk = false;
        self.hk = false;
        self.start = false;
    }

    /// Establece el estado de una tecla especificada.
    ///
    /// Establece el estado de la tecla especificada al valor booleano especificado.
    /// Devuelve el estado anterior de la tecla.
    ///
    /// # Argumentos
    ///
    /// * `symbol` - Una referencia a la tecla para establecer el estado.
    /// * `state` - El valor booleano que indica si la tecla está presionada (`true`) o liberada (`false`).
    ///
    /// # Retorna
    ///
    /// El estado anterior de la tecla antes de que se estableciera.
    pub fn set_state(&mut self, symbol: &str, state: bool) -> bool {
        let result;
        match symbol {
            "lp" => {
                result = self.lp;
                self.lp = state
            }
            "mp" => {
                result = self.mp;
                self.mp = state
            }
            "hp" => {
                result = self.hp;
                self.hp = state
            }
            "lk" => {
                result = self.lk;
                self.lk = state
            }
            "mk" => {
                result = self.mk;
                self.mk = state
            }
            "hk" => {
                result = self.hk;
                self.hk = state
            }
            "start" => {
                result = self.start;
                self.start = state
            }
            "u" => {
                result = self.u;
                self.u = state
            }
            "d" => {
                result = self.d;
                self.d = state
            }
            "b" => {
                result = self.b;
                self.b = state
            }
            "f" => {
                result = self.f;
                self.f = state
            }
            _ => return false, // Other cases are not considered
        }
        result
    }

    /// Recupera el estado de una tecla de comando especificada.
    ///
    /// Devuelve el estado actual de la tecla de comando especificada.
    ///
    /// # Argumentos
    ///
    /// * `cmd_key` - Una referencia a la tecla de comando cuyo estado se va a recuperar.
    ///
    /// # Retorna
    ///
    /// El estado actual de la tecla de comando especificada.
    fn get_state(&self, cmd_key: &CK) -> bool {
        match cmd_key {
            CK::DB => self.db,
            CK::D => self.d,
            CK::DF => self.df,
            CK::B => self.b,
            CK::F => self.f,
            CK::UB => self.ub,
            CK::U => self.u,
            CK::UF => self.uf,
            CK::LP => self.lp,
            CK::MP => self.mp,
            CK::HP => self.hp,
            CK::LK => self.lk,
            CK::MK => self.mk,
            CK::HK => self.hk,
            CK::Start => self.start,
        }
    }

    /// Convierte la entrada del jugador a una representación de bits.
    ///
    /// Convierte el estado actual de la entrada del jugador en una representación de 16 bits
    /// donde cada bit corresponde al estado de una tecla específica o una entrada direccional.
    ///
    /// # Retorna
    ///
    /// El estado actual de la entrada del jugador en 16 bits.
    fn to_bits(&self) -> u16 {
        let mut result = 0;
        if self.f {
            result += 5
        };
        if self.u {
            result += 2
        };
        if self.b {
            result += 6
        };
        if self.d {
            result += 9
        };
        if self.ub {
            result += 8
        };
        if self.uf {
            result += 7
        };
        if self.df {
            result += 14
        };
        if self.db {
            result += 15
        };
        if self.lp {
            result += 1 << 4
        };
        if self.mp {
            result += 1 << 5
        };
        if self.hp {
            result += 1 << 6
        };
        if self.lk {
            result += 1 << 7
        };
        if self.mk {
            result += 1 << 8
        };
        if self.hk {
            result += 1 << 9
        };
        if self.start {
            result += 1 << 10
        };
        result
    }
}

impl InputKey {
    /// Crea una nueva instancia de `InputKey` con la tecla de comando especificada.
    ///
    /// Devuelve una nueva instancia de `InputKey` inicializada con la tecla de comando proporcionada
    /// y un tiempo de búfer de cero.
    ///
    /// # Argumentos
    ///
    /// * `key` - La tecla de comando asociada con la entrada.
    ///
    /// # Retorna
    /// Una nueva instancia de `InputKey` por defecto usando la tecla designada.
    fn new(key: CK) -> Self {
        Self {
            cmd_key: key,
            buff_time: 0,
        }
    }

    /// Actualiza el tiempo de búfer de la tecla de entrada.
    ///
    /// Incrementa el tiempo de búfer de la tecla de entrada en una unidad.
    fn update(&mut self) {
        self.buff_time += 1;
    }

    /// Recupera una referencia a la tecla de comando asociada con la entrada.
    ///
    /// # Retorna
    ///
    /// Una referencia a la tecla de comando asociada con la entrada.
    pub fn get_cmd_key_ref(&self) -> &CK {
        &self.cmd_key
    }
}

impl CommandInput {
    /// Crea una nueva instancia de `CommandInput`.
    ///
    /// # Retorna
    /// Una nueva instancia de `CommandInput` inicalizada por defecto
    fn new() -> Self {
        Self {
            keys: Vec::new(),
            input_window: 0,
            walked: false,
            found: false,
        }
    }

    /// Recupera una referencia a la duración de la ventana de entrada.
    ///
    /// # Retorna
    /// Una referencia a la duración de la ventana de entrada, que indica la duración máxima
    /// durante la cual la entrada es válida.
    pub fn get_input_window_ref(&self) -> &u16 {
        &self.input_window
    }

    /// Recupera una referencia a la lista de teclas de entrada.
    ///
    /// # Retorna
    /// Una referencia a la lista de `keys` de entrada almacenadas en la instancia de `CommandInput`.
    pub fn get_keys_ref(&self) -> &Vec<InputKey> {
        &self.keys
    }

    /// Establece el estado de `found`.
    ///
    /// # Argumentos
    /// * `bool` - El nuevo estado de `found`.
    pub fn set_found(&mut self, bool: bool) {
        self.found = bool;
    }
}

impl InputManager {
    /// Crea una nueva instancia de `InputManager`.
    ///
    /// # Retorna
    /// Una nueva instancia de `InputManager` inicializada por defecto.
    pub fn new() -> Self {
        Self {
            player_input: PlayerInput::new(),
            input_buffer: Vec::new(),
        }
    }

    /// Actualiza la duración de retención de las teclas de entrada en el búfer de entrada.
    ///
    /// Itera a través de cada comando de entrada en el búfer de entrada e incrementa
    /// la duración de retención de las teclas de entrada que actualmente están presionadas.
    pub fn update_hold_key(&mut self) {
        for inputs in &mut self.input_buffer {
            for key in &mut inputs.keys {
                if self.player_input.get_state(&key.cmd_key) {
                    key.update();
                }
            }
        }
    }

    /// Voltea la dirección de entrada del jugador.
    ///
    /// Invierte la dirección de la entrada del jugador intercambiando los comandos de izquierda y derecha.

    pub fn flip(&mut self) {
        for inputs in &mut self.input_buffer {
            for key in &mut inputs.keys {
                if key.cmd_key == CK::B {
                    key.cmd_key = CK::F;
                } else if key.cmd_key == CK::F {
                    key.cmd_key = CK::B;
                }
            }
        }
        let f = self.player_input.f;
        let b = self.player_input.b;
        self.player_input.b = f;
        self.player_input.f = b;
    }

    /// Procesa el búfer de entrada en busca de secuencias de comandos válidas.
    ///
    /// Recorre el búfer de entrada en busca de secuencias de comandos válidas
    /// basadas en el árbol de nodos de comando proporcionado. Si se encuentra una
    /// secuencia válida, se ejecuta la acción correspondiente.
    ///
    /// # Argumentos
    ///
    /// * `tree` - Una referencia al árbol de nodos de comando utilizado para buscar secuencias de comandos válidas.
    ///
    /// # Retorna
    ///
    /// El nombre de la acción ejecutada si se encuentra una secuencia válida, de lo contrario, una cadena vacía.
    pub fn walk_input_buffer(&mut self, tree: &CommandNode) -> String {
        let input_buffer = &mut self.input_buffer;
        for pos in 0..input_buffer.len() {
            for input in &mut *input_buffer {
                input.set_found(false);
            }
            if let Some(input) = input_buffer.get_mut(pos) {
                if !input.walked {
                    input.walked = true;
                    if let Some(name) = tree.search(input_buffer, pos) {
                        return name.clone();
                    } else {
                        if let Some(name) = tree.search(input_buffer, pos) {
                            return name.clone();
                        }
                    }
                }
            }
        }
        return "".to_string();
    }

    /// Obtiene la dirección activa según la entrada del jugador.
    ///
    /// Determina la dirección activa basada en la entrada del jugador y el historial de entrada.
    ///
    /// # Retorna
    ///
    /// Un número entero que representa la dirección activa:
    /// * 1 - Abajo-Atrás (DB)
    /// * 2 - Abajo (D)
    /// * 3 - Abajo-Adelante (DF)
    /// * 4 - Atrás (B)
    /// * 5 - Neutro
    /// * 6 - Adelante (F)
    /// * 7 - Arriba-Atrás (UB)
    /// * 8 - Arriba (U)
    /// * 9 - Arriba-Adelante (UF)
    pub fn get_active_direction(&self) -> u8 {
        let player = &self.player_input;
        if self.input_buffer.len() > 1
            && (player.f || player.b || player.u || player.uf || player.ub)
        {
            if let (Some(last_b_one), Some(last)) = (
                self.input_buffer.get(self.input_buffer.len() - 2),
                self.input_buffer.last(),
            ) {
                let last_b_one_values = &last_b_one.get_keys_ref()[0];
                let last_values = &last.get_keys_ref()[0];

                if last_b_one_values.cmd_key == last_values.cmd_key {
                    match last_values.cmd_key {
                        CK::F => {
                            if last.input_window < 15 {
                                if last_values.buff_time > 1 {
                                    return 6;
                                } else {
                                    return 66;
                                }
                            }
                        }
                        CK::B => {
                            if last.input_window < 15 {
                                if last_values.buff_time > 1 {
                                    return 4;
                                } else {
                                    return 44;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        if player.db || (player.d && player.b) {
            return 1;
        }
        if player.df || (player.d && player.f) {
            return 3;
        }
        if player.ub || (player.u && player.b) {
            return 7;
        }
        if player.uf || (player.u && player.f) {
            return 9;
        }
        if player.b && !player.f {
            return 4;
        }
        if player.f && !player.b {
            return 6;
        }
        if player.u && !player.d {
            return 8;
        }
        if player.d && !player.u {
            return 2;
        }
        5
    }

    /// Limpia el estado de entrada del jugador y el búfer de entrada.
    ///
    /// Restablece todas las teclas de acción del jugador y vacía el búfer de entrada.
    pub fn clear(&mut self) {
        self.player_input.clear_action();
        self.input_buffer.clear();
    }

    /// Maneja la entrada del jugador y actualiza el búfer de entrada en consecuencia.
    ///
    /// Procesa la entrada del jugador, la convierte en comandos de entrada, y actualiza
    /// el búfer de entrada con la nueva entrada. Opcionalmente, reemplaza la última entrada en
    /// el búfer si el búfer excede su tamaño máximo.
    ///
    /// # Argumentos
    ///
    /// * `ticks` - Una referencia mutable al contador de ticks que representa el tiempo actual del juego.
    /// * `replace` - Un booleano que indica si reemplazar la última entrada en el búfer si es necesario.
    pub fn handle_key_input(&mut self, ticks: &mut u16, replace: bool) {
        let input_buffer = &mut self.input_buffer;
        let player_input = self.player_input.to_bits();
        let mut input = CommandInput::new();

        if input_buffer.len() > 32 {
            input_buffer.remove(0);
        }

        let actions = [
            (Action::LP, CK::LP),
            (Action::MP, CK::MP),
            (Action::HP, CK::HP),
            (Action::LK, CK::LK),
            (Action::MK, CK::MK),
            (Action::HK, CK::HK),
            (Action::Start, CK::Start),
        ];

        for (action, command_key) in &actions {
            if player_input & (*action as u16) != 0 {
                input.keys.push(InputKey::new(*command_key));
            }
        }

        let last_bits = player_input & 0b1111;

        let directions = [
            (Direction::U as u16, CK::U),
            (Direction::F as u16, CK::F),
            (Direction::D as u16, CK::D),
            (Direction::B as u16, CK::B),
            (Direction::UF as u16, CK::UF),
            (Direction::UB as u16, CK::UB),
            (Direction::DF as u16, CK::DF),
            (Direction::DB as u16, CK::DB),
        ];

        for (bits, command_key) in &directions {
            if last_bits == *bits {
                input.keys.push(InputKey::new(*command_key));
                break;
            }
        }

        if !input.keys.is_empty() {
            if replace {
                if let Some(last) = input_buffer.last_mut() {
                    input.input_window = last.input_window;
                    *last = input;
                }
            } else {
                input.input_window = mem::replace(ticks, 0);
                input_buffer.push(input);
            }
        }
    }
}
