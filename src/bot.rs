// Declare the local sub-directories relative to this root target
pub mod commands;
pub mod loadcmds;
pub mod state;
pub mod games;

use std::env;
use state::BotState;

// High-performance thread-safe memory manager for Threadripper/Xeon core architectures
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    // Build your high-capacity Thread/RAM runtime pool
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(num_cpus::get()) // Spawns execution tracks matching your core limits
        .thread_name("discord-bot-worker")
        .build()
        .expect("Failed to initialize multi-threaded runtime");

    runtime.block_on(async_main());
}

async fn async_main() {
    // Looks directly for your .env located in the base rust-bot/ folder
    if let Err(e) = dotenvy::dotenv() {
        println!("[Warning] No .env file found or failed to parse: {}. Falling back to system env.", e);
    }

    let token = env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN environmental variable. Ensure it is defined in your .env file.");

    let custom_state = BotState::new();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: loadcmds::get_all_commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(custom_state)
            })
        })
        .build();

    let intents = serenity::all::GatewayIntents::non_privileged()
        | serenity::all::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::all::Client::builder(&token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();
}
