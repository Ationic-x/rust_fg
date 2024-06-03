use crate::{error::cmd_error::CmdError, player::input::manager::CommandInput, CK};

use regex::Regex;
use std::collections::HashSet;

/// Represents a command, which is a sequence of command elements forming a movement execution.
/// This struct holds the list of command elements and the name of the command.
#[derive(Debug, Clone)]
struct Command {
    cmd_elements: Vec<CommandElement>,
    name: String,
}

/// Represents a command element, which is the smallest unit of a command, containing the keys pressed and timing information.
#[derive(Debug, Clone)]
struct CommandElement {
    elements: HashSet<CK>,
    sensitive: bool,
    time: u16,
}

/// Represents a node in a tree of commands.
/// This struct represents a branch in the command tree and contains information about the command elements,
/// name of the command, sensitivity, input window, and sub-nodes.
#[derive(Debug, Clone)]
pub struct CommandNode {
    level: usize,
    cmd_elements: Option<Vec<CK>>,
    name: Option<String>,
    sensitive: bool,
    input_window: HashSet<u16>,
    sub_nodes: Vec<CommandNode>,
}

impl Command {
    /// Creates a new instance of `Command` with an empty list of command elements and an empty name.
    ///
    /// # Returns
    ///
    /// A new `Command` instance:
    /// * `cmd_elements` initialized with an empty vector.
    /// * `name` initialized with an empty name.
    fn new() -> Self {
        Self {
            cmd_elements: Vec::new(),
            name: String::new(),
        }
    }
}

impl CommandElement {
    /// Creates a new instance of `CommandElement` with the given inputs, time, sensitivity, and directions and actions.
    /// It converts the inputs into command keys based on the provided directions and actions mapping.
    ///
    /// # Arguments
    ///
    /// * `inputs` - A HashSet of String containing the inputs for the command element.
    /// * `time` - A u16 value representing the timing for the command element.
    /// * `sensitive` - A boolean indicating whether the command element is sensitive.
    /// * `directions_and_actions` - A reference to an array of tuples containing the mappings of directions and actions.
    ///
    /// # Returns
    ///
    /// A new `CommandElement` instance:
    /// * `elements` initialized with the specified inputs.
    /// * `time` initialized wit the specified time.
    /// * `sensitive` intialized with the specified sensitivity.
    fn new(
        inputs: HashSet<String>,
        time: u16,
        sensitive: bool,
        directions_and_actions: &[(&str, CK); 15],
    ) -> Self {
        let mut elements = HashSet::new();
        for input in inputs {
            for &(k, v) in directions_and_actions {
                if k == input {
                    elements.insert(v);
                }
            }
        }
        Self {
            elements,
            sensitive,
            time,
        }
    }
}

impl CommandNode {
    /// Creates a new instance of `CommandNode` with default values.
    ///
    /// # Returns
    ///
    /// A new `CommandNode` instance:
    /// * `cmd_elements` initialized to `None`.
    /// * `name` initialized to `None`.
    /// * `sensitive` initialized to `false`.
    /// * `input_window` initialized with an empty `HashSet`.
    /// * `sub_node` initialized with an empty vector.
    fn new() -> Self {
        Self {
            level: 0,
            cmd_elements: None,
            name: None,
            sensitive: false,
            input_window: HashSet::new(),
            sub_nodes: Vec::new(),
        }
    }

    /// Inserts a command into the command tree starting from himself.
    /// It recursively traverses the tree and inserts the command elements at appropriate positions.
    ///
    /// # Arguments
    ///
    /// * `command` - A reference to the command to be inserted.
    /// * `pos` - The position where the command element should be inserted.
    fn insert(&mut self, command: &Command, pos: usize) {
        if let Some(cmd_element) = command.cmd_elements.get(pos) {
            if let Some(node) = self.sub_nodes.iter_mut().find(|node| {
                node.cmd_elements == Some(cmd_element.elements.clone().into_iter().collect())
            }) {
                node.input_window.insert(cmd_element.time);
                node.sensitive = cmd_element.sensitive;
                node.level = pos;
                node.insert(command, pos + 1);
            } else {
                let mut new_command = CommandNode::new();
                new_command.cmd_elements = Some(cmd_element.elements.clone().into_iter().collect());
                new_command.input_window.insert(cmd_element.time);
                new_command.sensitive = cmd_element.sensitive;
                new_command.level = pos;
                if command.cmd_elements.len() - 1 <= pos {
                    new_command.name = Some(command.name.clone());
                }
                self.sub_nodes.push(new_command);
                self.sub_nodes.last_mut().unwrap().insert(command, pos + 1);
            }
        }
    }

    /// Searches for a matching command based on the input buffer starting from the specified position.
    /// It recursively searches the command tree for a matching command and returns its name if found.
    ///
    /// # Arguments
    ///
    /// * `input_buffer` - A reference to the input buffer containing command inputs.
    /// * `pos` - The position in the input buffer to start searching from.
    ///
    /// # Returns
    ///
    /// An optional reference to the `name` of the matching command if found, otherwise `None`.
    pub fn search(&self, input_buffer: &Vec<CommandInput>, pos: usize) -> Option<&String> {
        let sub_nodes = &self.sub_nodes;
        let ib_len = input_buffer.len();
        if let Some(input) = input_buffer.get(pos) {
            for sub_node in sub_nodes.iter().filter(|sub_node| {
                sub_node
                    .input_window
                    .iter()
                    .all(|time| *time == 0 || input.get_input_window_ref() <= time)
                    && sub_node.level < ib_len
            }) {
                let matched = if sub_node.sensitive {
                    sub_node.cmd_elements.iter().all(|commands| {
                        input.get_keys_ref().len() == commands.len()
                            && commands.iter().all(|command| {
                                input
                                    .get_keys_ref()
                                    .iter()
                                    .any(|input_key| input_key.get_cmd_key_ref() == command)
                            })
                    })
                } else {
                    sub_node.cmd_elements.iter().all(|commands| {
                        commands.iter().all(|command| {
                            input
                                .get_keys_ref()
                                .iter()
                                .any(|input_key| input_key.get_cmd_key_ref() == command)
                        })
                    })
                };
                if matched {
                    if let Some(name) = &sub_node.name {
                        return Some(name);
                    } else {
                        if let Some(result) = sub_node.search(input_buffer, pos + 1) {
                            return Some(result);
                        } else {
                            return sub_node.search(input_buffer, pos + 2);
                        }
                    }
                }
            }
        }
        None
    }

    /// Prints the command tree starting from himself with proper indentation.
    ///
    /// # Arguments
    ///
    /// * `level` - The current level of indentation and depth in the command tree.
    // pub fn print(&self, level: usize) {
    //     if let Some(command) = &self.cmd_elements {
    //         for _ in 1..level {
    //             print!("  ");
    //         }
    //         if let Some(name) = &self.name {
    //             println!("{:?} - {}", command, name);
    //         } else {
    //             println!("{:?}", command);
    //         }
    //     }
    //     for sub_command in &self.sub_nodes {
    //         sub_command.print(level + 1);
    //     }
    // }

    fn sort(&mut self) {
        let priorities = vec![CK::LK, CK::MK, CK::HK, CK::LP, CK::HP, CK::MP];

        self.sub_nodes.sort_by(|a, b| {
            let a_priority = a
                .cmd_elements
                .as_ref()
                .and_then(|elems| elems.iter().position(|x| priorities.contains(x)))
                .unwrap_or(usize::MAX);

            let b_priority = b
                .cmd_elements
                .as_ref()
                .and_then(|elems| elems.iter().position(|x| priorities.contains(x)))
                .unwrap_or(usize::MAX);

            if let (Some(a_elems), Some(b_elems)) =
                (a.cmd_elements.as_ref(), b.cmd_elements.as_ref())
            {
                if a_elems.len() > b_elems.len() {
                    return std::cmp::Ordering::Less;
                } else if b_elems.len() > a_elems.len() {
                    return std::cmp::Ordering::Greater;
                }
            }

            a_priority.cmp(&b_priority)
        });

        for sub_node in &mut self.sub_nodes {
            sub_node.sort();
        }
    }
}

/// Reads a command file and parses its contents to create a vector of commands.
///
/// This function reads a command file specified by the filename parameter. It parses the file contents
/// to extract the commands and their associated details, such as names, input sequences, and timing information.
/// The command file format should adhere to the following conventions:
/// - The file should contain sections delineated by '#commands' and '#endcommands' markers.
/// - Each command should be specified in the following format:
///   - 'name=<command_name>' to specify the command name.
///   - 'command=<input_sequence>' to specify the input sequence for the command.
///   - Optionally, 'time=<timing_values>' can be used to specify the timing values for the command.
/// - The input sequence should consist of comma-separated inputs.
/// - Inputs can include special characters such as '$' for multiple inputs, '>' for sensitivity, and '/' for holding.
/// - Timing values can be specified as a comma-separated list of integers.
///
/// # Arguments
///
/// * `filename` - An optional reference to a string containing the name of the command file to read.
///
/// # Returns
///
/// An `io::Result` containing a vector of `Command` instances if the file is successfully read and parsed,
/// otherwise an `io::Error` indicating the encountered issue.
fn read_command_file(lines: &Vec<&str>) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        i += 1;

        match line {
            s if s.starts_with("name") => {
                let parts: Vec<&str> = line.split('=').collect();
                commands.push(Command::new());
                if let Some(command) = commands.last_mut() {
                    command.name = parts[1].trim().to_string();
                    let re = Regex::new(r#""([^"]*)""#).unwrap();
                    if let Some(captures) = re.captures(parts[1].trim()) {
                        if let Some(matched) = captures.get(1) {
                            command.name = matched.as_str().to_string();
                        }
                    }
                }
            }
            s if s.starts_with("command") => {
                let mut time: Vec<u16> = vec![15];

                if lines[i].starts_with("time") {
                    let parts: Vec<&str> = lines[i].split('=').collect();
                    let time_gotten: Vec<u16> = parts[1]
                        .split(',')
                        .map(|e| e.trim().parse().unwrap())
                        .collect();
                    time = time_gotten;
                }

                let parts: Vec<&str> = line.split('=').collect();
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
    commands
}

/// Parses a command sequence and populates the corresponding command with the parsed details.
///
/// This function takes a vector of command elements, position, commands vector, and timing information as input.
/// It iterates over each element in the command sequence, parses it, and populates the command with the parsed details.
/// The parsed details include the inputs, timing, sensitivity, and any special characters present in the command sequence.
///
/// # Arguments
///
/// * `elements` - A vector of string slices representing the command elements.
/// * `pos` - The position in the commands vector where the command details should be populated.
/// * `commands` - A mutable reference to a vector of Command instances.
/// * `time` - A reference to a vector containing the timing information for the command.
fn parse_command(elements: Vec<&str>, pos: usize, commands: &mut Vec<Command>, time: &Vec<u16>) {
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
        ("Start", CK::Start),
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
            if let Some(cmd_element) = Some(CommandElement::new(
                inputs,
                time_value,
                sensitive,
                &directions_and_actions,
            )) {
                command.cmd_elements.push(cmd_element);
            }
        }
    }
}

/// Creates a command tree for a given character name by reading the corresponding command file.
///
/// This function constructs a command tree for a specified character by reading the corresponding command file.
/// It parses the command file contents to extract the commands and their associated details.
/// The command file should be located in the 'src/assets' directory and named as '<character_name>.cmd'.
///
/// # Arguments
///
/// * `char_name` - A string slice containing the name of the character for which the command tree is to be created.
///
/// # Returns
///
/// A `CommandNode` representing the root of the command tree with her branches for the specified character.
pub fn create_command_tree(cmd: &str) -> Result<CommandNode, CmdError> {
    let content = match std::fs::read_to_string(cmd) {
        Ok(content) => content,
        Err(_) => return Err(CmdError::NotFound(cmd.to_string())),
    };
    let lines: Vec<&str> = content.lines().map(|line| line.trim()).collect();
    let mut tree = CommandNode::new();

    let commands = read_command_file(&lines);

    for command in &commands {
        tree.insert(command, 0);
    }
    tree.sort();
    return Ok(tree);
}
