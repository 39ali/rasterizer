use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use std::collections::HashSet;
pub struct Input {
    quit: bool,
    keys_pressed: HashSet<Scancode>,
}

impl Default for Input {
    fn default() -> Self {
        let keys_pressed = HashSet::new();

        Input {
            quit: false,
            keys_pressed,
        }
    }
}

impl Input {
    pub fn poll_events(&mut self, event_pump: &mut EventPump) {
        self.keys_pressed.clear();
        self.quit = false;
        
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                self.quit = true;
            };
        }
      for scancode in event_pump.keyboard_state().pressed_scancodes(){
        self.keys_pressed.insert(scancode);
      }

    }
    pub fn key_pressed(&self, key: Scancode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }
}
