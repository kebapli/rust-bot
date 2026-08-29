use crate::loadcmds::{Context, Error};
use crate::games::{blackjack::Blackjack, roulette::Roulette, engine::execute_game_session};
use std::sync::Arc;

/// Play a quick session of high-performance Blackjack
#[poise::command(slash_command, prefix_command)]
pub async fn play_blackjack(ctx: Context<'_>) -> Result<(), Error> {
    let game = Arc::new(Blackjack);
    execute_game_session(ctx, game).await
}

/// Play a quick session of high-performance Roulette
#[poise::command(slash_command, prefix_command)]
pub async fn play_roulette(ctx: Context<'_>) -> Result<(), Error> {
    let game = Arc::new(Roulette);
    execute_game_session(ctx, game).await
}
