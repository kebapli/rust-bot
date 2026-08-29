# Rust Discord Bot

Hello guys, this discord bot is a rust template thats capable of doing moderation, fun commands and verifying!
All you gotta do is either fork this, download it or use `git clone https://github.com/kebapli/rust-bot` and move on to how to run it section

## WARNING:

This bot uses threading sharding so this was designed to run on cpus like **AMD Threadripper**, **AMD EPYC**, **Intel Xeon** or similar cpus with high threads. Low threads can still work but if the active session of fun commands passes the amount of threads it might crash, stop or cause lag.

## How to run it

To run it, first you need to fork or download it. Then in .env file, find the DISCORD_TOKEN and paste your token inside that. Then use `cargo run` or `cargo build`. And it should work. If you find any bugs please create an issue!
