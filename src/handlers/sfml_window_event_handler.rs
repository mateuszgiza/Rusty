use common::logger;
pub use sfml::window::{Event, Key};
use sfml::graphics::{RenderWindow};
use std::sync::Mutex;
use components::*;

pub struct KeyInfo {
    pub key: Key,
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub system: bool
}

lazy_static! {
    static ref Events: Mutex<Vec<Event>> = Mutex::new(vec![]);
    static ref Pressed_Keys: Mutex<Vec<KeyInfo>> = Mutex::new(vec![]);
}

pub struct SfmlWindowEventHandler;
impl SfmlWindowEventHandler {
    pub fn new() -> Self {
        SfmlWindowEventHandler { }
    }

    pub fn handle_events(&mut self, window: &mut RenderWindow) {
        Events.lock().unwrap().clear();
        Pressed_Keys.lock().unwrap().clear();

        while let Some(event) = window.poll_event() {
            Events.lock().unwrap().push(event);
            if let Event::KeyPressed { code, alt, ctrl, shift, system } = event {
                Pressed_Keys.lock().unwrap().push(KeyInfo {key:code, alt, ctrl, shift, system});
            }
        }

        self.check_if_escape_was_pressed(window);
    }

    fn check_if_escape_was_pressed(&self, window: &mut RenderWindow) {
        if Input::is_pressed(Key::Escape) {
            logger::info("Window close or Escape pressed!");
            window.close();
        }
    }
}

pub struct Input;
impl Input {
    pub fn is_pressed(key: Key) -> bool {
        for key_info in Pressed_Keys.lock().unwrap().iter() {
            if key_info.key == key {
                return true;
            }
        }

        return false;
    }

    pub fn log_pressed_key<T: Component>(key: Key) {
        logger::info(&format!("Key {key:?} was pressed in {componentName}!", key=key, componentName=T::name()));
    }
}