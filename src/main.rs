

use std::fs::File;

use game_maths::maths::Maths;
use log::{LevelFilter, info};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("logs/game-maths.log").unwrap(),
        ),
    ])
    .unwrap();
let _maths = Maths   ::new();
    info!("Game Maths: Hello world!")
}
