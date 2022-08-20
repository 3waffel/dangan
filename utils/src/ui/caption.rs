use std::borrow::Borrow;

use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass, Debug)]
#[inherit(Control)]
pub struct Caption {
    index: i32,
    #[property]
    texts: StringArray,
    #[property]
    speed: f32,
}

#[methods]
impl Caption {
    fn new(owner: &Control) -> Self {
        Self {
            index: 0,
            texts: StringArray::from_vec(vec!["test".into()]),
            speed: 1.,
        }
    }

    #[export]
    fn _ready(&self, owner: &Control) {
        self.spawn(owner);
    }

    #[export]
    fn _process(&self, owner: &Control, delta: f32) {}

    #[export]
    fn spawn(&self, owner: &Control) {
        let text_field = TextEdit::new();
        owner.add_child(text_field, false);
    }
}
