use std::borrow::Borrow;
use std::borrow::BorrowMut;

use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass, Debug)]
#[inherit(Node)]
pub struct UI {
    #[property]
    cursor_position: Vector2,
    cursor: Option<Ref<Control>>,
}

#[methods]
impl UI {
    fn new(owner: &Node) -> Self {
        Self {
            cursor_position: Vector2::ZERO,
            cursor: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node>) {
        let cursor = unsafe { owner.get_node("Cursor").unwrap().assume_safe() };
        self.cursor = unsafe { Some(cursor.cast::<Control>().unwrap().assume_shared()) };

        godot_print!("UI initialized.\n{:?}", self);
    }

    #[export]
    fn _process(&mut self, owner: &Node, delta: f32) {
        let cursor = unsafe { self.cursor.unwrap().assume_safe() };
        cursor.set_position(self.cursor_position, false);
    }

    #[export]
    fn _input(&mut self, owner: &Node, event: Ref<InputEvent>) {
        let event = unsafe { event.assume_safe() };
        match event.cast::<InputEventMouse>() {
            Some(e) => {
                self.cursor_position = e.position();
            }
            None => {}
        }
    }
}
