pub mod blackjack;
pub mod roulette;
pub mod engine;

use crate::loadcmds::Error;

/// The template interface that all massive-RAM dynamic games inherit
pub trait DiscordGame: Send + Sync {
    fn name(&self) -> &'static str;
    fn run(&self, player_name: &str) -> Result<String, Error>;
}
