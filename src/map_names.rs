use std::collections::HashMap;

pub struct MapNames {
    map_names: HashMap<u32, &'static str>,
}

impl MapNames {
    pub fn new() -> Self {
        let mut map_names = HashMap::new();

        // Loading Screen
        map_names.insert(0, "Loading Screen");

        // Lobbies
        map_names.insert(1206, "Mistlock Sanctuary");
        map_names.insert(872, "Mistlock Observatory");

        // Fractals
        map_names.insert(950, "Urban Battleground");
        map_names.insert(954, "Volcanic");
        map_names.insert(956, "Aetherblade");

        MapNames { map_names }
    }

    pub fn get(&self, id: u32) -> String {
        match self.map_names.get(&id) {
            Some(&name) => name.to_string(),
            None => format!("UNKNOWN[{}]", id),
        }
    }
}
