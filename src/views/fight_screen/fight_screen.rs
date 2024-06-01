use std::{sync::mpsc::Sender, time::{Duration, Instant}};

use gfx_device_gl::Device;
use graphics::clear;
use piston::Key;
use piston_window::{Context, G2d, Glyphs, PistonWindow, TextureSettings};

use crate::{player::player::Player, views::{common::Screen, screen_manager::{Event, ScreenType}}};

use super::gui;

pub struct FightScreen {
    players: [Player; 2],
    ticks: u16,
    debug: bool,
    last_print_time: Instant,
    last_update: Instant,
    total_time: Duration,
    total_frames: i32,
    timer: u32,
    glyphs: Glyphs,
    end_round: u8,
    event_sender: Sender<Event>,
    average_fps: f64,
}

impl FightScreen {
    pub fn new(window: &mut PistonWindow, event_sender: Sender<Event>, characters: &[String;2], palettes: [usize;2]) -> Self {
        let context_p1 = window.create_texture_context();
        let context_p2 = window.create_texture_context();

        let mut player_one = Player::new(true);
        player_one.choose_char(&characters[0], context_p1);
        player_one.set_palette(palettes[0] - 1);

        let mut player_two = Player::new(false);
        player_two.choose_char(&characters[1], context_p2);
        player_two.set_palette(palettes[1] - 1);

        let players = [player_one, player_two];

        let last_print_time = Instant::now();
        let last_update = Instant::now();
        let total_time: Duration = Duration::new(0, 0);

        let glyphs = Glyphs::new("assets\\fonts\\OpenSans-Regular.ttf", window.create_texture_context(), TextureSettings::new()).unwrap();

        Self {
            players,
            ticks: 0,
            debug: false,
            last_print_time,
            last_update,
            total_time,
            total_frames: -1,
            timer: 100,
            glyphs,
            end_round: 0,
            event_sender,
            average_fps: 0.0,
        }
    }
}

impl Screen for FightScreen {
    fn new(window: &mut PistonWindow, event_sender: Sender<Event>) -> Self where Self: Sized {
        let _ = event_sender;
        let _ = window;
        todo!()
    }

    fn update(&mut self, window: Option<&mut PistonWindow>) {
        let _ = window;
        let delta_time = self.last_update.elapsed();
        self.last_update = Instant::now();
        self.total_time += delta_time;

        if self.end_round == 0 && self.total_time.as_secs() > 100 - self.timer as u64 {
            if self.timer > 0{
                self.timer -= 1;
            }
        }
        self.ticks += 1u16;



        for player in &mut self.players {
            player.update();
        }

        let (prev, curr) = self.players.split_at_mut(1);
        let p1 = prev.last_mut().unwrap();
        let p2 = curr.first_mut().unwrap();
        
        Player::check_collision(p1, p2);

        if self.debug {
            self.total_frames += 1;
            let elapsed_seconds = self.last_print_time.elapsed().as_secs();
            if elapsed_seconds > 0 {
                self.average_fps = (self.total_frames as f64) / (elapsed_seconds as f64);
                self.total_frames = -1;
                self.last_print_time = Instant::now();
            }
        }

        if self.end_round > 0 {
            return;
        }

        let p1_life = p1.get_life();
        let p2_life = p2.get_life();
        
        if p1_life == 0 {
            self.end_round += 1;
            p1.set_lose(true);
            if p2_life > 0 {p2.set_win(true)};
        }
        
        if p2_life == 0 {
            self.end_round += 1;
            p2.set_lose(true);
            if p1_life > 0 {p1.set_win(true)};
        }

        if self.timer == 0 {
            if p1_life > p2_life {
                self.end_round = 1;
                p2.set_lose(true);
                p1.set_win(true);
            }
            else if p1_life < p2_life {
                self.end_round = 2;
                p1.set_lose(true);
                p2.set_win(true);
            }
            else {
                self.end_round = 3;
                p1.set_lose(true);
                p2.set_lose(true);
            }
        }
    }

    fn on_press(&mut self, key: Key) {
        match key {
            Key::F1 => {
                self.debug = !self.debug;
                if self.debug {
                    self.total_frames = -1;
                    self.last_print_time = Instant::now();
                }
            }
            Key::Escape => {
                if self.end_round > 0 {
                    self.event_sender.send(Event::ChangeScreen(ScreenType::Roster)).unwrap();
                }
            }
            _ => {
                for player in &mut self.players {
                    if !player.set_player_input(&key, true) {
                        player.handle_key_input(&mut self.ticks, true);
                        if !player.is_moving(key) && !player.is_replacing_action() {
                            player.set_replacing();
                        }
                    }
                }
            }
        }
    }

    fn on_release(&mut self, key: Key) {
        match key {
            _ => {
                for player in &mut self.players {
                    if player.set_player_input(&key, false) && !player.is_replacing_action() {
                        player.handle_key_input(&mut self.ticks, false);
                    }
                }
            }
        }
    }

    fn draw(&mut self, c: Context, g: &mut G2d, device: &mut Device) {
        clear([1.0; 4], g);
        let priority;
        if !self.players[0].is_doing_action() || self.players[1].is_doing_action() {
            priority = 1
        } else {
            priority = 0
        }
        let mut i = 0;
        while i < 2 {
            self.players[(i + priority) % 2].get_mut_sprite().draw(c.transform, g);
            let player = &self.players[(i + priority) % 2];
            if self.debug {
                for clsn in player.get_clsns() {
                    let rect = clsn.get_rectangle();
                    let color = if clsn.is_hitbox() {
                        [0.0, 1.0, 0.0, 0.5]
                    } else {
                        [1.0, 0.0, 0.0, 0.5]
                    };
                    graphics::rectangle(color, rect, c.transform, g);
                }
            }
            gui::draw_health_bar(c, g, player.get_life_as_percentage(), player.is_first_player());
            gui::draw_power_bar(c, g, player.get_power_as_percentage(), player.is_first_player());
            i += 1;
        }
        if self.debug {
            gui::draw_fps(c, g, device, &mut self.glyphs, self.average_fps);
        }
        gui::draw_timer(c, g, device, &mut self.glyphs, self.timer);
        if self.end_round > 0 {
            gui::end_round(c, g, device, &mut self.glyphs, self.end_round);
        }
    }
    
}
