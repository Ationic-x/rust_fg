use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use gfx_device_gl::Device;
use graphics::Context;
use piston::Key;
use piston_window::{G2d, PistonWindow};

use crate::{error::pop_up::show_error_popup, preloader::preloader::Preloads};

use super::{common::Screen, FightScreen, LoadingScreen, MainScreen, RosterScreen};

#[derive(PartialEq)]
pub enum ScreenType {
    Main,
    Roster,
    Fight,
}

pub enum Event {
    ScreenReady(),
    SetPalettes([usize; 2]),
    SetCharacters([String; 2]),
    ChangeScreen(ScreenType),
}

pub struct ScreenManager {
    current_screen: Option<Box<dyn Screen>>,
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,
    current_characters: [String; 2],
    current_palettes: [usize; 2],
    switch_screen: ScreenType,
    preloads: Arc<Mutex<Preloads>>,
}

impl ScreenManager {
    pub fn new(screen_type: ScreenType, window: &mut PistonWindow) -> Self {
        let preloads = match Preloads::new(window) {
            Ok(_preloads) => _preloads,
            Err(err) => {
                show_error_popup(&err);
                std::process::exit(1);
            }
        };
        let preloads = Arc::new(Mutex::new(preloads));
        let cloned_preloads = preloads.clone();
        let (tx, rx) = mpsc::channel();
        let cloned_sender = tx.clone();
        let current_characters = [String::new(), String::new()];
        let current_palettes = [0; 2];

        let screen = match screen_type {
            ScreenType::Main => {
                Box::new(MainScreen::new(cloned_sender, cloned_preloads)) as Box<dyn Screen>
            }
            ScreenType::Roster => {
                Box::new(RosterScreen::new(cloned_sender, cloned_preloads)) as Box<dyn Screen>
            }
            ScreenType::Fight => Box::new(FightScreen::new(
                window,
                cloned_sender,
                &current_characters,
                current_palettes,
                cloned_preloads,
            )) as Box<dyn Screen>,
        };

        Self {
            current_screen: Some(screen),
            event_sender: tx,
            event_receiver: rx,
            current_characters,
            current_palettes,
            switch_screen: ScreenType::Main,
            preloads,
        }
    }

    pub fn switch_screen(&mut self, window: &mut PistonWindow) {
        let cloned_sender = self.event_sender.clone();
        self.current_screen = match self.switch_screen {
            ScreenType::Main => Some(
                Box::new(MainScreen::new(cloned_sender, self.preloads.clone())) as Box<dyn Screen>,
            ),
            ScreenType::Roster => Some(Box::new(RosterScreen::new(
                cloned_sender,
                self.preloads.clone(),
            )) as Box<dyn Screen>),
            ScreenType::Fight => Some(Box::new(FightScreen::new(
                window,
                cloned_sender,
                &self.current_characters,
                self.current_palettes,
                self.preloads.clone(),
            )) as Box<dyn Screen>),
        };
    }

    pub fn update(&mut self, window: &mut PistonWindow) {
        if let Some(screen) = self.current_screen.as_mut() {
            screen.update();
        }
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::ChangeScreen(screen_type) => {
                    self.current_screen = Some(Box::new(LoadingScreen::new(
                        self.event_sender.clone(),
                        self.preloads.clone(),
                    )) as Box<dyn Screen>);
                    self.switch_screen = screen_type;
                }
                Event::SetPalettes(palettes) => {
                    self.current_palettes = palettes;
                }
                Event::ScreenReady() => {
                    self.switch_screen(window);
                }
                Event::SetCharacters(char_name) => {
                    self.current_characters[0] = char_name[0].clone();
                    self.current_characters[1] = char_name[1].clone();
                }
            }
        }
    }

    pub fn on_press(&mut self, key: Key) {
        if let Some(screen) = self.current_screen.as_mut() {
            screen.on_press(key);
        }
    }

    pub fn on_release(&mut self, key: Key) {
        if let Some(screen) = self.current_screen.as_mut() {
            screen.on_release(key);
        }
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d, device: &mut Device) {
        if let Some(screen) = self.current_screen.as_mut() {
            screen.draw(c, g, device);
        }
    }
}
