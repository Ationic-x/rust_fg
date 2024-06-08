use crate::{error::cmd_error::CmdError, player::input::manager::CommandInput, CK};

use regex::Regex;
use std::collections::HashSet;

/// Representa un comando, que es una secuencia de elementos que forman una ejecución de movimiento.
/// Contiene la lista de elementos de comando y el nombre del comando.
#[derive(Debug, Clone)]
struct Command {
    cmd_elements: Vec<CommandElement>,
    name: String,
}

/// Representa un elemento de comando, que es la unidad más pequeña de un comando, y contiene las teclas presionadas y el tiempo mantenido.
#[derive(Debug, Clone)]
struct CommandElement {
    elements: HashSet<CK>,
    sensitive: bool,
    time: u16,
}

/// Representa un nodo en un árbol de comandos.
/// Contiene información sobre los elementos de comando, nombre del comando, sensibilidad, ventana de entrada y sub-nodos.
#[derive(Debug, Clone)]
pub struct CommandNode {
    /// Nivel dentro del árbol
    level: usize,
    /// Elemento para seguir la secuencia de comandos
    cmd_elements: Option<Vec<CK>>,
    /// Nombre de la hoja
    name: Option<String>,
    /// Sensibilidad con la que se debe ejecutar el comando
    sensitive: bool,
    /// Ventana o franja para introducir un comando entre este y el anterior nodo
    input_window: Vec<u16>,
    /// Nodos subsiguientes a este
    sub_nodes: Vec<CommandNode>,
}

impl Command {
    /// Crea una nueva instancia de `Command` con valores por defecto.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `Command` con valores por defecto
    fn new() -> Self {
        Self {
            cmd_elements: Vec::new(),
            name: String::new(),
        }
    }
}

impl CommandElement {
    /// Crea una nueva instancia de `CommandElement` con los valores especificados.
    ///
    /// # Agumentos
    ///
    /// * `inputs` - Un HashSet de String que contiene los inputs para el elemento de comando.
    /// * `time` - Un valor u16 que representa el tiempo para el elemento de comando.
    /// * `sensitive` - Un booleano que indica si el elemento de comando es sensible.
    /// * `directions_and_actions` - Una referencia a una matriz de tuplas que contiene los mapeos de direcciones y acciones.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `CommandElement` iniciada con los valores pasados.
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
    /// Crea una nueva instancia de `CommandNode` con valores por defecto.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `CommandNode` con valores iniciales predeterminados.
    fn new() -> Self {
        Self {
            level: 0,
            cmd_elements: None,
            name: None,
            sensitive: false,
            input_window: Vec::new(),
            sub_nodes: Vec::new(),
        }
    }

    /// Inserta un comando en el árbol de comandos a partir de él mismo.
    /// Recorre recursivamente el árbol e inserta los elementos de comando en las posiciones apropiadas.
    ///
    /// # Argumentos
    ///
    /// * `command` - Una referencia al comando que se va a insertar.
    /// * `pos` - La posición donde debería insertarse el elemento de comando.
    fn insert(&mut self, command: &Command, pos: usize) {
        if let Some(cmd_element) = command.cmd_elements.get(pos) {
            if let Some(node) = self.sub_nodes.iter_mut().find(|node| {
                node.cmd_elements == Some(cmd_element.elements.clone().into_iter().collect())
            }) {
                node.input_window.push(cmd_element.time);
                node.sensitive = cmd_element.sensitive;
                node.level = pos;
                node.insert(command, pos + 1);
            } else {
                let mut new_command = CommandNode::new();
                new_command.cmd_elements = Some(cmd_element.elements.clone().into_iter().collect());
                new_command.input_window.push(cmd_element.time);
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

    /// Busca un comando que coincida con el búfer de entrada comenzando desde la posición especificada.
    /// Busca de forma recursiva en el árbol de comandos un comando que coincida y devuelve su nombre si lo encuentra.
    ///
    /// # Argumentos
    ///
    /// * `input_buffer` - Una referencia al búfer de entrada que contiene los inputs de comando.
    /// * `pos` - La posición en el búfer de entrada desde la cual comenzar la búsqueda.
    ///
    /// # Retorna
    ///
    /// Una referencia opcional al `nombre` del comando que coincide, de lo contrario `None`.
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

    /// Ordena el árbol de comandos dando prioridad a las ramas más cortas.
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

/// Lee un archivo de comandos y analiza su contenido para crear un vector de comandos.
///
/// Esta función lee un archivo de comandos especificado por el parámetro de nombre de archivo. Analiza el contenido del archivo
/// para extraer los comandos y sus detalles asociados, como nombres, secuencias de entrada e información de temporización.
/// El formato del archivo de comandos debe adherirse a las siguientes convenciones:
/// - El archivo debe contener secciones delimitadas por marcadores '#commands' y '#endcommands'.
/// - Cada comando debe especificarse en el siguiente formato:
///   - 'name=<nombre_del_comando>' para especificar el nombre del comando.
///   - 'command=<secuencia_de_entrada>' para especificar la secuencia de entrada para el comando.
///   - Opcionalmente, se puede usar 'time=<valores_de_tiempo>' para especificar los valores de temporización para el comando.
/// - La secuencia de entrada debe consistir en inputs separados por comas.
/// - Los inputs pueden incluir caracteres especiales como '$' para múltiples inputs, '>' para sensibilidad y '/' para retención.
/// - Los valores de temporización se pueden especificar como una lista separada por comas de enteros.
///
/// # Argumentos
///
/// * `filename` - Una referencia opcional a una cadena que contiene el nombre del archivo de comandos a leer.
///
/// # Retorna
///
/// Un `io::Result` que contiene un vector de instancias de `Command` si el archivo se lee y analiza correctamente,
/// de lo contrario, un `io::Error` indicando el problema encontrado.
fn read_command_file(lines: &Vec<&str>) -> Result<Vec<Command>, CmdError> {
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
                match parse_command(
                    parts[1].split(',').map(|e| e.trim()).collect(),
                    commands.len() - 1,
                    &mut commands,
                    &time,
                ) {
                 Ok(_) => {},
                 Err(_) => return Err(CmdError::Malformed(i)),
                };
            }
            _ => {}
        }
    }
    Ok(commands)
}

/// Analiza una secuencia de comandos y completa el comando correspondiente con los detalles analizados.
///
/// Esta función toma un vector de elementos de comando, una posición, un vector de comandos y la información de temporización como entrada.
/// Itera sobre cada elemento en la secuencia de comandos, lo analiza y completa el comando con los detalles analizados.
/// Los detalles analizados incluyen los inputs, el tiempo, la sensibilidad y cualquier caracter especial presente en la secuencia de comandos.
///
/// # Argumentos
///
/// * `elements` - Un vector de fragmentos de cadena que representan los elementos de comando.
/// * `pos` - La posición en el vector de comandos donde deben completarse los detalles del comando.
/// * `commands` - Una referencia mutable a un vector de instancias de `Command`.
/// * `time` - Una referencia a un vector que contiene la información de temporización para el comando.
fn parse_command(elements: Vec<&str>, pos: usize, commands: &mut Vec<Command>, time: &Vec<u16>) -> Result<(), ()> {
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
                                        )?;
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
                                )?;
                            }

                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            return Ok(());
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
            let size = inputs.len();

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
                if cmd_element.elements.len() < size {
                    return Err(());
                }
                command.cmd_elements.push(cmd_element);
            }
        }
    }
    Ok(())
}

/// Crea un árbol de comandos para un personaje dado leyendo el archivo de comandos correspondiente.
///
/// Esta función construye un árbol de comandos para un personaje especificado leyendo el archivo de comandos correspondiente.
/// Analiza el contenido del archivo de comandos para extraer los comandos y sus detalles asociados.
/// El archivo de comandos debe estar ubicado en el directorio 'src/assets' y tener el nombre '<character_name>.cmd'.
///
/// # Argumentos
///
/// * `char_name` - Una porción de cadena que contiene el nombre del personaje para el cual se creará el árbol de comandos.
///
/// # Retorna
///
/// Un `CommandNode` que representa la raíz del árbol de comandos con sus ramas para el personaje especificado.
pub fn create_command_tree(cmd: &str) -> Result<CommandNode, CmdError> {
    let content = match std::fs::read_to_string(cmd) {
        Ok(content) => content,
        Err(_) => return Err(CmdError::NotFound(cmd.to_string())),
    };
    let lines: Vec<&str> = content.lines().map(|line| line.trim()).collect();
    let mut tree = CommandNode::new();

    let commands = read_command_file(&lines)?;

    for command in &commands {
        tree.insert(command, 0);
    }
    tree.sort();
    return Ok(tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIRANDACT: [(&str, CK); 15] = [
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

    /// Prueba de la creación de un cmd_element
    #[test]
    fn test_command_element_creation() {
        let inputs: HashSet<String> = ["LP".to_string(), "MP".to_string()]
            .iter()
            .cloned()
            .collect();

        let cmd_element = CommandElement::new(inputs, 10, true, &DIRANDACT);

        assert!(cmd_element.elements.contains(&CK::LP));
        assert!(cmd_element.elements.contains(&CK::MP));
        assert!(cmd_element.sensitive);
        assert_eq!(cmd_element.time, 10);
    }

    /// Prueba para la isercción de un commando
    #[test]
    fn test_insert_command_node() {
        let mut root = CommandNode::new();
        let mut command = Command::new();
        command.name = "test_command".to_string();
        command.cmd_elements.push(CommandElement::new(
            ["LP".to_string()].iter().cloned().collect(),
            15,
            true,
            &DIRANDACT,
        ));

        root.insert(&command, 0);

        assert_eq!(root.sub_nodes.len(), 1);
        assert_eq!(root.sub_nodes[0].name, Some("test_command".to_string()));
    }

    // Prueba de lectura de archivo de comandos
    #[test]
    fn test_read_command_file() {
        let lines = vec![
            "[Command]",
            "name = \"Prueba\"",
            "command = F, MP+LP",
            "time = 15",
        ];
        let commands = read_command_file(&lines).unwrap();

        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name, "Prueba");
        assert_eq!(commands[0].cmd_elements.len(), 2);
        assert!(commands[0].cmd_elements[0].elements.contains(&CK::F));
        assert_eq!(commands[0].cmd_elements[0].sensitive, false);
        assert_eq!(commands[0].cmd_elements[0].time, 0);
        assert!(commands[0].cmd_elements[1].elements.contains(&CK::LP));
        assert!(commands[0].cmd_elements[1].elements.contains(&CK::MP));
        assert_eq!(commands[0].cmd_elements[1].time, 15);
        assert_eq!(commands[0].cmd_elements[1].sensitive, false);
    }

    // Prueba de creación de árbol
    #[test]
    fn test_create_command_tree() {
        let content = "
        [Command]
        name = \"Prueba\"
        command = F, MP+LP
        time = 15
    ";
        let lines: Vec<&str> = content.lines().map(|line| line.trim()).collect();
        let mut tree = CommandNode::new();

        let commands = read_command_file(&lines).unwrap();

        for command in &commands {
            tree.insert(command, 0);
        }
        tree.sort();

        assert_eq!(tree.sub_nodes.len(), 1);
        assert_eq!(tree.sub_nodes[0].cmd_elements, Some(vec![CK::F]));
        assert_eq!(tree.sub_nodes[0].input_window, [0]);
        assert_eq!(tree.sub_nodes[0].name,None);
        assert_eq!(tree.sub_nodes[0].sensitive,false);
        assert!(
            tree.sub_nodes[0].sub_nodes[0].cmd_elements == Some(vec![CK::MP, CK::LP]) ||
            tree.sub_nodes[0].sub_nodes[0].cmd_elements == Some(vec![CK::LP, CK::MP])
        );
        assert_eq!(tree.sub_nodes[0].sub_nodes[0].input_window, [15]);
        assert_eq!(tree.sub_nodes[0].sub_nodes[0].name, Some("Prueba".to_string()));
        assert_eq!(tree.sub_nodes[0].sub_nodes[0].sensitive, false);
    }
}
