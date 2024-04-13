use piston_window::*;
use sprite::*;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    mem,
    rc::Rc,
    time::Instant,
    u128,
};
use winit::window::WindowButtons;

const FPS: u64 = 60;
const PAUSE_DURATION: i32 = 3;

// CK refer to command keys avaible commands in a fight
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum CK {
    DB,
    D,
    DF,
    B,
    F,
    UB,
    U,
    UF,
    LP,
    MP,
    HP,
    LK,
    MK,
    HK,
}

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

// Player store values like if is pressing or not the keys
struct Player {
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

impl Player {
    fn new() -> Self {
        Player {
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

// InputKey refer to the key pressed and her lifetime hold
#[derive(Debug, Clone, Copy)]
struct InputKey {
    cmd_key: CK,
    buff_time: u128,
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
}

// Command Node refer a node of a tree of commands example [2,3,6,a+b] <- branch
#[derive(Debug)]
struct CommandNode {
    cmd_elements: Option<Vec<CK>>,
    name: Option<String>,
    sensitive: bool,
    input_window: HashSet<u16>,
    sub_nodes: Vec<CommandNode>,
}

impl CommandNode {
    fn new() -> Self {
        Self {
            cmd_elements: None,
            name: None,
            sensitive: false,
            input_window: HashSet::new(),
            sub_nodes: Vec::new(),
        }
    }

    fn insert(&mut self, command: &Command, pos: usize) {
        if let Some(cmd_element) = command.cmd_elements.get(pos) {
            if let Some(node) = self.sub_nodes.iter_mut().find(|node| {
                node.cmd_elements == Some(cmd_element.elements.clone().into_iter().collect())
            }) {
                node.input_window.insert(cmd_element.time);
                node.sensitive = cmd_element.sensitive;
                node.insert(command, pos + 1);
            } else {
                let mut new_command = CommandNode::new();
                new_command.cmd_elements = Some(cmd_element.elements.clone().into_iter().collect());
                new_command.input_window.insert(cmd_element.time);
                new_command.sensitive = cmd_element.sensitive;
                if command.cmd_elements.len() - 1 <= pos {
                    new_command.name = Some(command.name.clone());
                }
                self.sub_nodes.push(new_command);
                self.sub_nodes.last_mut().unwrap().insert(command, pos + 1);
            }
        }
    }

    fn search(&self, input_buffer: &Vec<CommandInput>, pos: usize) -> Option<&String> {
        let sub_nodes = &self.sub_nodes;
        if let Some(input) = input_buffer.get(pos) {
            for sub_node in sub_nodes.iter().filter(|sub_node| {
                sub_node
                    .input_window
                    .iter()
                    .all(|time| *time == 0 || &input.input_window <= time)
            }) {
                let matched = if sub_node.sensitive {
                    sub_node.cmd_elements.iter().all(|commands| {
                        input.keys.len() == commands.len()
                            && commands.iter().all(|command| {
                                input
                                    .keys
                                    .iter()
                                    .any(|input_key| input_key.cmd_key == *command)
                            })
                    })
                } else {
                    sub_node.cmd_elements.iter().all(|commands| {
                        commands.iter().all(|command| {
                            input
                                .keys
                                .iter()
                                .any(|input_key| input_key.cmd_key == *command)
                        })
                    })
                };
                if matched {
                    if let Some(name) = &sub_node.name {
                        return Some(name);
                    } else {
                        return sub_node.search(&input_buffer, pos + 1);
                    }
                }
            }
        }
        None
    }

    fn print(&self, level: usize) {
        if let Some(command) = &self.cmd_elements {
            for _ in 0..level {
                print!("  ");
            }
            if let Some(name) = &self.name {
                println!("{:?} - {}", command, name);
            } else {
                println!("{:?}", command);
            }
        }
        for sub_command in &self.sub_nodes {
            sub_command.print(level + 1);
        }
    }
}

// Command refer to the wanted execution to execute a movement
// Basically the movelist of a character
#[derive(Debug, Clone)]
struct Command {
    cmd_elements: Vec<CommandElement>,
    name: String,
}

impl Command {
    fn new() -> Self {
        Self {
            cmd_elements: Vec::new(),
            name: String::new(),
        }
    }
}

// Command element refer to the smallest element of a command
// The set o key pressed on one input
#[derive(Debug, Clone)]
struct CommandElement {
    elements: HashSet<CK>,
    sensitive: bool,
    time: u16,
}

impl CommandElement {
    fn new() -> Self {
        Self {
            elements: HashSet::new(),
            sensitive: false,
            time: 15,
        }
    }

    /*fn is_action(&self) -> bool {
        let actions = [CK::LP, CK::MP, CK::HP, CK::LK, CK::MK, CK::HK];

        self.elements
            .iter()
            .any(|element| actions.contains(element))
    }*/
}

// Commmand Input refer a command or a set of commands in a input sequence (buffered)
// can be directions or actions LP, MP, Backward, Forward...
#[derive(Clone, Debug)]
struct CommandInput {
    keys: Vec<InputKey>,
    input_window: u16,
    walked: bool,
}

impl CommandInput {
    fn new() -> Self {
        Self {
            keys: Vec::new(),
            input_window: 0,
            walked: false,
        }
    }
}

fn main() {
    // --------------------------------------------
    // - CREATE WINDOW
    // --------------------------------------------
    // Size window
    let window_size = [512; 2];
    // Making the window were to play
    let mut window: PistonWindow = WindowSettings::new("Square Game", window_size)
        .resizable(false)
        .build()
        .unwrap();

    // Shorter reference to window
    let conf_window: &winit::window::Window = &window.window.window;

    // Extra settings
    // Disable maximize option
    conf_window.set_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);

    // Creating a texture context of the PistonWindow
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    // --------------------------------------------
    // - CREATE SPRITE
    // --------------------------------------------
    // Getting folder of assets
    let assets = std::env::current_dir().unwrap().join("src").join("assets");

    // Creating a texture of the sprite inside assets and the window
    let texture = Rc::new(
        Texture::from_path(
            &mut texture_context,
            assets.join("HotaruFutaba_861.png"),
            Flip::Horizontal,
            &TextureSettings::new(),
        )
        .unwrap(),
    );
    // Getting the sprite from texture
    let sprite = Sprite::from_texture(texture);
    // Getting the position of the sprite
    let sprite_coord = sprite.get_position();

    let mut input_buffer: Vec<CommandInput> = Vec::new();
    let mut ticks = 0;
    let mut player: Player = Player::new();

    let mut tree = CommandNode::new();

    match read_command_file(assets.join("example").to_str()) {
        Ok(commands) => {
            for command in &commands {
                tree.insert(command, 0);
            }
        }
        Err(err) => {
            eprintln!("Error al leer el archivo: {}", err);
            std::process::exit(1);
        }
    }

    tree.print(0);

    let mut debug = false;

    window.events.set_max_fps(FPS);
    window.events.set_ups(FPS);
    let mut total_frames = -1;
    let mut action_timer = 0;
    let mut enable_action_timer = false;
    let mut last_print_time = Instant::now();

    // --------------------------------------------
    // - LOOP WINDOW
    // --------------------------------------------
    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            for inputs in &mut input_buffer {
                for key in &mut inputs.keys {
                    if player.get_state(&key.cmd_key) {
                        key.update();
                    }
                }
            }
            ticks += 1u16;
            if action_timer < PAUSE_DURATION {
                action_timer += 1;
            } else if enable_action_timer {
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
                enable_action_timer = false;
            }
            if debug {
                total_frames += 1;

                let elapsed_seconds = last_print_time.elapsed().as_secs();
                if elapsed_seconds > 0 {
                    let average_fps = (total_frames as f64) / (elapsed_seconds as f64);
                    println!("FPS: {:}", average_fps as u64);
                    total_frames = -1;
                    last_print_time = Instant::now();
                }
            }
        }

        // Read Key pressed
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up
                | Key::Down
                | Key::Left
                | Key::Right
                | Key::A
                | Key::S
                | Key::D
                | Key::Z
                | Key::X
                | Key::C => {
                    if !player.set_state(&key, true) {
                        handle_key_input(
                            player.to_bits(),
                            &mut input_buffer,
                            &mut ticks,
                            enable_action_timer,
                        );
                        if ![Key::Up, Key::Down, Key::Left, Key::Right].contains(&key)
                            && !enable_action_timer
                        {
                            action_timer = 0;
                            enable_action_timer = true;
                        }
                    }
                }
                Key::F1 => {
                    debug = !debug;
                    if debug {
                        println!("FPS: {:}", 0);
                        total_frames = -1;
                        last_print_time = Instant::now();
                    }
                }
                _ => {}
            }
        }

        // Read Key released
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::Up
                | Key::Down
                | Key::Left
                | Key::Right
                | Key::A
                | Key::S
                | Key::D
                | Key::Z
                | Key::X
                | Key::C => {
                    if player.set_state(&key, false) {
                        handle_key_input(
                            player.to_bits(),
                            &mut input_buffer,
                            &mut ticks,
                            enable_action_timer,
                        );
                    }
                }
                _ => {}
            }
        }

        // Update the window image, redraw all the sprites
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            sprite.draw(c.transform.trans(sprite_coord.0, sprite_coord.1), g);
        });
    }
}

fn read_command_file(filename: Option<&str>) -> io::Result<Vec<Command>> {
    if let Some(name) = filename {
        let file = File::open(format!("{}.cmd", name))?;
        let reader = io::BufReader::new(file);

        let mut reading_commands = false;
        let mut commands: Vec<Command> = Vec::new();
        let mut lines = reader.lines().peekable();
        while let Some(line) = lines.next() {
            let text = line?;
            let trimmed_text = text.trim();

            if trimmed_text == "#commands" {
                reading_commands = true;
                continue;
            } else if trimmed_text == "#endcommands" {
                break;
            }

            if reading_commands {
                match trimmed_text {
                    s if s.starts_with("name") => {
                        let parts: Vec<&str> = trimmed_text.split('=').collect();
                        commands.push(Command::new());
                        if let Some(command) = commands.last_mut() {
                            command.name = parts[1].trim().to_string();
                        }
                    }
                    s if s.starts_with("command") => {
                        let mut time: Vec<u16> = vec![15];
                        
                        if let Some(next_line_result) = lines.peek() {
                            if let Ok(next_line) = next_line_result {
                                if next_line.starts_with("time") {
                                    let parts: Vec<&str> = next_line.split('=').collect();
                                    let time_gotten: Vec<u16> = parts[1]
                                        .split(',')
                                        .map(|e| e.trim().parse().unwrap())
                                        .collect();
                                    time = time_gotten;
                                }
                            }
                        }
        
                        let parts: Vec<&str> = trimmed_text.split('=').collect();
                        parse_command(
                            parts[1].split(',').map(|e| e.trim()).collect(),
                            commands.len() - 1,
                            &mut commands,
                            &time,
                        );
                    }
                    _ => {}
                }
            }
        }
        Ok(commands)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing filename",
        ))
    }
}

fn parse_command(
    elements: Vec<&str>,
    pos: usize,
    commands: &mut Vec<Command>,
    time: &Vec<u16>,
) {
    let mut hold_element = String::new();
    let mut hold = false;

    let directions_and_actions = [
        ("LP", CK::LP),
        ("MP", CK::MP),
        ("HP", CK::HP),
        ("LK", CK::LK),
        ("MK", CK::MK),
        ("HK", CK::HK),
        ("U", CK::U),
        ("F", CK::F),
        ("D", CK::D),
        ("B", CK::B),
        ("UF", CK::UF),
        ("UB", CK::UB),
        ("DF", CK::DF),
        ("DB", CK::DB),
    ];

    for (index, element) in elements.iter().enumerate() {
        let mut sensitive = false;
        let mut inputs = HashSet::new();
        let mut modified_element = element.to_string();
        if modified_element.contains('$') {
            if let Some(dollar_pos) = modified_element.find('$') {
                if let Some(command) = commands.get(pos).cloned() {
                    commands.push(command.clone());
                    commands.push(command);
                }
                let length = commands.len();
                match &modified_element.as_str()[dollar_pos + 1..dollar_pos + 2] {
                    "F" | "U" | "D" | "B" | "P" | "K" => {
                        let base_key = &modified_element.as_str()[dollar_pos..dollar_pos + 2];
                        let variations =
                            match &modified_element.as_str()[dollar_pos + 1..dollar_pos + 2] {
                                "F" => ["F", "UF", "DF"],
                                "U" => ["U", "UF", "UB"],
                                "D" => ["D", "DF", "DB"],
                                "B" => ["B", "UB", "DB"],
                                "P" => ["LP", "MP", "HP"],
                                "K" => ["LK", "MK", "HK"],
                                _ => ["", "", ""],
                            };
                        let repeated = elements[index].matches(base_key).count();
                        match elements[index].matches(base_key).count() {
                            1 | 2 => {
                                for (i, variation) in variations.iter().enumerate() {
                                    if !elements[index].contains(variation)
                                        || base_key.contains(variation)
                                    {
                                        let mut modified_elements = elements.clone();
                                        let mut replaced_str = modified_elements[index]
                                            .replacen(base_key, variation, 1);
                                        if repeated == 2 {
                                            replaced_str = replaced_str.replacen(
                                                base_key,
                                                variations[(i + 1) % 3],
                                                1,
                                            )
                                        };
                                        modified_elements[index] = &replaced_str;
                                        let offset = if i == 0 { pos } else { length - i };
                                        parse_command(
                                            modified_elements.clone()[index..].to_vec(),
                                            offset,
                                            commands,
                                            &time,
                                        );
                                    }
                                }
                            }
                            3 => {
                                let mut modified_elements = elements.clone();
                                let mut replaced_str = modified_elements[index].to_owned();
                                for variation in variations {
                                    replaced_str = replaced_str.replacen(base_key, variation, 1);
                                }
                                modified_elements[index] = &replaced_str;
                                parse_command(
                                    modified_elements.clone()[index..].to_vec(),
                                    pos,
                                    commands,
                                    &time,
                                );
                            }

                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            return;
        }
        if modified_element.contains('>') {
            modified_element = modified_element.replace('>', "");
            sensitive = true;
        }
        if modified_element.contains('/') {
            modified_element = modified_element.replace('/', "");
            hold = true;
        }

        if modified_element.contains('+') {
            inputs = modified_element
                .split('+')
                .map(|e| e.trim().to_string())
                .collect();
        } else {
            inputs.insert(modified_element.clone());
        }

        if hold {
            if hold_element.len() > 0 {
                if index == elements.len() - 1 {
                    inputs.insert(hold_element);
                    hold_element = String::new();
                } else {
                    hold_element = String::new();
                    hold = false;
                }
            } else {
                hold_element = modified_element;
            }
        }

        if let Some(command) = commands.get_mut(pos) {
            let time_value;

            if command.cmd_elements.is_empty() {
                time_value = 0;
            } else {
                time_value = time[(command.cmd_elements.len() - 1) % time.len()]
            }
            println!("{:?}", time_value);
            if let Some(cmd_element) =
                create_cmd_element(inputs, time_value, sensitive, &directions_and_actions)
            {
                command.cmd_elements.push(cmd_element);
            }
        }
    }
}

fn create_cmd_element(
    inputs: HashSet<String>,
    time: u16,
    sensitive: bool,
    directions_and_actions: &[(&str, CK); 14],
) -> Option<CommandElement> {
    let mut cmd_element = CommandElement::new();
    for input in inputs {
        for &(k, v) in directions_and_actions {
            if k == input {
                cmd_element.elements.insert(v);
            }
        }
        cmd_element.time = time;
    }
    cmd_element.sensitive = sensitive;
    Some(cmd_element)
}

fn handle_key_input(
    player_input: u16,
    input_buffer: &mut Vec<CommandInput>,
    ticks: &mut u16,
    replace: bool,
) {
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
