use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass, Debug)]
#[inherit(Control)]
pub struct Cursor {
    sprite: Option<Ref<Sprite>>,
    delta_sum: f32,
}

#[methods]
impl Cursor {
    fn new(owner: &Control) -> Self {
        Self {
            sprite: None,
            delta_sum: 0.,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Control) {
        let sprite = owner.get_node("Sprite").expect("Sprite not found");
        let sprite = unsafe {
            sprite
                .assume_safe()
                .cast::<Sprite>()
                .unwrap()
                .assume_shared()
        };
        self.sprite = Some(sprite);

        godot_print!("Cursor initialized\n{:?}", self);
    }

    #[export]
    fn _process(&mut self, owner: &Control, delta: f32) {
        self.delta_sum += delta;
        let sprite = unsafe { self.sprite.unwrap().assume_safe() };
        let tmp = self.delta_sum;
        let position = Vector2::new(
            tmp.cos() * 20. - tmp.sin() * 20.,
            tmp.cos() * 20. - tmp.sin() * 20.,
        );
        sprite.set_position(position);
    }
}
