#![allow(unused)]

mod ui;
mod config;
use gdnative::prelude::*;
use ui::{cursor::Cursor, ui_main::*};
use config::*;

fn init(handle: InitHandle) {
    handle.add_class::<UI>();
    handle.add_class::<Cursor>();
    handle.add_class::<Config>();
}

godot_init!(init);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
