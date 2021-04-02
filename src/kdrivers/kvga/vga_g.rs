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
    draw_banner(mode);
}

fn draw_banner(mode: Graphics640x480x16) -> Graphics640x480x16 {
    for (offset, character) in "AuraOS".chars().enumerate() {
        mode.draw_character(32 + offset * 8, 8, character, Color16::White)
    }
    mode
}
