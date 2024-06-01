use crate::{player::character::cmd, CK};

use cmd::manager::CommandNode;
use std::mem;

/// Represents different attack commands and their associated values.
///
/// This enum defines the various attack commands that can be executed in the game,
/// along with their corresponding values. These commands include LP (Light Punch),
/// MP (Medium Punch), HP (Heavy Punch), LK (Light Kick), MK (Medium Kick), and HK (Heavy Kick).
#[derive(Copy, Clone, Debug)]
enum Action {
    /// Ligth Punch Action
    LP = 1 << 4,
    /// Medium Punch Action
    MP = 1 << 5,
    /// Heavy Punch Action
    HP = 1 << 6,
    /// Ligth Kick Action
    LK = 1 << 7,
    /// Medium Kick Action
    MK = 1 << 8,
    /// Heavy Kick Action
    HK = 1 << 9,
    Start = 1 << 10,
}

/// Represents directional movement in 2D space.
///
/// This enum defines the possible directional movements in 2D space,
/// including forward, backward, up, down, and various diagonal combinations.
#[derive(Copy, Clone, Debug)]
enum Direction {
    /// Forward direction.
    F = 5,
    /// Up direction.
    U = 2,
    /// Backward direction.
    B = 6,
    /// Down direction.
    D = 9,
    /// Up-Backward diagonal direction.
    UB = 8,
    /// Up-Forward diagonal direction.
    UF = 7,
    /// Down-Forward diagonal direction.
    DF = 14,
    /// Down-Backward diagonal direction.
    DB = 15,
}

/// Stores the input state of a player.
///
/// This struct holds the current input state of a player, indicating whether
/// specific keys or directional inputs are being pressed.
pub struct PlayerInput {
    /// Light Punch key state
    pub lp: bool,
    /// Medium Punch key state
    pub mp: bool,
    /// Heavy Punch key state
    pub hp: bool,
    /// Light Kick key state
    pub lk: bool,
    /// Medium Kick key state
    pub mk: bool,
    /// Heavy Kick key state
    pub hk: bool,
    /// Forward key state
    pub f: bool,
    /// Up key state
    pub u: bool,
    /// Backward key state
    pub b: bool,
    /// Down key state
    pub d: bool,
    /// Up-Backward key state
    pub ub: bool,
    /// Up-Forward key state
    pub uf: bool,
    /// Down-Forward key state
    pub df: bool,
    /// Down-Backward key state
    pub db: bool,
    pub start: bool,
}

/// Represents an input key along with its buffer time.
///
/// This struct holds an input key along with the time it has been buffered.
/// It is used to track the duration for which a particular key has been held.
#[derive(Debug, Clone, Copy)]
pub struct InputKey {
    /// Command key associated with the input
    cmd_key: CK,
    /// Time for which the input has been buffered
    buff_time: u128,
}

/// Represents a sequence of input commands.
///
/// This struct represents a sequence of input commands that have been buffered,
/// forming a command input. It contains a list of input keys, an input window
/// indicating the maximum duration for which the input is valid, and a flag to
/// track whether the input has been processed.
#[derive(Clone, Debug)]
pub struct CommandInput {
    /// List of input keys in the sequence
    keys: Vec<InputKey>,
    /// Maximum duration for which the input is valid
    input_window: u16,
    // Flag indicating whether the input has been processed
    walked: bool,
    found: bool,
}

/// Manages player input and command buffering.
///
/// This struct handles player input and manages the buffering of input commands.
/// It maintains the current input state of the player and stores a buffer of
/// command inputs for processing.
pub struct InputManager {
    /// Player input state
    pub player_input: PlayerInput,
    /// Buffer for storing command inputs
    input_buffer: Vec<CommandInput>,
}

impl PlayerInput {
    /// Creates a new `PlayerInput` instance with all keys initially released.
    ///
    /// # Returns
    ///
    ///  A new `PlayerInput` instance with all key states set to `false`.
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

    fn clear_action(&mut self) {
        self.lp = false;
        self.mp = false;
        self.hp = false;
        self.lk = false;
        self.mk = false;
        self.hk = false;
        self.start = false;
    }
    /// Sets the state of a specified key.
    ///
    /// Sets the state of the given key to the specified boolean value.
    /// Returns the previous state of the key.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key to set the state for.
    /// * `state` - The boolean value indicating whether the key is pressed (`true`) or released (`false`).
    ///
    /// # Returns
    ///
    /// The previous state of the key before it was set.
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

    /// Retrieves the state of a specified command key.
    ///
    /// Returns the current state of the specified command key.
    ///
    /// # Arguments
    ///
    /// * `cmd_key` - A reference to the command key whose state is to be retrieved.
    ///
    /// # Returns
    ///
    /// The current state of the specified command key.
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

    /// Converts the player input to a bit representation.
    ///
    /// Converts the current state of the player input into a 16-bit representation
    /// where each bit corresponds to the state of a specific key or directional input.
    ///
    /// # Returns
    ///
    /// The current state of the player input in 16-bit
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
    /// Creates a new `InputKey` instance with the specified command key.
    ///
    /// Returns a new `InputKey` instance initialized with the provided command key
    /// and a buffer time of zero.
    ///
    /// # Arguments
    ///
    /// * `key` - The command key associated with the input.
    ///
    /// # Returns
    /// A new `InputKey` instance:
    /// * `cmd_key` initialized with `Key` value.
    /// * `buff_time` initialized to zero.
    fn new(key: CK) -> Self {
        Self {
            cmd_key: key,
            buff_time: 0,
        }
    }

    /// Updates the buffer time of the input key.
    ///
    /// Increments the buffer time of the input key by one unit.
    fn update(&mut self) {
        self.buff_time += 1;
    }

    /// Retrieves a reference to the command key associated with the input.
    ///
    /// # Returns
    ///
    /// A reference to the command key associated with the input.
    pub fn get_cmd_key_ref(&self) -> &CK {
        &self.cmd_key
    }
}

impl CommandInput {
    /// Creates a new `CommandInput` instance.
    ///
    /// # Returns
    /// A new `CommandInput` instance:
    /// * `keys` initialize an empty vector
    /// * `input_window` initialized to zero
    /// * `walked` flag initialized to `false`.
    fn new() -> Self {
        Self {
            keys: Vec::new(),
            input_window: 0,
            walked: false,
            found: false,
        }
    }

    /// Retrieves a reference to the input window duration.
    ///
    /// # Returns
    /// A reference to the input window duration, indicating the maximum
    /// duration for which the input is valid.
    pub fn get_input_window_ref(&self) -> &u16 {
        &self.input_window
    }

    /// Retrieves a reference to the list of input keys.
    ///
    /// # Returns
    /// A reference to the list of input `keys` stored in the `CommandInput` instance.
    pub fn get_keys_ref(&self) -> &Vec<InputKey> {
        &self.keys
    }

    pub fn set_found(&mut self, bool: bool) {
        self.found = bool;
    }
}

impl InputManager {
    /// Creates a new `InputManager` instance.
    ///
    /// # Returns
    /// A new `InputManager` instance:
    /// * `player_input` initialized to all keys released (`false`)
    /// * `input_buffer` initialize an empty vector.
    pub fn new() -> Self {
        Self {
            player_input: PlayerInput::new(),
            input_buffer: Vec::new(),
        }
    }

    /// Updates the hold duration of input keys in the input buffer.
    ///
    /// Iterates through each command input in the input buffer and increments
    /// the hold duration of any input keys that are currently held down.
    pub fn update_hold_key(&mut self) {
        for inputs in &mut self.input_buffer {
            for key in &mut inputs.keys {
                if self.player_input.get_state(&key.cmd_key) {
                    key.update();
                }
            }
        }
    }

    pub fn flip(&mut self){
        for inputs in &mut self.input_buffer {
            for key in &mut inputs.keys {
                if key.cmd_key == CK::B {
                    key.cmd_key = CK::F;
                } 
                if key.cmd_key == CK::F {
                    key.cmd_key = CK::B;
                } 
            }
        }
        let f = self.player_input.f;
        let b = self.player_input.b;
        self.player_input.b = f;
        self.player_input.f = b;
    }


    /// Traverses the input buffer and checks for valid input sequences.
    ///
    /// Walks through the input buffer, searching for valid input sequences
    /// based on the provided command node tree. If a valid sequence is found,
    /// the corresponding action is executed.
    ///
    /// # Arguments
    ///
    /// * `tree` - A reference to the command node tree used to search for valid input sequences.
    pub fn walk_input_buffer(&mut self, tree: &CommandNode) -> String {
        let input_buffer = &mut self.input_buffer;
        for pos in 0..input_buffer.len() {
            for input in &mut *input_buffer{
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

    pub fn clear(&mut self) {
        self.player_input.clear_action();
        self.input_buffer.clear();
    }

    /// Handles the input from the player and updates the input buffer accordingly.
    ///
    /// Processes the player's input, converts it into command inputs, and updates
    /// the input buffer with the new input. Optionally, replaces the last input in
    /// the buffer if the buffer exceeds its maximum size.
    ///
    /// # Arguments
    ///
    /// * `ticks` - A mutable reference to the tick counter representing the current game time.
    /// * `replace` - A boolean indicating whether to replace the last input in the buffer if necessary.
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
