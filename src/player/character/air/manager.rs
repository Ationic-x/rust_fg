use std::{
    collections::HashMap,
    fmt::Error,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{chars::Character, player::character::sff::{self, decoder::Sff}};
use gfx_device_gl::Resources;
use piston_window::{G2dTextureContext, Texture};
use sprite::Sprite;

#[derive(Debug, PartialEq)]
pub struct Clsn {
    hitbox: bool,
    ofs_x: f64,
    ofs_right: f64,
    ofs_y: f64,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub struct AnimationTable {
    animations: HashMap<i32, Animation>,
    sff: Option<Arc<Mutex<Sff>>>,
    spr: Option<Arc<Mutex<Sprite<Texture<Resources>>>>>,
    clsns: Vec<Clsn>,
}

#[derive(Clone, Debug)]
pub struct AnimFrame {
    pub time: i32,
    pub group: i16,
    pub number: i16,
    x: i16,
    y: i16,
    src_alpha: u8,
    dst_alpha: u8,
    h: i8,
    v: i8,
    pub ex: Vec<Vec<f64>>,
    def1: bool,
    def2: bool,
}

#[derive(Clone)]
pub struct Animation {
    sff: Option<Arc<Mutex<Sff>>>,
    spr: Option<Arc<Mutex<Sprite<Texture<Resources>>>>>,
    pub frames: Vec<AnimFrame>,
    //tile: Tiling,
    loopstart: i32,
    interpolate_offset: Vec<i32>,
    interpolate_scale: Vec<i32>,
    interpolate_angle: Vec<i32>,
    interpolate_blend: Vec<i32>,
    current: i32,
    drawidx: i32,
    time: i32,
    sumtime: i32,
    pub totaltime: i32,
    looptime: i32,
    nazotime: i32,
    mask: i16,
    // srcAlpha: i16,
    // dstAlpha: i16,
    loopend: bool,
    // interpolate_offset_x: f64,
    // interpolate_offset_y: f64,
    // scale_x: f64,
    // scale_y: f64,
    // angle: f32,
    // interpolate_blend_srcalpha: f32,
    // interpolate_blend_dstalpha: f32,
    //remap: RemapPreset,
    // start_scale: [f32; 2],
}

impl AnimFrame {
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
    fn new(left: f64, top: f64, right: f64, bottom: f64, hitbox: bool) -> Self {
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

    pub fn get_rectangle(&self) -> [f64; 4] {
        [self.x, self.y, self.width, self.height]
    }

    pub fn set_position(&mut self, trans_x: f64, trans_y: f64, flip: bool) {
        if flip {
            self.x = trans_x - (self.ofs_x + self.width);
        } else {
            self.x = trans_x + self.ofs_x;
        }
        self.y = trans_y + self.ofs_y;
    }

    pub fn is_hitbox(&self) -> bool {
        self.hitbox
    }

    pub fn collides(&self, clsn_p2: &Clsn) -> bool {
        !(self.x + self.width < clsn_p2.x
            || self.x > clsn_p2.x + clsn_p2.width
            || self.y + self.height < clsn_p2.y
            || self.y > clsn_p2.y + clsn_p2.height)
    }
}

impl Animation {
    fn new() -> Self {
        Self {
            //palettedata: &'a PaletteList,
            sff: None,
            spr: None,
            frames: Vec::new(),
            //tile: Tiling,
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
            // srcAlpha: -1,
            // dstAlpha: 0,
            loopend: false,
            // interpolate_offset_x: 0.0,
            // interpolate_offset_y: 0.0,
            // scale_x: 0.0,
            // scale_y: 0.0,
            // angle: 0.0,
            // interpolate_blend_srcalpha: 0.0,
            // interpolate_blend_dstalpha: 0.0,
            //remap: RemapPreset,
            // start_scale: [0f32; 2],
        }
    }

    fn set_sff(&mut self, sff: Arc<Mutex<Sff>>) {
        self.sff = Some(sff);
    }

    pub fn reset(&mut self) {
        self.current = 0;
        self.drawidx = 0;
        self.time = -1;
        self.sumtime = 0;
        self.loopend = false;
    }

    pub fn delta_time(&self) -> i32 {
        self.totaltime - self.sumtime
    }

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

    pub fn set_sprite(&mut self, spr: Option<Arc<Mutex<Sprite<Texture<Resources>>>>>) {
        self.spr = spr;
    }
}

impl AnimationTable {
    fn new() -> Self {
        Self {
            animations: HashMap::new(),
            sff: None,
            spr: None,
            clsns: Vec::new(),
        }
    }

    pub fn set_palette(&mut self, palette: usize) {
        self.sff.as_mut().unwrap().lock().unwrap().set_palette(palette);
    }

    pub fn get_sprite(&self) -> MutexGuard<Sprite<Texture<Resources>>> {
        self.spr.as_ref().unwrap().lock().unwrap()
    }

    fn read_action(&mut self, lines: &Vec<&str>, i: &mut usize) -> Option<Animation> {
        while *i < lines.len() {
            let mut a;
            if let Some(a1) = read_action(&lines, i) {
                a = a1.1.clone();
                if let Some(tmp) = self.animations.get(&a1.0) {
                    return Some(tmp.clone());
                }
                self.animations.insert(a1.0, a1.1.clone());
                while a1.1.frames.is_empty() && *i < lines.len() {
                    if let Some(a2) = self.read_action(lines, i) {
                        a = a2;
                        break;
                    }
                    *i += 1;
                }
                return Some(a);
            }
            *i += 1;
        }
        None
    }

    pub fn set_sprite(&mut self, spr: sprite::Sprite<Texture<Resources>>) {
        let sprite = Arc::new(Mutex::new(spr));

        self.spr = Some(sprite);
        self.set_animation_sprite();
    }

    fn set_animation_sprite(&mut self) {
        for animation in self.animations.values_mut() {
            animation.set_sprite(self.spr.clone());
        }
    }

    pub fn set_sff(
        &mut self,
        char_name: &str,
        filename: String,
        char: bool,
        context: G2dTextureContext,
    ) {
        self.sff = Some(Arc::new(
            Mutex::new(
                sff::decoder::Sff::load_sff(char_name, filename, char, context).unwrap()
            )
        ));
        self.set_animation_sff();
    }

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

    // pub fn reset_animations(&mut self) {
    //     for animation in self.animations.values_mut() {
    //         animation.reset();
    //     }
    // }

    pub fn update_sprite(&mut self, char: &mut Box<dyn Character>) {
        let animation = self.animations.get_mut(char.get_anim()).unwrap();
        let flip = char.is_flipped();
        if char.get_new_anim() {
            animation.reset();
            char.set_new_anim(false);
            char.set_hit(-1);
            char.set_time(-1);
            self.spr.as_mut().unwrap().lock().unwrap().set_flip_x(flip);
            self.spr
                .as_mut()
                .unwrap()
                .lock()
                .unwrap()
                .set_anchor(if flip { 1.0 } else { 0.0 }, 0.0);
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

    pub fn get_clsns(&self) -> &Vec<Clsn> {
        &self.clsns
    }
}

fn read_action(lines: &Vec<&str>, i: &mut usize) -> Option<(i32, Animation)> {
    let length = lines.len();

    let mut name = String::new();
    let mut subname = String::new();

    // Reading section name
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
        return None;
    }

    let spi;
    match subname.find(" ") {
        Some(pos) => {
            spi = pos;
        }
        None => return None,
    }

    if &subname[..spi] != "action" {
        return None;
    }

    *i += 1;

    return Some((
        atoi(&subname[spi + 1..].to_string()).unwrap(),
        read_animation(lines, i),
    ));
}

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
        for c in a.chars() {
            if c == '.' {
                if p != 0 {
                    break;
                }
                p = i + 1;
                continue;
            }
            if !c.is_digit(10) {
                break;
            }
            f = f * 10.0 + (c as u32 - '0' as u32) as f64;
            i += 1;
        }
        let mut e = 0.0;
        if i + 1 < a.len()
            && (a.chars().nth(i).unwrap_or_default() == 'e'
                || a.chars().nth(i).unwrap_or_default() == 'E')
        {
            let mut j = i + 1;
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
                if a.chars().nth(i + 1).unwrap_or_default() == '-' {
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

fn read_animation(lines: &Vec<&str>, i: &mut usize) -> Animation {
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
                            Err(_) => {
                                break;
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
    a
}

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

fn read_animation_table(air: &str) -> AnimationTable {
    let mut i = 0;
    let mut at = AnimationTable::new();
    let lines: Vec<&str> = air.lines().map(|line| line.trim()).collect();
    while let Some(_) = at.read_action(&lines, &mut i) {}
    at
}

pub fn parse_air(air: &str) -> AnimationTable {
    let content = std::fs::read_to_string(air).expect("Cant read the file");
    read_animation_table(&content.as_str())
}
