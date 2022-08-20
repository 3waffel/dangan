use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass, Debug)]
#[inherit(Node)]
pub struct Config {
    #[property(default = false)]
    fullscreen: bool,
}

#[methods]
impl Config {
    fn new(owner: &Node) -> Self {
        Self { fullscreen: false }
    }

    #[export]
    fn _ready(&self, owner: &Node) {
        OS::set_window_fullscreen(OS::godot_singleton(), self.fullscreen);
        OS::set_window_position(OS::godot_singleton(), Vector2::ZERO);
        Input::set_mouse_mode(Input::godot_singleton(), Input::MOUSE_MODE_HIDDEN);
    }
}
