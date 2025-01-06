use chrono::Utc;
use gw2_mumble::MumbleLink;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

mod map_names;
use map_names::MapNames;

fn format_duration(duration_ms: i64) -> String {
    let duration_secs = duration_ms / 1000;
    let minutes = duration_secs / 60;
    let seconds = duration_secs % 60;

    format!("{:02}:{:02}", minutes, seconds)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let link = MumbleLink::new()?;
    let map_names = MapNames::new();
    let mut last_map_id = None;
    let mut last_change_time = Utc::now().timestamp_millis();
    let mut last_tick = 0u32;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("changes.tsv")?;

    println!("Started monitoring map changes...");

    loop {
        let current_tick = link.read_ui_tick();
        let current_map_id = if current_tick == last_tick {
            0 // Loading screen
        } else if let Ok(identity) = link.parse_identity() {
            identity.map_id
        } else {
            println!("Failed to get current map id");
            continue;
        };

        if last_map_id != Some(current_map_id) {
            let current_time = Utc::now().timestamp_millis();

            if last_map_id.is_some() {
                let current_time_fmt = Utc::now().format("%Y-%m-%d %H:%M:%S");
                let map_name = map_names.get(last_map_id.unwrap());
                let duration_ms = current_time - last_change_time;

                let log_entry = format!(
                    "{}\t{}\t{}\n",
                    current_time_fmt,
                    map_name,
                    format_duration(duration_ms)
                );

                if duration_ms > 500 {
                    println!("{}", log_entry.trim());

                    file.write_all(log_entry.as_bytes())?;
                    file.flush()?;
                }
            }

            last_map_id = Some(current_map_id);
            last_change_time = current_time;
        }

        last_tick = current_tick;
        thread::sleep(Duration::from_millis(100));
    }
}
