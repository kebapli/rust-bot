use crate::commands;
use crate::state::BotState;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, BotState, Error>;

pub fn get_all_commands() -> Vec<poise::Command<BotState, Error>> {
    vec![
        commands::general::heavy_compute(),
        commands::general::ram_stress(),
        // Add your dynamic session command here
        commands::blackjack::blackjack(),
    ]
}
