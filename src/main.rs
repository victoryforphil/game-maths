

use std::fs::File;

use game_maths::maths::{Maths, self};
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
        let t_vec = maths::Vector3D::default();
        info!("Game Maths: {:?}", t_vec);
        let t_mat = maths::Matrix3D::index_test();
        info!("Game Maths: {:?}", t_mat);
    info!("Game Maths: Hello world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}