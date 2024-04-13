use piston_window::Key;
use std::mem;
use crate::{CommandNode, CK};

// Action refer to the attack commands and her values
#[derive(Copy, Clone, Debug)]
enum Action {
    LP = 1 << 4,
    MP = 1 << 5,
    HP = 1 << 6,
    LK = 1 << 7,
    MK = 1 << 8,
    HK = 1 << 9,
}

// Direction refer to the directional movement in the 8 axis and her values
#[derive(Copy, Clone, Debug)]
enum Direction {
    F = 5,
    U = 2,
    B = 6,
    D = 9,
    UB = 8,
    UF = 7,
    DF = 14,
    DB = 15,
}

// Player store values like if is pressing or not the keys
struct PlayerInput {
    lp: bool,
    mp: bool,
    hp: bool,
    lk: bool,
    mk: bool,
    hk: bool,
    f: bool,
    u: bool,
    b: bool,
    d: bool,
    ub: bool,
    uf: bool,
    df: bool,
    db: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct InputKey {
    cmd_key: CK,
    buff_time: u128,
}

// Commmand Input refer a command or a set of commands in a input sequence (buffered)
// can be directions or actions LP, MP, Backward, Forward...
#[derive(Clone, Debug)]
pub struct CommandInput {
    keys: Vec<InputKey>,
    input_window: u16,
    walked: bool,
}
pub struct InputManager {
    player_input: PlayerInput,
    input_buffer: Vec<CommandInput>
}

impl PlayerInput {
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
        }
    }

    fn set_state(&mut self, key: &Key, state: bool) -> bool {
        let result;
        match key {
            Key::A => {
                result = self.lp;
                self.lp = state
            }
            Key::S => {
                result = self.mp;
                self.mp = state
            }
            Key::D => {
                result = self.hp;
                self.hp = state
            }
            Key::Z => {
                result = self.lk;
                self.lk = state
            }
            Key::X => {
                result = self.mk;
                self.mk = state
            }
            Key::C => {
                result = self.hk;
                self.hk = state
            }
            Key::Up => {
                result = self.u;
                self.u = state
            }
            Key::Down => {
                result = self.d;
                self.d = state
            }
            Key::Left => {
                result = self.b;
                self.b = state
            }
            Key::Right => {
                result = self.f;
                self.f = state
            }
            _ => return false, // Otros casos no se consideran
        }
        result
    }

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
        }
    }

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
        result
    }
}

impl InputKey {
    fn new(key: CK) -> Self {
        Self {
            cmd_key: key,
            buff_time: 0,
        }
    }

    fn update(&mut self) {
        self.buff_time += 1;
    }

    pub fn get_cmd_key_ref(&self) -> &CK {
        &self.cmd_key
    }
}

impl CommandInput {
    fn new() -> Self {
        Self {
            keys: Vec::new(),
            input_window: 0,
            walked: false,
        }
    }

    pub fn get_input_window_ref(&self) -> &u16{
        &self.input_window
    }

    pub fn get_keys_ref(&self) -> &Vec<InputKey>{
        &self.keys
    }
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            player_input: PlayerInput::new(),
            input_buffer: Vec::new()
        }
    }

    
    pub fn update_hold_key(&mut self) {
        for inputs in &mut self.input_buffer {
            for key in &mut inputs.keys {
                if self.player_input.get_state(&key.cmd_key) {
                    key.update();
                }
            }
        }
    }

    pub fn walk_input_buffer(&mut self, tree: &CommandNode) {
        let input_buffer = &mut self.input_buffer;
        for pos in 1..input_buffer.len() {
            if let Some(input) = input_buffer.get_mut(pos - 1) {
                if !input.walked {
                    input.walked = true;
                    if let Some(name) = tree.search(&input_buffer, pos - 1) {
                        println!("{:?}", name);
                    }
                }
            }
        }
    }

    pub fn handle_key_input(
        &mut self,
        ticks: &mut u16,
        replace: bool,
    ) {
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

    pub fn set_player_input(&mut self, key: &Key, value: bool) -> bool{
        return self.player_input.set_state(key, value);
    }

    pub fn get_input_buffer_ref(&self) -> &Vec<CommandInput> {
        &self.input_buffer
    }
}