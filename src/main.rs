use clap::Parser;
use env_logger::Env;
use gui::gui_main;

mod analysis;
mod gui;
mod rules;
mod sim;
mod space;

#[derive(Parser, Debug)]
struct Args {
    /// Ruleset (e.g. wireworld)
    #[arg(short, long)]
    ruleset: String,
    /// Run analysis instead of launching the GUI
    #[arg(short, long)]
    analyze: bool,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();

    let ruleset = crate::rules::create_ruleset(&args.ruleset).expect("Unknown ruleset");

    if args.analyze {
        let result = crate::analysis::analyze(ruleset);
        print!("{}", result);
        return;
    }

    if args.ruleset.as_str() == "byl" {
        gui_main(ruleset, |sim| {
            crate::rules::byl::create_initial_state(sim);
        });
    } else {
        gui_main(ruleset, |_| {});
    }
}
