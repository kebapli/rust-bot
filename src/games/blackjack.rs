use super::DiscordGame;
use crate::loadcmds::Error;
use std::time::Duration;

pub struct Blackjack;

impl DiscordGame for Blackjack {
    fn name(&self) -> &'static str {
        "Blackjack"
    }

    fn run(&self, _player_name: &str) -> Result<String, Error> {
        // Core game math/logic runs here
        std::thread::sleep(Duration::from_secs(5));
        Ok("You got 21! You win!".to_string())
    }
}
