use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    time::{Duration, Instant},
};

use gfx_device_gl::Device;
use graphics::{clear, image};
use piston::Key;
use piston_window::{Context, G2d, PistonWindow};

use crate::{
    player::player::Player, preloader::preloader::Preloads, views::{
        screen::Screen,
        screen_manager::{Event, ScreenType},
    }
};

use super::gui;

/// Enumera los estados posibles de la pantalla de combate.
#[derive(PartialEq)]
enum State {
    /// Estado de inicio de la pelea.
    StartFight,
    /// Estado de combate.
    Fighting,
    /// Estado de fin de la pelea.
    EndFight,
}

/// Representa la pantalla de combate del juego.
pub struct FightScreen {
    players: [Player; 2],
    ticks: u16,
    debug: bool,
    last_print_time: Instant,
    last_update: Instant,
    total_time: Duration,
    total_frames: i32,
    timer: u32,
    end_round: u8,
    event_sender: Sender<Event>,
    average_fps: f64,
    state: State,
    preloads: Arc<Mutex<Preloads>>,
}

impl FightScreen {
      /// Crea una nueva instancia de `FightScreen`.
    ///
    /// # Argumentos
    ///
    /// * `window` - La ventana de Piston para el juego.
    /// * `event_sender` - El canal de eventos para comunicarse con el administrador de pantallas.
    /// * `characters` - Los nombres de los personajes que participarán en la pelea.
    /// * `palettes` - Las paletas de colores de los personajes.
    /// * `preloads` - Los recursos precargados para el juego.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `FightScreen`.
    pub fn new(
        window: &mut PistonWindow,
        event_sender: Sender<Event>,
        characters: &[String; 2],
        palettes: [usize; 2],
        preloads: Arc<Mutex<Preloads>>
    ) -> Self {
        let context_p1 = window.create_texture_context();
        let mut player_one = Player::new(true);
        player_one.choose_char(&characters[0], context_p1);
        player_one.set_palette(palettes[0] - 1);

        let context_p2 = window.create_texture_context();
        let mut player_two = Player::new(false);
        player_two.choose_char(&characters[1], context_p2);
        player_two.set_palette(palettes[1] - 1);

        let players = [player_one, player_two];

        let last_print_time = Instant::now();
        let last_update = Instant::now();
        let total_time: Duration = Duration::new(0, 0);

        Self {
            players,
            ticks: 0,
            debug: false,
            last_print_time,
            last_update,
            total_time,
            total_frames: -1,
            timer: 3,
            end_round: 0,
            event_sender,
            average_fps: 0.0,
            state: State::StartFight,
            preloads,
        }
    }
}

impl Screen for FightScreen {
    /// Crea una nueva instancia de `FightScreen`.
    ///
    /// # Argumentos
    ///
    /// * `event_sender` - El canal de eventos para comunicarse con el administrador de pantallas.
    /// * `preloads` - Los recursos precargados para el juego.
    ///
    /// # Retorna
    ///
    /// Una nueva instancia de `FightScreen`.
    fn new(event_sender: Sender<Event>, preloads: Arc<Mutex<Preloads>>) -> Self
    where
        Self: Sized,
    {
        let _ = preloads;
        let _ = event_sender;
        todo!()
    }
    
     /// Actualiza el estado de la pantalla de combate.
    fn update(&mut self) {
        let delta_time = self.last_update.elapsed();
        self.last_update = Instant::now();
        self.total_time += delta_time;

        if self.debug {
            self.total_frames += 1;
            let elapsed_seconds = self.last_print_time.elapsed().as_secs();
            if elapsed_seconds > 0 {
                self.average_fps = (self.total_frames as f64) / (elapsed_seconds as f64);
                self.total_frames = -1;
                self.last_print_time = Instant::now();
            }
        }
        
        for player in &mut self.players {
            player.update();
        }
        
        if self.state == State::StartFight {
            if self.total_time.as_secs() > 3 - self.timer as u64 {
                if self.timer > 0 {
                    self.timer -= 1;
                } else {
                    self.timer = 104;
                    self.state = State::Fighting;
                }
            }
        }
        
        if self.state == State::Fighting {
            if self.total_time.as_secs() > 104 - self.timer as u64 {
                if self.timer > 0 {
                    self.timer -= 1;
                }
            }
            self.ticks += 1u16;


            let (prev, curr) = self.players.split_at_mut(1);
            let p1 = prev.last_mut().unwrap();
            let p2 = curr.first_mut().unwrap();

            Player::check_collision(p1, p2);

            let p1_life = p1.get_life();
            let p2_life = p2.get_life();

            if p1_life == 0 {
                self.end_round += 2;
                self.state = State::EndFight;
                p1.set_lose(true);
                if p2_life > 0 {
                    p2.set_win(true);
                };
            }

            if p2_life == 0 {
                self.end_round += 1;
                self.state = State::EndFight;
                p2.set_lose(true);
                if p1_life > 0 {
                    p1.set_win(true);
                };
            }

            if self.timer == 0 {
                self.state = State::EndFight;
                if p1_life > p2_life {
                    self.end_round = 1;
                    p2.set_lose(true);
                    p1.set_win(true);
                } else if p1_life < p2_life {
                    self.end_round = 2;
                    p1.set_lose(true);
                    p2.set_win(true);
                } else {
                    self.end_round = 3;
                    p1.set_lose(true);
                    p2.set_lose(true);
                }
            }
        }
    }


    /// Maneja el evento de presionar una tecla en la pantalla de combate.
    ///
    /// # Argumentos
    ///
    /// * `key` - La tecla que se ha presionado.
    fn on_press(&mut self, key: Key) {
        match key {
            k if k == Key::F1 && self.state == State::Fighting => {
                self.debug = !self.debug;
                if self.debug {
                    self.total_frames = -1;
                    self.last_print_time = Instant::now();
                }
            }
            k if k == Key::Escape && self.state == State::EndFight => {
                if self.end_round > 0 {
                    self.event_sender
                        .send(Event::ChangeScreen(ScreenType::Roster))
                        .unwrap();
                }
            }
            _ if self.state == State::Fighting => {
                for player in &mut self.players {
                    if !player.set_player_input(&key, true) {
                        player.handle_key_input(&mut self.ticks, true);
                        if !player.is_moving(key) && !player.is_replacing_action() {
                            player.set_replacing();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Maneja el evento de soltar una tecla en la pantalla de combate.
    ///
    /// # Argumentos
    ///
    /// * `key` - La tecla que se ha soltado.
    fn on_release(&mut self, key: Key) {
        match key {
            _ if self.state == State::Fighting => {
                for player in &mut self.players {
                    if player.set_player_input(&key, false) && !player.is_replacing_action() {
                        player.handle_key_input(&mut self.ticks, false);
                    }
                }
            }
            _ => {}
        }
    }

    /// Dibuja el contenido de la pantalla de combate.
    ///
    /// # Argumentos
    ///
    /// * `c` - El contexto de dibujo.
    /// * `g` - El contexto de gráficos.
    /// * `device` - El dispositivo de dibujo.
    fn draw(&mut self, c: Context, g: &mut G2d, device: &mut Device) {
        clear([1.0; 4], g);
        let mut preloads = self.preloads.lock().unwrap();
        image(preloads.get_mut_ref_background().get(1).unwrap(), c.transform, g);
        if self.state == State::StartFight {
            gui::draw_countdown(c, g, device, preloads.get_mut_ref_fonts().get_mut(1).unwrap(), self.timer);
        }

        if self.state == State::Fighting || self.state == State::EndFight {
            let priority;
            if !self.players[0].is_doing_action() || self.players[1].is_doing_action() {
                priority = 1
            } else {
                priority = 0
            }
            let mut i = 0;
            while i < 2 {
                self.players[(i + priority) % 2]
                    .get_mut_sprite()
                    .draw(c.transform, g);
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
                gui::draw_health_bar(
                    c,
                    g,
                    player.get_life_as_percentage(),
                    player.is_first_player(),
                );
                gui::draw_power_bar(
                    c,
                    g,
                    player.get_power_as_percentage(),
                    player.is_first_player(),
                );
                i += 1;
            }
            if self.debug {
                gui::draw_fps(c, g, device, preloads.get_mut_ref_fonts().get_mut(1).unwrap(), self.average_fps);
            }
            gui::draw_timer(c, g, device, preloads.get_mut_ref_fonts().get_mut(1).unwrap(), self.timer);

            if self.state == State::EndFight {
                gui::end_round(c, g, device, preloads.get_mut_ref_fonts().get_mut(1).unwrap(), self.end_round);
            }
        }
    }
}
