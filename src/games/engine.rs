use crate::loadcmds::{Context, Error};
use super::DiscordGame;
use std::sync::Arc;

/// Spawns a dedicated core thread and destroys 10MB of RAM for ANY game engine passed into it.
pub async fn execute_game_session(ctx: Context<'_>, game: Arc<dyn DiscordGame>) -> Result<(), Error> {
    let player_name = ctx.author().name.clone();
    ctx.say(format!("🚀 Launching {} engine for {}...", game.name(), player_name)).await?;

    let (tx, rx) = std::sync::mpsc::channel::<String>();

    // 1. DYNAMIC OS THREAD SPUN UP FOR THIS SPECIFIC SESSION
    tokio::task::spawn_blocking(move || {
        println!("[System] Thread created for {} session (Player: {}).", game.name(), player_name);

        // 2. RAM SHREDDING: Allocate exactly 10 MB chunk instantly
        let target_bytes = 10 * 1024 * 1024;
        let mut _allocated_ram = vec![0u8; target_bytes];
        _allocated_ram[0] = 1; // Force system execution mapping

        // 3. EXECUTE THE DYNAMIC GAME LOGIC
        let game_result = match game.run(&player_name) {
            Ok(result) => result,
            Err(e) => format!("Game Error: {:?}", e),
        };

        // Send the output back to Discord stream
        let _ = tx.send(game_result);

        // 4. AUTOMATIC CLEANUP: Core drops `_allocated_ram` and frees 10MB instantly.
        println!("[System] Session ended. 10MB RAM freed from Xeon matrix.");
    });

    // Wait for the background hardware thread to finish computing
    let final_output = tokio::task::block_in_place(|| rx.recv())?;

    ctx.say(format!("🏆 **[{}] Results:**\n{}", game.name(), final_output)).await?;
    Ok(())
}
