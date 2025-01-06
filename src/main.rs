use chrono::Local;
use csv::ReaderBuilder;
use gw2_mumble::MumbleLink;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn load_map_names() -> Result<HashMap<u32, String>, Box<dyn std::error::Error>> {
    let mut map_names = HashMap::new();
    let mut rdr = ReaderBuilder::new().from_path("map_names.csv")?;

    for result in rdr.records() {
        let record = result?;
        if let (Some(id_str), Some(name)) = (record.get(0), record.get(1)) {
            if let Ok(id) = id_str.parse::<u32>() {
                map_names.insert(id, name.to_string());
            }
        }
    }

    Ok(map_names)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let link = MumbleLink::new()?;
    let map_names = load_map_names()?;
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

                let map_name_fallback = format!("UNKNOWN[{}]", current_map_id);

                let map_name = map_names
                    .get(&current_map_id)
                    .map(|name| name.as_str())
                    .unwrap_or(&map_name_fallback);

                let log_entry = format!("{},{}\n", timestamp, map_name);

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
