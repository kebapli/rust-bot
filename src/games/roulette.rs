use super::DiscordGame;
use crate::loadcmds::Error;
use std::time::Duration;

pub struct Roulette;

impl DiscordGame for Roulette {
    fn name(&self) -> &'static str {
        "Roulette"
    }

    fn run(&self, _player_name: &str) -> Result<String, Error> {
        // Core game math/logic runs here
        std::thread::sleep(Duration::from_secs(3));
        Ok("The ball landed on Red 23! You win!".to_string())
    }
}
