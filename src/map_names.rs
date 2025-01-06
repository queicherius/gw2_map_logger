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
        map_names.insert(951, "Aquatic Ruins Fractal");
        map_names.insert(952, "Cliffside Fractal");
        map_names.insert(948, "Snowblind Fractal");
        map_names.insert(958, "Solid Ocean Fractal");
        map_names.insert(949, "Swampland Fractal");
        map_names.insert(947, "Uncategorized Fractal");
        map_names.insert(953, "Underground Facility Fractal");
        map_names.insert(950, "Urban Battleground Fractal");
        map_names.insert(954, "Volcanic Fractal");
        map_names.insert(956, "Aetherblade Fractal");
        map_names.insert(960, "Captain Mai Trin Boss Fractal");
        map_names.insert(959, "Molten Boss Fractal");
        map_names.insert(955, "Molten Furnace Fractal");
        map_names.insert(957, "Thaumanova Reactor Fractal");
        map_names.insert(1164, "Chaos Fractal");
        map_names.insert(1177, "Nightmare Fractal");
        map_names.insert(1205, "Shattered Observatory Fractal");
        map_names.insert(1267, "Twilight Oasis Fractal");
        map_names.insert(1290, "Deepstone Fractal");
        map_names.insert(1309, "Siren's Reef Fractal");
        map_names.insert(1384, "Sunqua Peak Fractal");
        map_names.insert(1500, "Silent Surf Fractal");
        map_names.insert(1538, "Lonely Tower Fractal");

        MapNames { map_names }
    }

    pub fn get(&self, id: u32) -> String {
        match self.map_names.get(&id) {
            Some(&name) => name.to_string(),
            None => format!("UNKNOWN[{}]", id),
        }
    }
}
