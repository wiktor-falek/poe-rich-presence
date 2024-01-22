// use discord_rich_presence::activity::Button;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use notify::{RecursiveMode, Watcher};
use std::path::Path;
use std::{thread, time};

struct Character<'a> {
    league: &'a str,
    ascendancy: &'a str,
    level: u8,
    location: &'a str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("1198727396296241202")?;
    client.connect()?;

    let mut watcher = notify::recommended_watcher(|res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })?;

    let poe_log_file_path =
        Path::new("C:/Program Files (x86)/Grinding Gear Games/Path of Exile/logs/Client.txt");
    watcher.watch(poe_log_file_path, RecursiveMode::NonRecursive)?;

    println!(
        "Watching for changes in {:?}",
        poe_log_file_path.to_str().unwrap()
    );

    let app_start_unix_timestamp = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)?
        .as_secs();

    let mut last_iteration = time::Instant::now();

    loop {
        let character = Character {
            league: "Affliction",
            ascendancy: "Occultist",
            level: 100,
            location: "Cartographer's Hideout",
        };
        // TODO: listen to log file changes and update the location

        let details: &str =
            &(format!("Lvl {} {}", character.level, character.ascendancy).to_string());

        let timestamps = activity::Timestamps::new().start(app_start_unix_timestamp as i64);

        let buttons = vec![
            // activity::Button::new(
            // "Profile",
            // "https://www.pathofexile.com/account/view-profile/EGBlade",
            // )
        ];

        // TODO: default to "Character Select" if no character is present
        let large_text = format!(
            "Lvl {} {} in {} League",
            character.level, character.ascendancy, character.league
        );

        let assets = activity::Assets::new()
            // TODO: map ascendancy to image, default to "path-of-exile-logo"
            .large_image("ascendancy_occultist")
            .large_text(large_text.as_str());

        client.set_activity(
            activity::Activity::new()
                .state(character.location)
                .details(details)
                .timestamps(timestamps)
                .buttons(buttons)
                .assets(assets),
        )?;

        thread::sleep(time::Duration::from_secs(1) - last_iteration.elapsed());
        last_iteration = time::Instant::now()
    }
}
