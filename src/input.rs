
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
pub struct Input {
_quit:bool,
_keys_pressed:Vec<bool>
}

impl Input {

    pub fn new()->Self{

        let _keys_pressed = vec![false;2];

        Input{_quit
            :false,
        _keys_pressed}
    }
pub    fn key_pressed(&self,event_pump:&mut EventPump , _key_pressed:Keycode) -> bool{
        for event in event_pump.poll_iter() {
            match event {
            Event::KeyDown{keycode:Some(_key_pressed),..}=> return true
            , _=> {}    
        }
                }
                false
    }

    pub fn is_quit(&self,event_pump:&mut EventPump) -> bool{
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }=> return true
            , _=> {}    
        }
                }
                false
    }

}