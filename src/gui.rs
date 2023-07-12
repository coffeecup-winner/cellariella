use raylib::prelude::*;

pub fn gui_main() {
    logging::set_trace_log(TraceLogLevel::LOG_WARNING);

    const WIDTH: i32 = 1024;
    const HEIGHT: i32 = 768;
    const MARGIN: i32 = 20;
    const RIGHT_SIDE_WIDTH: i32 = 300;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH + MARGIN * 2, HEIGHT + MARGIN * 2)
        .title(&format!("Cellariella v{}", env!("CARGO_PKG_VERSION")))
        .build();

    while !rl.window_should_close() {
        // ===== HIT TEST =====

        // TODO

        // ===== INTERACTION =====

        if let Some(k) = rl.get_key_pressed() {
            match k {
                _ => {}
            }
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        } else if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
        }
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_MIDDLE_BUTTON) {
        } else if rl.is_mouse_button_released(MouseButton::MOUSE_MIDDLE_BUTTON) {
        }

        let wheel_move = rl.get_mouse_wheel_move() as i32;
        if wheel_move != 0 {}

        // ===== HANDLING =====

        // TODO

        // ===== DRAWING =====

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);

        // Right side

        let lines = &["Commands:".to_owned(), "  - TODO".to_owned()];

        for (idx, line) in lines.iter().enumerate() {
            d.draw_text(
                line,
                WIDTH - RIGHT_SIDE_WIDTH,
                MARGIN + 12 * idx as i32,
                12,
                Color::BLACK,
            );
        }
    }
}
