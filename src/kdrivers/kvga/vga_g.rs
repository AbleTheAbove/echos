use crate::jump;
pub use vga::colors::Color16;
pub use vga::writers::{Graphics640x480x16, GraphicsWriter};
pub fn hi() {
    jump!();
}

#[macro_export]
macro_rules! jump {
    ($($arg:tt)*) => {
        draw();
    };
}

fn draw() {
    let black = Color16::Black;

    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(black);
    mode.draw_character(0, 0, 'c', Color16::White);
    for (offset, character) in "Hello World!".chars().enumerate() {
        mode.draw_character(270 + offset * 8, 72, character, Color16::White)
    }
}
