use raylib::prelude::*;

use crate::{
    rules::{Cell, RuleSet},
    sim::Simulation,
};

struct GuiState {
    sim: Simulation,
    current_cell: Cell,
}

impl GuiState {
    pub fn new(ruleset: RuleSet) -> Self {
        GuiState {
            sim: Simulation::new(ruleset),
            current_cell: Cell(0),
        }
    }
}

fn palette(cell_idx: u8) -> Color {
    match cell_idx {
        0 => Color::from_hex("073642").unwrap(),
        1 => Color::from_hex("268bd2").unwrap(),
        2 => Color::from_hex("2aa198").unwrap(),
        3 => Color::from_hex("859900").unwrap(),
        4 => Color::from_hex("6c71c4").unwrap(),
        5 => Color::from_hex("d33682").unwrap(),
        6 => Color::from_hex("b58900").unwrap(),
        7 => Color::from_hex("cb4b16").unwrap(),
        8 => Color::from_hex("dc322f").unwrap(),
        _ => panic!("Unsupported cell type count"),
    }
}

pub fn gui_main(ruleset: RuleSet, set_up: impl FnOnce(&mut Simulation)) {
    logging::set_trace_log(TraceLogLevel::LOG_WARNING);

    const WIDTH: i32 = 1024;
    const HEIGHT: i32 = 768;
    const MARGIN: i32 = 20;
    const RIGHT_SIDE_WIDTH: i32 = 200;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH + MARGIN * 2, HEIGHT + MARGIN * 2)
        .title(&format!("Cellariella v{}", env!("CARGO_PKG_VERSION")))
        .build();

    let mut state = GuiState::new(ruleset);

    set_up(&mut state.sim);

    const STEP_TIME: f64 = 0.100;
    let mut time = rl.get_time();
    let mut autostep = false;
    while !rl.window_should_close() {
        // ===== HIT TEST =====

        const CELL_SIZE: i32 = 12;
        const FIELD_SIZE: i64 = 32;

        let mouse_pos = rl.get_mouse_position();
        let mouse_pos_space_x =
            (mouse_pos.x as i64 - MARGIN as i64) / CELL_SIZE as i64 - FIELD_SIZE;
        let mouse_pos_space_y =
            (mouse_pos.y as i64 - MARGIN as i64) / CELL_SIZE as i64 - FIELD_SIZE;

        // ===== INTERACTION =====

        if let Some(k) = rl.get_key_pressed() {
            match k {
                KeyboardKey::KEY_ZERO => {
                    state.current_cell = Cell(0);
                }
                KeyboardKey::KEY_ONE => {
                    state.current_cell = Cell(1);
                }
                KeyboardKey::KEY_TWO => {
                    state.current_cell = Cell(2);
                }
                KeyboardKey::KEY_THREE => {
                    state.current_cell = Cell(3);
                }
                KeyboardKey::KEY_FOUR => {
                    state.current_cell = Cell(4);
                }
                KeyboardKey::KEY_FIVE => {
                    state.current_cell = Cell(5);
                }
                KeyboardKey::KEY_SIX => {
                    state.current_cell = Cell(6);
                }
                KeyboardKey::KEY_SEVEN => {
                    state.current_cell = Cell(7);
                }
                KeyboardKey::KEY_EIGHT => {
                    state.current_cell = Cell(8);
                }
                KeyboardKey::KEY_NINE => {
                    state.current_cell = Cell(9);
                }
                KeyboardKey::KEY_SPACE => {
                    if autostep {
                        autostep = false;
                    } else {
                        if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                            autostep = true;
                            time = rl.get_time();
                        }
                        state.sim.step();
                    }
                }
                KeyboardKey::KEY_R => {
                    state
                        .sim
                        .randomize(-FIELD_SIZE, FIELD_SIZE, -FIELD_SIZE, FIELD_SIZE);
                }
                _ => {}
            }
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            state
                .sim
                .set(mouse_pos_space_x, mouse_pos_space_y, state.current_cell);
        } else if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            state.sim.set(mouse_pos_space_x, mouse_pos_space_y, Cell(0));
        }

        let wheel_move = rl.get_mouse_wheel_move() as i32;
        if wheel_move != 0 {}

        // ===== HANDLING =====

        if autostep {
            let curr_time = rl.get_time();
            if curr_time - time >= STEP_TIME {
                time = curr_time;
                state.sim.step();
            }
        }

        // ===== DRAWING =====

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::from_hex("002b36").unwrap());

        for x in -FIELD_SIZE..FIELD_SIZE {
            for y in -FIELD_SIZE..FIELD_SIZE {
                let cell = state.sim.get(x, y);
                let color = palette(cell.0);
                d.draw_rectangle(
                    MARGIN + (x + FIELD_SIZE) as i32 * CELL_SIZE,
                    MARGIN + (y + FIELD_SIZE) as i32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );
            }
        }
        for x in -FIELD_SIZE..=FIELD_SIZE {
            d.draw_line(
                MARGIN + (x + FIELD_SIZE) as i32 * CELL_SIZE,
                MARGIN,
                MARGIN + (x + FIELD_SIZE) as i32 * CELL_SIZE,
                MARGIN + 2 * FIELD_SIZE as i32 * CELL_SIZE,
                Color::from_hex("586e75").unwrap(),
            );
        }
        for y in -FIELD_SIZE..=FIELD_SIZE {
            d.draw_line(
                MARGIN,
                MARGIN + (y + FIELD_SIZE) as i32 * CELL_SIZE,
                MARGIN + 2 * FIELD_SIZE as i32 * CELL_SIZE,
                MARGIN + (y + FIELD_SIZE) as i32 * CELL_SIZE,
                Color::from_hex("586e75").unwrap(),
            );
        }

        // Right side

        let lines = &[
            "Commands:".to_owned(),
            "  - Step: <Space>".to_owned(),
            "  - Auto step: <Shift+Space>".to_owned(),
            "  - Pick cell: <0..9>".to_owned(),
            "  - Randomize all: <R>".to_owned(),
            format!("Current simulation step: {}", state.sim.current_step()),
            "Logical mouse coords:".to_owned(),
            format!("  - x: {}", mouse_pos_space_x),
            format!("  - y: {}", mouse_pos_space_y),
            format!(
                "  - Cell: {}",
                state.sim.get(mouse_pos_space_x, mouse_pos_space_y).0
            ),
        ];

        for (idx, line) in lines.iter().enumerate() {
            d.draw_text(
                line,
                WIDTH - RIGHT_SIDE_WIDTH,
                MARGIN + 12 * idx as i32,
                12,
                Color::from_hex("839496").unwrap(),
            );
        }
    }
}
