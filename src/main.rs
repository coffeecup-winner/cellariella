use env_logger::Env;
use gui::gui_main;

mod gui;
mod rules;
mod sim;
mod space;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    gui_main();
}
