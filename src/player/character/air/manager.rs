use std::{
    collections::HashMap,
    fmt::Error,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{
    chars::Character,
    error::{air_error::AirError, pop_up::show_error_popup},
    player::character::sff::decoder::Sff,
};
use gfx_device_gl::Resources;
use piston_window::{G2dTextureContext, Texture};
use sprite::Sprite;

/// Struct que representa una caja de colisiones (incluye "HitBox" y "HurtBox").
///
/// Esta estructura se utiliza para definir áreas de colisión en un juego, que pueden ser
/// utilizadas para detectar colisiones entre objetos del juego.
#[derive(Debug, PartialEq)]
pub struct Clsn {
    /// Es una hitbos o una Hurtbox
    hitbox: bool,
    /// Desplazamiento en la X respecto su original
    ofs_x: f64,
    /// Desplazamiento a la derecha respecto su original
    ofs_right: f64,
    /// Desplazamiento en la Y respecto su original
    ofs_y: f64,
    /// Posición en la X
    x: f64,
    /// Posición en la Y
    y: f64,
    /// Ancho de la colisión
    width: f64,
    /// Alto de la colisión
    height: f64,
}

/// Struct que representa una tabla de animaciones.
///
/// Esta estructura contiene las animaciones disponibles para un personaje en un juego.
/// Permite cargar y manipular animaciones, así como también trabajar con las colisiones asociadas a cada una.
pub struct AnimationTable {
    /// Mapa de animación identificaod por su ID
    animations: HashMap<i32, Animation>,
    /// Referencia mutable del manejador del archivo SFF
    sff: Option<Arc<Mutex<Sff>>>,
    /// Referencia del sprite del jugador
    spr: Option<Arc<Mutex<Sprite<Texture<Resources>>>>>,
    /// Vector con las colisiones activas del jugador
    clsns: Vec<Clsn>,
}

/// Struct que representa un fotograma de una animación, incluyendo las colisiones.
#[derive(Clone, Debug)]
pub struct AnimFrame {
    /// Tiempo que dura el frame
    time: i32,
    /// Grupo al que pertenece el frame
    group: i16,
    /// Número dentro del grupo al que pertence el frame
    number: i16,
    /// Posción en la X
    x: i16,
    /// Posición en la Y
    y: i16,
    /// Color alpha del sprite original
    src_alpha: u8,
    /// Color alpha del sprite al que transiciona
    dst_alpha: u8,
    /// Rotación horizontal del sprite
    h: i8,
    /// Rotación vertical del sprite
    v: i8,
    /// Colisiones del frame
    ex: Vec<Vec<f64>>,
    /// Hurtboxes por defecto en el frame
    def1: bool,
    /// Hitboxes por defecto en el frame
    def2: bool,
}

/// Struct que representa una animación.
#[derive(Clone)]
pub struct Animation {
    /// Referencia mutable del gestor del archivo SFF
    sff: Option<Arc<Mutex<Sff>>>,
    /// Refenrecia mutable del sprite del jugador
    spr: Option<Arc<Mutex<Sprite<Texture<Resources>>>>>,
    /// Vector de los frames del jugador
    frames: Vec<AnimFrame>,
    /// Indicador del frame donde inicia un bucle
    loopstart: i32,
    /// Tranlación del sprite en el X
    interpolate_offset: Vec<i32>,
    /// Cambio de la escala en el sprite
    interpolate_scale: Vec<i32>,
    /// Cambio del angulo en el sprite
    interpolate_angle: Vec<i32>,
    /// Cambio del dibujado en el sprite
    interpolate_blend: Vec<i32>,
    /// Frame actualmente que se anda dibujando
    current: i32,
    /// ID del frame dibujado
    drawidx: i32,
    /// Tiempo que se lleva dibujando el frame
    time: i32,
    /// Suma total que se llava en la animación
    sumtime: i32,
    /// Tiempo total que se debe llegar
    totaltime: i32,
    /// Tiempo en el bucle
    looptime: i32,
    /// Tiempo para eventos o pausas
    nazotime: i32,
    /// Máscara que cambia la paleta
    mask: i16,
    /// Fin del bucle
    loopend: bool,
}

impl AnimFrame {
    /// Crea un nuevo fotograma de animación.
    ///
    /// Retorna un nuevo `AnimFrame` con valores predeterminados.
    fn new() -> Self {
        Self {
            time: -1,
            group: -1,
            number: 0,
            x: 0,
            y: 0,
            src_alpha: 255,
            dst_alpha: 0,
            h: 1,
            v: 1,
            ex: Vec::new(),
            def1: false,
            def2: false,
        }
    }
}

impl Clsn {
    /// Crea una nueva colisión.
    ///
    /// # Argumentos
    ///
    /// * `left` - La coordenada izquierda de la caja de colisión.
    /// * `top` - La coordenada superior de la caja de colisión.
    /// * `right` - La coordenada derecha de la caja de colisión.
    /// * `bottom` - La coordenada inferior de la caja de colisión.
    /// * `hitbox` - Indica si esta caja es una HitBox (true) o una HurtBox (false).
    pub fn new(left: f64, top: f64, right: f64, bottom: f64, hitbox: bool) -> Self {
        let width = right - left;
        let height = bottom - top;
        Self {
            hitbox,
            ofs_x: left,
            ofs_right: right,
            ofs_y: top,
            x: left,
            y: top,
            width,
            height,
        }
    }

    /// Obtiene los parámetros necesarios para dibujar la caja.
    ///
    /// Retorna un arreglo que contiene la posición (x, y) y las dimensiones (ancho, alto) de la caja.
    pub fn get_rectangle(&self) -> [f64; 4] {
        [self.x, self.y, self.width, self.height]
    }

    /// Asigna un valor a la `x` e `y` de la colisión en base a la `x` e `y` del personaje y si está girado.
    ///
    /// # Argumentos
    ///
    /// * `trans_x` - La coordenada x de traslación del personaje.
    /// * `trans_y` - La coordenada y de traslación del personaje.
    /// * `flip` - Indica si el personaje está girado horizontalmente (true) o no (false).
    pub fn set_position(&mut self, trans_x: f64, trans_y: f64, flip: bool) {
        if flip {
            self.x = trans_x - (self.ofs_x + self.width);
        } else {
            self.x = trans_x + self.ofs_x;
        }
        self.y = trans_y + self.ofs_y;
    }

    /// Devuelve si es una "HitBox" o una "HurtBox".
    ///
    /// Retorna true si la caja de colisión es una HitBox, o false si es una HurtBox.
    pub fn is_hitbox(&self) -> bool {
        self.hitbox
    }

    /// Devuelve si la caja de colisiones entra en contacto con otra.
    ///
    /// # Argumentos
    ///
    /// * `clsn_p2` - Otra caja de colisión con la que se va a comprobar la colisión.
    pub fn collides(&self, clsn_p2: &Clsn) -> bool {
        !(self.x + self.width < clsn_p2.x
            || self.x > clsn_p2.x + clsn_p2.width
            || self.y + self.height < clsn_p2.y
            || self.y > clsn_p2.y + clsn_p2.height)
    }
}

impl Animation {
    /// Crear una nueva animación.
    ///
    /// Retorna una nueva `Animation` con valores predeterminados.
    fn new() -> Self {
        Self {
            sff: None,
            spr: None,
            frames: Vec::new(),
            loopstart: 0,
            interpolate_offset: Vec::new(),
            interpolate_scale: Vec::new(),
            interpolate_angle: Vec::new(),
            interpolate_blend: Vec::new(),
            current: 0,
            drawidx: 0,
            time: 0,
            sumtime: 0,
            totaltime: 0,
            looptime: 0,
            nazotime: 0,
            mask: -1,
            loopend: false,
        }
    }

    /// Se le asigna una referencia al archivo SFF.
    ///
    /// # Argumentos
    ///
    /// * `sff` - Una referencia al SFF cargado.
    fn set_sff(&mut self, sff: Arc<Mutex<Sff>>) {
        self.sff = Some(sff);
    }

    /// Se reinician varios parámetros de la animación por defecto.
    pub fn reset(&mut self) {
        self.current = 0;
        self.drawidx = 0;
        self.time = -1;
        self.sumtime = 0;
        self.loopend = false;
    }

    /// Obtiene la diferencia entre el tiempo que dura la animación y el tiempo que lleva la animación.
    pub fn delta_time(&self) -> i32 {
        self.totaltime - self.sumtime
    }

    /// Avanza un paso dentro de la animación, actualizando el tiempo o el frame por el que va.
    ///
    /// # Argumentos
    ///
    /// * `char` - El personaje que está utilizando esta animación.
    ///
    /// Los pasos que sigue:
    /// - Si es una animación en bucle, los valores nunca se reinician.
    /// - Cada frame tiene definido un tiempo de duración, se asigna este valor la primera vez.
    /// - Se va sumando el tiempo que llevamos en la animación y el frame.
    /// - Cuando se finaliza un frame, se pasa al siguiente. Si se finaliza el tiempo de animación, reiniciamos.
    /// - Mientras, el sprite se actualiza con la imagen del frame actual.
    pub fn step(&mut self, char: &mut Box<dyn Character>) {
        if self.totaltime > 1 && self.delta_time() == 0 && self.loopstart == 0 {
            self.reset();
            char.set_time(-1);
        }
        let mut frame = &self.frames[self.current as usize];
        if self.time == -1 {
            self.time = frame.time;
        }
        if self.time == 0 && frame.time != -1 {
            self.current = (self.current + 1) % self.frames.len() as i32;
            if self.loopstart > 0 && self.current == 0 {
                self.current = self.loopstart;
                self.time = self.looptime;
            }
            frame = &self.frames[self.current as usize];
            self.time = frame.time;
        }
        self.sumtime += 1;
        self.time -= 1;

        let key = [frame.group, frame.number];

        if let Some(spr) = self.sff.as_ref().unwrap().lock().unwrap().sprites.get(&key) {
            let texture = spr.tex.clone().unwrap();
            let offset = spr.offset;
            char.set_width(spr.size[0]);
            char.set_offset_x(offset[0] as f64);
            char.set_offset_y(offset[1] as f64);
            let mut sprite = self.spr.as_ref().unwrap().lock().unwrap();
            sprite.set_texture(texture);
            if frame.h == -1 || char.get_flip_x() {
                char.is_flipped();
                sprite.set_flip_x(!char.is_flipped());
            } else {
                sprite.set_flip_x(char.is_flipped());
            }
            if frame.v == -1 {
                sprite.set_flip_y(true);
            } else {
                sprite.set_flip_y(false);
            }
        }
    }

    /// Actualiza un vector de colisiones en base a las colisiones del frame actual.
    ///
    /// # Argumentos
    ///
    /// * `clsns` - Un vector mutable de colisiones que se actualizará.
    ///
    /// A la hora de actualizar se intenta evitar redundancias al eliminar o agregar colisiones al vector.
    pub fn update_clsns(&self, clsns: &mut Vec<Clsn>) {
        let frame = &self.frames[self.current as usize];
        let mut i = 0;

        fn process_clsns(
            clsns: &mut Vec<Clsn>,
            data: &[f64],
            hitbox: bool,
            start_index: usize,
        ) -> usize {
            let mut i = start_index;
            for clsn in data.chunks(4) {
                let new_clsn = Clsn::new(clsn[0], clsn[1], clsn[2], clsn[3], hitbox);
                if i < clsns.len() && clsns[i] != new_clsn {
                    clsns[i] = new_clsn;
                } else {
                    clsns.push(new_clsn);
                }
                i += 1;
            }
            i
        }

        if !frame.def2 {
            if let Some(clsns2) = frame.ex.get(1) {
                i = process_clsns(clsns, clsns2, true, i);
            }
        } else {
            if let Some(clsns2) = frame.ex.get(1) {
                let clsns2_len = clsns2.len() / 4;
                i += clsns2_len;
            }
        }

        if !frame.def1 {
            if let Some(clsns1) = frame.ex.get(0) {
                i = process_clsns(clsns, clsns1, false, i);
            }
        } else {
            if let Some(clsns1) = frame.ex.get(0) {
                let clsns1_len = clsns1.len() / 4;
                i += clsns1_len;
            }
        }

        let mut size = clsns.len();
        while i < size {
            clsns.pop();
            size = clsns.len();
        }
    }

    /// Asigna una referencia al sprite del personaje.
    ///
    /// # Argumentos
    ///
    /// * `spr` - Una referencia al sprite del personaje.
    pub fn set_sprite(&mut self, spr: Option<Arc<Mutex<Sprite<Texture<Resources>>>>>) {
        self.spr = spr;
    }
}

impl AnimationTable {
    /// Crea una nueva tabla de animación.
    ///
    /// Retorna una nueva instancia de `AnimationTable` con valores iniciales.
    fn new() -> Self {
        Self {
            animations: HashMap::new(),
            sff: None,
            spr: None,
            clsns: Vec::new(),
        }
    }

    /// Le asigna una paleta a la animación utilizando un índice válido.
    ///
    /// # Argumentos
    ///
    /// * `palette` - El índice de la paleta que se desea asignar a la animación.
    pub fn set_palette(&mut self, palette: usize) {
        self.sff
            .as_mut()
            .unwrap()
            .lock()
            .unwrap()
            .set_palette(palette);
    }

    /// Obtiene el sprite para trabajar con él, desbloqueando su Mutex.
    ///
    /// Retorna una referencia mutable al sprite.
    pub fn get_sprite(&self) -> MutexGuard<Sprite<Texture<Resources>>> {
        self.spr.as_ref().unwrap().lock().unwrap()
    }

    /// Lee las líneas de un archivo en formato AIR y devuelve una animación.
    ///
    /// # Argumentos
    ///
    /// * `lines` - Vector de referencias de texto que representan las líneas del archivo AIR.
    /// * `i` - Índice de la línea actual que se está leyendo.
    ///
    /// # Retorna
    ///
    /// Una opción que contiene la animación leída, o None si no se encontró ninguna animación.
    fn read_action(
        &mut self,
        lines: &Vec<&str>,
        i: &mut usize,
    ) -> Result<Option<Animation>, AirError> {
        while *i < lines.len() {
            let mut a;
            if let Some(a1) = read_action(&lines, i)? {
                a = a1.1.clone();
                if let Some(tmp) = self.animations.get(&a1.0) {
                    return Ok(Some(tmp.clone()));
                }
                self.animations.insert(a1.0, a1.1.clone());
                while a1.1.frames.is_empty() && *i < lines.len() {
                    if let Ok(Some(a2)) = self.read_action(lines, i) {
                        a = a2;
                        break;
                    }
                    *i += 1;
                }
                return Ok(Some(a));
            }
            *i += 1;
        }
        Ok(None)
    }

    /// Le asigna un sprite a la tabla de animación y envía su referencia a sus animaciones.
    ///
    /// # Argumentos
    ///
    /// * `spr` - El sprite que se desea asignar a la tabla de animación.
    pub fn set_sprite(&mut self, spr: sprite::Sprite<Texture<Resources>>) {
        let sprite = Arc::new(Mutex::new(spr));

        self.spr = Some(sprite);
        self.set_animation_sprite();
    }

    /// Asigna un sprite a todas las animaciones que contiene la tabla de animación.
    fn set_animation_sprite(&mut self) {
        for animation in self.animations.values_mut() {
            animation.set_sprite(self.spr.clone());
        }
    }

    /// Se crea en base a los parámetros un SFF y se le asigna. A su vez, este SFF se le asigna a todas sus animaciones.
    ///
    /// # Argumentos
    ///
    /// * `char_name` - Nombre del personaje.
    /// * `filename` - Nombre del archivo que contiene el SFF.
    /// * `char` - Indica si se trata de un personaje (true) o no (false).
    /// * `context` - Contexto de textura G2d.
    pub fn set_sff(
        &mut self,
        char_name: &str,
        filename: String,
        char: bool,
        context: G2dTextureContext,
    ) {
        match Sff::load_sff(char_name, filename, char, context) {
            Ok(sff) => self.sff = Some(Arc::new(Mutex::new(sff))),
            Err(err) => {
                show_error_popup(&err);
                std::process::exit(1);
            }
        }
        self.set_animation_sff();
    }

    /// Asigna un SFF a todos las animaciones de una table de animación
    fn set_animation_sff(&mut self) {
        if self.sff.is_none() {
            println!("Set the sff");
            return;
        }
        let sff = self.sff.as_ref().unwrap();
        for animation in self.animations.values_mut() {
            animation.set_sff(Arc::clone(sff));
        }
    }

    /// Método el cual actualiza una animación especificada por un personaje,
    /// y a su vez se actualiza el personaje en base a la misma.
    ///
    /// # Argumentos
    ///
    /// * `char` - Referencia mutable al personaje que se actualizará.
    pub fn update_sprite(&mut self, char: &mut Box<dyn Character>) {
        let animation = self.animations.get_mut(char.get_anim()).unwrap();
        let flip = char.is_flipped();
        if char.get_new_anim() {
            animation.reset();
            char.set_new_anim(false);
            char.set_hit(-1);
            char.set_time(-1);
            let mut spr = self.spr.as_mut().unwrap().lock().unwrap();
            spr.set_flip_x(flip);
            spr.set_anchor(if flip { 1.0 } else { 0.0 }, 0.0);
        } else {
            let time = char.get_time();
            if time != animation.time {
                animation.time = time;
            }
        }
        animation.step(char);

        animation.update_clsns(&mut self.clsns);

        for clsn in &mut self.clsns {
            clsn.set_position(char.get_x(), char.get_y(), flip);
        }

        char.set_anim_element(animation.current);
        char.set_anim_time(animation.delta_time());
        char.set_time(animation.time);
    }

    /// Obtiene una referencia a la lista de cajas de colisión.
    ///
    /// Retorna una referencia a la lista de cajas de colisión.
    pub fn get_clsns(&self) -> &Vec<Clsn> {
        &self.clsns
    }
}

/// Lee línea por línea de un archivo con formato AIR y devuelve la animación y su número,
/// si es que se cumplen las condiciones del formato.
///
/// # Argumentos
///
/// * `lines` - Un vector de referencias a cadenas que representan las líneas del archivo AIR.
/// * `i` - Un contador de línea mutable que se utiliza para rastrear la posición actual en el vector `lines`.
///
/// # Retorna
///
/// Una tupla que contiene el número de la animación y la propia animación, envueltos en `Some`, si se cumplen las condiciones del formato AIR.
/// Si no se encuentra una animación válida, devuelve `None`. En caso de error en el formato del archivo, devuelve un `Result` que encapsula un `AirError`.
///
/// # Ejemplo de formato
///
/// El formato esperado para la línea que indica el comienzo de una animación debe ser similar a `[Begin Action 5]`.
/// Esto implica que la cadena debe comenzar con `[Begin`, seguido de un espacio y la palabra `Action`, seguido de otro espacio y el número de la animación, y finalmente `]`.
fn read_action(lines: &Vec<&str>, i: &mut usize) -> Result<Option<(i32, Animation)>, AirError> {
    let length = lines.len();

    let mut name = String::new();
    let mut subname = String::new();

    while *i < length {
        let sec = &lines[*i];
        if sec.is_empty() || !sec.starts_with('[') {
            name = "".to_string();
            subname = "".to_string();
        } else {
            let sec = sec.splitn(2, ';').next().unwrap().trim();
            if !sec.ends_with(']') {
                name = "".to_string();
                subname = "".to_string();
            } else {
                let sec = &sec[1..sec.rfind(']').unwrap()];
                if let Some(j) = sec.find(' ') {
                    name = sec[..=j].to_string();
                    subname = sec[j + 1..].to_string();
                } else {
                    name = sec.to_string();
                    subname = "".to_string();
                }
            }
        }
        if !name.is_empty() {
            break;
        }
        *i += 1;
    }

    name = name.to_lowercase();
    subname = subname.to_lowercase();

    if name.to_lowercase() != "begin " {
        return Ok(None);
    }

    let spi;
    match subname.find(" ") {
        Some(pos) => {
            spi = pos;
        }
        None => return Ok(None),
    }

    if &subname[..spi] != "action" {
        return Ok(None);
    }

    *i += 1;

    match read_animation(lines, i) {
        Ok(animation) => {
            return Ok(Some((
                atoi(&subname[spi + 1..].to_string()).unwrap(),
                animation,
            )))
        }
        Err(err) => return Err(err),
    }
}

/// Lee una línea de un archivo con formato tipo AIR, intentando interpretarla como un fotograma de animación.
///
/// # Argumentos
///
/// * `line` - Una referencia a una cadena que representa una línea del archivo con formato AIR.
///
/// # Retorna
///
/// Un `Option<AnimFrame>` que contiene los datos del fotograma de animación si la línea se puede interpretar como tal.
/// Si la línea no cumple con el formato esperado para un fotograma de animación, retorna `None`.
///
/// # Formato Esperado
///
/// Un fotograma de animación tiene el siguiente formato:
/// - Los primeros cinco valores son obligatorios: `group`, `number`, `x`, `y` y `time`.
/// - Luego, puede haber un sexto valor opcional para indicar espejado o volteado horizontal y/o verticalmente.
/// - Después, puede haber un séptimo valor opcional para indicar transparencia.
/// - Finalmente, puede haber hasta tres valores adicionales opcionales para datos extras.
///
/// Ejemplo de formato:
/// ```text
/// Clsn2: 2
/// Clsn2[0] = 12,-11,-11,-94
/// Clsn2[1] = -6,-109, 6,-9
/// 41,0, 0,0, 7
/// Loopstart
/// Clsn2Default: 2
/// Clsn2[0] = 14,-23,-11,-90
/// Clsn2[1] = -4,-105, 9,-89
/// 41,7, 0,0, 4
/// 41,8, 0,0, 4
fn read_anim_frame(line: &String) -> Option<AnimFrame> {
    if line.is_empty() || !line.starts_with(|c: char| c.is_digit(10) || c == '-') {
        return None;
    }

    let mut ary = line
        .splitn(10, ',')
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    if ary.len() < 5 {
        return None;
    }

    let mut af = AnimFrame::new();
    af.group = ary[0].parse().unwrap_or_default();
    af.number = ary[1].parse().unwrap_or_default();
    af.x = ary[2].parse().unwrap_or_default();
    af.y = ary[3].parse().unwrap_or_default();
    af.time = ary[4].parse().unwrap_or_default();

    if ary.len() < 6 {
        return Some(af);
    }

    for c in ary[5].chars() {
        match c {
            'H' | 'h' => af.h *= -1,
            'V' | 'v' => af.v *= -1,
            _ => (),
        }
    }

    if af.h < 0 {
        af.x *= -1;
    }

    if af.v < 0 {
        af.y *= -1;
    }

    if ary.len() < 7 {
        return Some(af);
    }

    if let Some(ia) = ary[6].find(|c: char| c == 'A' || c == 'S' || c == 'a' || c == 's') {
        ary[6] = &ary[6][ia..];
    }

    let a = ary[6].split(',').next().unwrap_or_default().to_lowercase();
    match a.as_str() {
        "a1" => {
            af.src_alpha = 255;
            af.dst_alpha = 128;
        }
        a if !a.is_empty() && a.starts_with('s') => {
            af.src_alpha = 1;
            af.dst_alpha = 255;
        }
        a if a.len() >= 2 && a.starts_with("as") => {
            if a.len() > 2 && a.chars().nth(2).unwrap_or_default().is_digit(10) {
                let mut i = 2;
                let mut alp = 0;
                while i < a.len() && a.chars().nth(i).unwrap_or_default().is_digit(10) {
                    alp = alp * 10 + (a.chars().nth(i).unwrap_or_default() as u8 - b'0') as i32;
                    i += 1;
                }
                alp &= 0x3FFF;
                if alp >= 255 {
                    af.src_alpha = 255;
                } else {
                    af.src_alpha = alp as u8;
                }
                if i < a.len() && a.chars().nth(i).unwrap_or_default() == 'd' {
                    i += 1;
                    if i < a.len() && a.chars().nth(i).unwrap_or_default().is_digit(10) {
                        alp = 0;
                        while i < a.len() && a.chars().nth(i).unwrap_or_default().is_digit(10) {
                            alp = alp * 10
                                + (a.chars().nth(i).unwrap_or_default() as u8 - b'0') as i32;
                            i += 1;
                        }
                        alp &= 0x3FFF;
                        if alp >= 255 {
                            af.dst_alpha = 255;
                        } else {
                            af.dst_alpha = alp as u8;
                        }
                        if af.src_alpha == 1 && af.dst_alpha == 255 {
                            af.src_alpha = 0;
                        }
                    }
                }
            }
        }
        a if !a.is_empty() && a.starts_with('a') => {
            af.src_alpha = 255;
            af.dst_alpha = 255;
        }
        _ => (),
    }
    if ary.len() < 8 || !ary[7].parse::<f32>().is_ok() {
        return Some(af);
    }
    af.ex = Vec::with_capacity(3);
    af.ex[2].push(atof(&ary[7]) as f64);
    if ary.len() < 9 || !ary[8].parse::<f32>().is_ok() {
        return Some(af);
    }
    af.ex[2].push(atof(&ary[8]) as f64);
    if ary.len() < 10 || !ary[9].parse::<f32>().is_ok() {
        return Some(af);
    }
    af.ex[2].push(atof(&ary[9]) as f64);
    Some(af)
}

/// Convierte una cadena de texto en un número de punto flotante (f64) y lo devuelve.
///
/// # Argumentos
///
/// * `s` - Una referencia a una cadena de texto que se convertirá en un número de punto flotante.
///
/// # Retorna
///
/// Un valor de punto flotante (f64) equivalente al contenido de la cadena de texto.
fn atof(s: &str) -> f64 {
    let mut f = 0.0;
    let mut chars = s.trim().chars();
    if let Some(c) = chars.next() {
        let mut a = String::new();
        if c == '-' || c == '+' {
            a = chars.collect();
        } else {
            a.push(c);
            a.push_str(chars.as_str());
        }
        let mut i = 0;
        let mut p = 0;
        let mut m = 0;
        for c in a.chars() {
            if c == '.' {
                if p != 0 {
                    break;
                }
                p = i;
                m = 1;
                continue;
            }
            if !c.is_digit(10) {
                break;
            }
            f = f * 10.0 + (c as u32 - '0' as u32) as f64;
            i += 1;
        }
        let mut e = 0.0;
        if i + m + 1 < a.len()
            && (a.chars().nth(i + m).unwrap_or_default() == 'e'
                || a.chars().nth(i + m).unwrap_or_default() == 'E')
        {
            let mut j = i + m + 1;
            if a.chars().nth(j).unwrap_or_default() == '-'
                || a.chars().nth(j).unwrap_or_default() == '+'
            {
                j += 1;
            }
            while j < a.len() && a.chars().nth(j).unwrap_or_default().is_digit(10) {
                e = e * 10.0 + (a.chars().nth(j).unwrap_or_default() as u32 - '0' as u32) as f64;
                j += 1;
            }
            if e != 0.0 {
                if a.chars().nth(i + m + 1).unwrap_or_default() == '-' {
                    e *= -1.0;
                }
                if p == 0 {
                    p = i;
                }
            }
        }
        if p > 0 {
            f *= 10.0_f64.powi(p as i32 - i as i32 + e as i32);
        }
        if s.chars().next().unwrap_or_default() == '-' {
            f *= -1.0;
        }
    }
    f
}

/// Método para leer línea por línea de un archivo con formato tipo AIR y devolver una animación.
///
/// Procedimiento:
/// - Primero se inicializa por defecto la animación.
/// - Seguidamente se lee línea por línea del archivo y se procesan los frames o las colisiones.
/// - Si se encuentra un frame, se almacena en la animación.
/// - Si no es un frame, se actualizan parámetros extras como si la animación tiene un bucle o el tiempo que dura.
///
/// # Argumentos
///
/// * `lines` - Una referencia a un vector de cadenas de texto que representan las líneas del archivo.
/// * `i` - Un mutable usize que representa el índice actual dentro del vector de líneas.
///
/// # Retorna
///
/// Un resultado que contiene una animación si la lectura fue exitosa, o un error `AirError` si hubo un problema de formato.
fn read_animation(lines: &Vec<&str>, i: &mut usize) -> Result<Animation, AirError> {
    let mut a = Animation::new();

    a.mask = 0;
    let mut ols = 0;
    let (mut clsn1, mut clsn1d, mut clsn2, mut clsn2d): (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let (mut def1, mut def2) = (true, true);
    let mut first = true;

    while *i < lines.len() {
        if !lines[*i].is_empty() && lines[*i].starts_with('[') {
            break;
        }

        let line = lines[*i]
            .splitn(2, ';')
            .next()
            .unwrap()
            .trim()
            .to_lowercase();

        let af = read_anim_frame(&line);

        match af {
            Some(mut af) => {
                ols = a.loopstart;
                af.def1 = if first { false } else { def1 };
                af.def2 = if first { false } else { def2 };
                if def1 || def2 {
                    first = false
                };
                if def1 {
                    clsn1 = clsn1d.clone();
                }
                if def2 {
                    clsn2 = clsn2d.clone();
                }
                if !clsn1.is_empty() || !clsn2.is_empty() {
                    if af.ex.len() < 2 {
                        af.ex = vec![vec![]; 2];
                    }
                    af.ex[0] = clsn1.clone();
                    af.ex[1] = clsn2.clone();
                }
                a.frames.push(af);
                def1 = true;
                def2 = true;
            }
            None => {
                if line.len() >= 9 && &line[..9] == "loopstart" {
                    a.loopstart = a.frames.len() as i32;
                }
                if line.len() >= 18 && &line[..18] == "interpolate offset" {
                    a.interpolate_offset.push(a.frames.len() as i32);
                }
                if line.len() >= 17 && &line[..17] == "interpolate scale" {
                    a.interpolate_scale.push(a.frames.len() as i32);
                }
                if line.len() >= 17 && &line[..17] == "interpolate angle" {
                    a.interpolate_angle.push(a.frames.len() as i32);
                }
                if line.len() >= 17 && &line[..17] == "interpolate blend" {
                    a.interpolate_blend.push(a.frames.len() as i32);
                }
                if line.len() >= 5 && &line[..4] == "clsn" {
                    if let Some(ii) = line.find(":") {
                        let size;
                        match atoi(&line[ii + 1..].to_string()) {
                            Ok(result) => {
                                size = result;
                            }
                            Err(err) => {
                                return Err(AirError::BadFormat(err));
                            }
                        }

                        if size < 0 {
                            break;
                        }

                        let mut clsn;
                        let is_clsn1;
                        if &line[4..5] == "1" {
                            clsn1 = vec![0.0; size as usize * 4];
                            clsn = clsn1.clone();
                            def1 = line.len() >= 12 && &line[5..12] == "default";
                            is_clsn1 = true;
                            first = true;
                        } else if &line[4..5] == "2" {
                            clsn2 = vec![0.0; size as usize * 4];
                            clsn = clsn2.clone();
                            def2 = line.len() >= 12 && &line[5..12] == "default";
                            is_clsn1 = false;
                            first = true;
                        } else {
                            break;
                        }
                        if size == 0 {
                            break;
                        }
                        *i += 1;
                        for n in 0..size {
                            if *i >= lines.len() {
                                break;
                            }
                            let line = lines[*i]
                                .splitn(2, ';')
                                .next()
                                .unwrap()
                                .trim()
                                .to_lowercase();
                            if line.is_empty() {
                                continue;
                            }
                            if line.len() < 4 || !line.starts_with("clsn") {
                                break;
                            }

                            let ii = line.find("=").unwrap_or_default();

                            let ary: Vec<_> = line[ii + 1..].split(',').collect();
                            if ary.len() < 4 {
                                break;
                            }

                            let (l, t, r, b) = (
                                atoi(&ary[0].to_string()).unwrap(),
                                atoi(&ary[1].to_string()).unwrap(),
                                atoi(&ary[2].to_string()).unwrap(),
                                atoi(&ary[3].to_string()).unwrap(),
                            );
                            let (l, r) = if l > r { (r, l) } else { (l, r) };
                            let (t, b) = if t > b { (b, t) } else { (t, b) };
                            clsn[n as usize * 4] = l as f64;
                            clsn[n as usize * 4 + 1] = t as f64;
                            clsn[n as usize * 4 + 2] = r as f64;
                            clsn[n as usize * 4 + 3] = b as f64;
                            *i += 1;
                        }
                        if is_clsn1 {
                            clsn1 = clsn.clone();
                            if def1 {
                                clsn1d = clsn;
                            }
                        } else {
                            clsn2 = clsn.clone();
                            if def2 {
                                clsn2d = clsn;
                            }
                        }
                        *i -= 1;
                    }
                }
            }
        }
        *i += 1;
    }
    if a.loopstart >= a.frames.len() as i32 {
        a.loopstart = ols;
    }
    if a.frames.is_empty() {
    } else if a.frames[a.frames.len() - 1].time == -1 {
        a.totaltime = -1;
    } else {
        let mut tmp = 0;
        for (i, f) in a.frames.iter().enumerate() {
            if f.time == -1 {
                a.totaltime = 0;
                a.looptime = -tmp;
                a.nazotime = 0;
            }
            a.totaltime += f.time;
            if i < a.loopstart as usize {
                a.nazotime += f.time;
                tmp += f.time;
            } else {
                a.looptime += f.time;
            }
        }
        if a.totaltime == -1 {
            a.nazotime = 0;
        }
    }
    Ok(a)
}

/// Método para convertir una cadena de texto en un i32 y devolverlo.
///
/// # Argumentos
///
/// * `string` - Una referencia a una cadena de texto que se desea convertir en un i32.
///
/// # Retorna
///
/// Un resultado que contiene el i32 convertido si la conversión fue exitosa, o un error si hubo un problema.
fn atoi(string: &String) -> Result<i32, Error> {
    let mut n = 0;
    let trimmed_str = string.trim();

    if !trimmed_str.is_empty() {
        let a = if trimmed_str.starts_with('-') || trimmed_str.starts_with('+') {
            &trimmed_str[1..]
        } else {
            trimmed_str
        };

        for c in a.chars() {
            if !c.is_ascii_digit() {
                break;
            }
            n = n * 10 + (c as i64 - '0' as i64);
            if n > 2147483647 {
                println!(
                    "WARNING: Atoi conversion outside int32 range: {}",
                    &trimmed_str[..a.len()]
                );
                if trimmed_str.starts_with('-') {
                    return Err(Error);
                }
                return Ok(0 as i32 >> 1);
            }
        }

        if trimmed_str.starts_with('-') {
            n *= -1;
        }
    }

    Ok(n as i32)
}

/// Método que lee un archivo tipo AIR y devuelve una tabla de animación.
///
/// # Argumentos
///
/// * `air` - Una cadena de texto que representa el contenido del archivo AIR.
///
/// # Retorna
///
/// Un resultado que contiene la tabla de animación si la lectura fue exitosa, o un error `AirError` si hubo un problema de formato.
fn read_animation_table(air: &str) -> Result<AnimationTable, AirError> {
    let mut i = 0;
    let mut at = AnimationTable::new();
    let lines = air.lines().map(|line| line.trim()).collect();
    while let Some(_) = at.read_action(&lines, &mut i)? {}
    Ok(at)
}

/// Método que busca un archivo AIR y devuelve una tabla de animación.
///
/// # Argumentos
///
/// * `air` - Una cadena de texto que representa la ruta del archivo AIR.
///
/// # Retorna
///
/// Un resultado que contiene la tabla de animación si la lectura y el análisis fueron exitosos, o un error `AirError` si hubo un problema.
pub fn parse_air(air: &str) -> Result<AnimationTable, AirError> {
    let content = match std::fs::read_to_string(air) {
        Ok(content) => content,
        Err(_) => return Err(AirError::NotFound(air.to_string())),
    };
    match read_animation_table(&content.as_str()) {
        Ok(at) => return Ok(at),
        Err(err) => return Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Función para comparar vectores f64.
    ///
    /// # Argumentos
    ///
    /// `a` - Un vector f64.
    /// `b` - Otro vector f64 para comparar con a.
    /// `epsilon` - Margen de error a la hora de comparar.
    ///
    /// # Retorna
    ///
    /// True si ambos vectores son iguales, false si son distintos.
    fn approx_eq_vec(a: &Vec<f64>, b: &Vec<f64>, epsilon: f64) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.iter().zip(b.iter()).all(|(a, b)| (a - b).abs() < epsilon)
    }

    #[test]
    /// Prueba si un archivo air bien formado lo crea y si uno que ni existe falla
    fn test_parse_air() {
        assert!(parse_air("src/chars/kfm/kfm.air").is_ok());
        assert!(parse_air("INVALIDAIR.A.A.A").is_err());
    }

    /// Prueba unitaria de la lectura de una acción normal.
    #[test]
    fn test_read_action_valid() {
        let lines = vec![
            "[Begin Action 5]",
            "Clsn1: 1",
            "Clsn1[0] =  16,-80, 61,-71",
            "Clsn2Default: 2",
            "Clsn2[0] = -13,  0, 16,-79",
            "Clsn2[1] =   5,-79, -7,-93",
            "10,20, 13,8, 7",
            "Loopstart",
            "1,10, 0,8, 5",
        ];
        let mut index = 0;
        let result = read_action(&lines, &mut index);

        assert!(result.is_ok());
        let animation = result.unwrap();

        assert!(animation.is_some());
        let (number, animation) = animation.unwrap();
        assert_eq!(number, 5);
        assert_eq!(animation.frames.len(), 2);
        assert_eq!(animation.frames[0].group, 10);
        assert_eq!(animation.frames[0].number, 20);
        assert_eq!(animation.frames[0].x, 13);
        assert_eq!(animation.frames[0].y, 8);
        assert_eq!(animation.frames[0].time, 7);
        assert!(
            approx_eq_vec(
                &animation.frames[0].ex[0],
                &vec![16.0, -80.0, 61.0, -71.0],
                1e-6
            ),
            "Clsn1 are not equal"
        );
        assert!(
            approx_eq_vec(
                &animation.frames[0].ex[1],
                &vec![-13.0, -79.0, 16.0, 0.0, -7.0, -93.0, 5.0, -79.0],
                1e-6
            ),
            "Clsn2 are not equal"
        );

        assert_eq!(animation.loopstart, 1);
        assert_eq!(animation.frames[1].group, 1);
        assert_eq!(animation.frames[1].number, 10);
        assert_eq!(animation.frames[1].x, 0);
        assert_eq!(animation.frames[1].y, 8);
        assert_eq!(animation.frames[1].time, 5);
        assert!(
            approx_eq_vec(&animation.frames[1].ex[0], &vec![], 1e-6),
            "Clsn1 are not equal"
        );
        assert!(
            approx_eq_vec(
                &animation.frames[1].ex[1],
                &vec![-13.0, -79.0, 16.0, 0.0, -7.0, -93.0, 5.0, -79.0],
                1e-6
            ),
            "Clsn2 are not equal"
        );
    }

    /// Prueba unitaria cuando se envía un formato inválido.
    #[test]
    fn test_read_action_invalid_format() {
        let lines = vec!["[Begin Action]", "10,20, 13,8, 7", "1,10, 0,8, 5"];
        let mut index = 0;
        let result = read_action(&lines, &mut index);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    /// Prueba unitaria cuando se envía una acción sin animación.
    #[test]
    fn test_read_action_no_animation() {
        let lines = vec![
            "[Begin Action 5]",
            "Clsn1: 1",
            "Clsn1[0] =  16,-80, 61,-71",
            "Clsn2Default: 2",
            "Clsn2[0] = -13,  0, 16,-79",
            "Clsn2[1] =   5,-79, -7,-93",
        ];
        let mut index = 0;
        let result = read_action(&lines, &mut index);
        assert!(result.is_ok());

        let animation = result.unwrap();
        assert!(animation.is_some());
        let (number, animation) = animation.unwrap();

        assert_eq!(number, 5);
        assert_eq!(animation.frames.len(), 0);
    }

    /// Prueba unitaria cuando se envía una acción con formato incorrecto en las líneas.
    #[test]
    fn test_read_action_invalid_line_format() {
        let lines = vec!["[Begin Action 5]", "Invalid Line", "10,20, 13,8, 7"];
        let mut index = 0;
        let result = read_action(&lines, &mut index);

        assert!(result.is_ok());
        let animation = result.unwrap();

        assert!(animation.is_some());
        let (number, animation) = animation.unwrap();
        assert_eq!(number, 5);
        assert_eq!(animation.frames.len(), 1);
        assert_eq!(animation.frames[0].group, 10);
        assert_eq!(animation.frames[0].number, 20);
        assert_eq!(animation.frames[0].x, 13);
        assert_eq!(animation.frames[0].y, 8);
        assert_eq!(animation.frames[0].time, 7);
    }

    /// Prueba unitaria cuando se envía una acción con un número negativo.
    #[test]
    fn test_read_action_negative_number() {
        let lines = vec![
            "[Begin Action -1]",
            "Clsn1: 1",
            "Clsn1[0] =  16,-80, 61,-71",
            "10,20, 13,8, 7",
        ];
        let mut index = 0;
        let result = read_action(&lines, &mut index);

        assert!(result.is_ok());
        let animation = result.unwrap();

        assert!(animation.is_some());
        let (number, animation) = animation.unwrap();
        assert_eq!(number, -1);
        assert_eq!(animation.frames.len(), 1);
        assert_eq!(animation.frames[0].group, 10);
        assert_eq!(animation.frames[0].number, 20);
        assert_eq!(animation.frames[0].x, 13);
        assert_eq!(animation.frames[0].y, 8);
        assert_eq!(animation.frames[0].time, 7);
    }

    /// Prueba unitaria para la función atoi.
    #[test]
    fn test_atoi() {
        // Casos de prueba válidos
        assert_eq!(atoi(&"123".to_string()), Ok(123));
        assert_eq!(atoi(&"-123".to_string()), Ok(-123));
        assert_eq!(atoi(&"+123".to_string()), Ok(123));
        assert_eq!(atoi(&"   123   ".to_string()), Ok(123));
        assert_eq!(atoi(&"0000123".to_string()), Ok(123));

        // Casos de prueba de entrada no válida
        assert_eq!(atoi(&"abc".to_string()), Ok(0));
        assert_eq!(atoi(&"123abc".to_string()), Ok(123));
        assert_eq!(atoi(&"-123abc".to_string()), Ok(-123));
        assert_eq!(atoi(&"".to_string()), Ok(0));
        assert_eq!(atoi(&"   ".to_string()), Ok(0));

        // Casos de prueba fuera del rango de int32
        assert_eq!(atoi(&"2147483648".to_string()), Ok(0));
        assert_eq!(atoi(&"-2147483649".to_string()), Err(Error));
    }

    /// Prueba unitaria para la función atof.
    #[test]
    fn test_atof() {
        // Casos de prueba válidos
        assert_eq!(atof("123.45"), 123.45);
        assert_eq!(atof("-123.45"), -123.45);
        assert_eq!(atof("+123.45"), 123.45);
        assert_eq!(atof("   123.45   "), 123.45);
        assert_eq!(atof("0.123"), 0.123);
        assert_eq!(atof("123e2"), 12300.0);
        assert_eq!(atof("123e-2"), 1.23);
        assert_eq!(atof("-123e2"), -12300.0);
        assert_eq!(atof("1.23e2"), 123.0);
        assert_eq!(atof("1.23e-2"), 0.0123);

        // Casos de prueba de entrada no válida
        assert_eq!(atof("abc"), 0.0);
        assert_eq!(atof("123.45abc"), 123.45);
        assert_eq!(atof("123.45.67"), 123.45);
        assert_eq!(atof(""), 0.0);
        assert_eq!(atof("   "), 0.0);
    }

    /// Prueba unitaria para la actualización de colisiones
    #[test]
    fn test_update_clsns() {

        let air = parse_air("src/chars/kfm/kfm.air");
        assert!(air.is_ok());
        let mut animation_table = air.unwrap();
        let clsns = &mut animation_table.clsns;
        if let Some(animation) = animation_table.animations.get(&0) {
            
            // Llamar al método update_clsns para actualizar el vector de colisiones
            animation.update_clsns(clsns);
            
            // [Begin Action 0]
            // Clsn2Default: 2
            //  Clsn2[0] = -13,  0, 16,-79
            //  Clsn2[1] =   5,-79, -7,-93
            // 0,0, 0,0, 10
            // 0,1, 0,0, 7
            // 0,2, 0,0, 7
            // 0,3, 0,0, 7
            // 0,4, 0,0, 7
            // 0,5, 0,0, 45
            // 0,4, 0,0, 7
            // 0,3, 0,0, 7
            // 0,2, 0,0, 7
            // 0,1, 0,0, 7
            // 0,0, 0,0, 40

            let expected_clsn0 = Clsn::new(13.0, -79.0, 16.0, 0.0, true);
            let expected_clsn1 = Clsn::new(-7.0, -93.0, 5.0, -79.0, true);
            println!("{:?}", expected_clsn0);
            assert_eq!(clsns.len(), 2);
            assert_eq!(clsns[0].height, expected_clsn0.height);
            assert_eq!(clsns[0].width, expected_clsn0.width);
            assert_eq!(clsns[0].hitbox, expected_clsn0.hitbox);
            assert_eq!(clsns[0].ofs_right, expected_clsn0.ofs_right);
            assert_eq!(clsns[0].ofs_x, expected_clsn0.ofs_x);
            assert_eq!(clsns[0].ofs_y, expected_clsn0.ofs_y);
            assert_eq!(clsns[0].x, expected_clsn0.x);
            assert_eq!(clsns[0].y, expected_clsn0.y);
            
            assert_eq!(clsns[1].height, expected_clsn1.height);
            assert_eq!(clsns[1].width, expected_clsn1.width);
            assert_eq!(clsns[1].hitbox, expected_clsn1.hitbox);
            assert_eq!(clsns[1].ofs_right, expected_clsn1.ofs_right);
            assert_eq!(clsns[1].ofs_x, expected_clsn1.ofs_x);
            assert_eq!(clsns[1].ofs_y, expected_clsn1.ofs_y);
            assert_eq!(clsns[1].x, expected_clsn1.x);
            assert_eq!(clsns[1].y, expected_clsn1.y);
        }
    }
}
