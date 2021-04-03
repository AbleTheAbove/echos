pub use vga::colors::Color16;
pub use vga::writers::{Graphics640x480x16, GraphicsWriter};

pub fn draw() {
    let mut mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);

    mode.draw_line((0, 0), (640, 480), Color16::Green);
    draw_cursor(mode, 168, 400);
}

fn draw_cursor(mode: Graphics640x480x16, offset_x: isize, offset_y: isize) -> Graphics640x480x16 {
    let cursor_color = Color16::Red;
    mode.draw_line(
        (0 + offset_x, 0 + offset_y),
        (0 + offset_x, 10 + offset_y),
        cursor_color,
    );
    mode.draw_line(
        (0 + offset_x, 0 + offset_y),
        (10 + offset_x, 10 + offset_y),
        cursor_color,
    );

    mode.draw_line(
        (0 + offset_x, 0 + offset_y),
        (10 + offset_x, 0 + offset_y),
        cursor_color,
    );

    mode.draw_line(
        (0 + offset_x, 10 + offset_y),
        (10 + offset_x, 0 + offset_y),
        cursor_color,
    );
    mode
}
