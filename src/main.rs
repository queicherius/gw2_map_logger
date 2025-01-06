use chrono::Local;
use gw2_mumble::MumbleLink;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let link = MumbleLink::new()?;

    let mut last_map_id = None;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("changes.txt")?;

    println!("Started monitoring map changes...");

    loop {
        if let Ok(identity) = link.parse_identity() {
            let current_map_id = identity.map_id;

            if last_map_id != Some(current_map_id) {
                let timestamp = Local::now().timestamp_millis();
                let log_entry = format!("{},{}\n", timestamp, current_map_id);

                file.write_all(log_entry.as_bytes())?;
                file.flush()?;

                println!("Map change detected: {}", log_entry.trim());
                last_map_id = Some(current_map_id);
            }
        } else {
            println!("Failed to parse identity data");
        }

        thread::sleep(Duration::from_millis(200));
    }
}
