use crate::emitter::EVENT_EMITTER;
use crate::mumblelink;
use rustcord::{EventHandlers, RichPresenceBuilder, Rustcord, User};
use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub struct Handlers;

impl EventHandlers for Handlers {
  fn ready(user: User) {
    log::info!("User {}#{} logged in...", user.username, user.discriminator);
  }
}

#[derive(Serialize, Deserialize)]
pub struct ApiMapData {
  id: u16,
  name: String,
  default_floor: u16,
  region_id: u16,
  continent_id: u16
}

#[derive(Serialize, Deserialize)]
pub struct ApiContinentData {
  name: String,
  sectors: HashMap<String, ApiContinentSectorData>
}

#[derive(Serialize, Deserialize)]
pub struct ApiContinentSectorData {
  name: String
}

#[derive(Serialize, Deserialize)]
pub struct ApiSpecData {
  name: String,
  profession: String,
  elite: bool,
}

fn get_race_map () -> HashMap<u16, String> {
  let mut m: HashMap<u16, String> = HashMap::new();
  m.insert(0, "Asuran".to_owned());
  m.insert(1, "Charr".to_owned());
  m.insert(2, "Human".to_owned());
  m.insert(3, "Norn".to_owned());
  m.insert(4, "Sylvari".to_owned());
  return m;
}

pub fn setup() {
  thread::spawn(|| {
    let client = reqwest::blocking::Client::new();
    let discord_client_id = option_env!("DISCORD_CLIENT_ID").unwrap_or("");
    log::debug!("Client ID: {}", discord_client_id);
    let discord = loop {
      let init_discord = Rustcord::init::<Handlers>(discord_client_id, true, None);
      if init_discord.is_ok() {
        break init_discord.unwrap();
      }
    };
    let locations_hash: Arc<Mutex<HashMap<u16, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let locations_hash_clone = locations_hash.clone();
    let spec_hash: Arc<Mutex<HashMap<u16, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let spec_hash_clone = spec_hash.clone();
    let init_time = SystemTime::now();

    // Keeping this here incase we ever want it
    #[allow(unused_variables)]
    let race_map = get_race_map();

    EVENT_EMITTER
      .lock()
      .unwrap()
      .on("link", move |data: String| {
        let mut current_locations_hash = locations_hash_clone.lock().unwrap();
        let mut current_spec_hash = spec_hash_clone.lock().unwrap();
        match serde_json::from_str(&data) {
          Ok(result) => {
            let linkmem: mumblelink::LinkedMem = result;
            if !current_locations_hash.contains_key(&linkmem.context.map_id) {
              log::info!("Getting location details for {}", linkmem.context.map_id);
              let res = client.get(reqwest::Url::parse(&format!("https://api.guildwars2.com/v2/maps/{}", &linkmem.context.map_id)).unwrap()).send().unwrap();
              let map_data_text = res.text().unwrap();
              let map_data_json: ApiMapData = serde_json::from_str(&map_data_text).unwrap();
              log::info!("Got location details for {} as {}", linkmem.context.map_id, map_data_json.name);
              let name = map_data_json.name.clone();
              current_locations_hash.insert(linkmem.context.map_id, map_data_json.name);
              if name == "Fractals of the Mists" {
                log::info!("Getting expanded Fractal name {}, {}, {}, {}",  map_data_json.continent_id, map_data_json.default_floor, map_data_json.region_id, map_data_json.id);
                let fotm_res = client.get(reqwest::Url::parse(&format!("https://api.guildwars2.com/v2/continents/{}/floors/{}/regions/{}/maps/{}", map_data_json.continent_id, map_data_json.default_floor, map_data_json.region_id, map_data_json.id)).unwrap()).send().unwrap();
                let fotm_map_data_text = fotm_res.text().unwrap();
                let fotm_map_data_json: ApiContinentData = serde_json::from_str(&fotm_map_data_text).unwrap();
                let sector_name = fotm_map_data_json.sectors.values().next().unwrap();
                log::info!("Got fractal location details for {} as {}", linkmem.context.map_id, sector_name.name);
                current_locations_hash.insert(linkmem.context.map_id, sector_name.name.to_owned());
              }
            }
            if !current_spec_hash.contains_key(&linkmem.identity.spec) {
              log::info!("Getting spec details for {}", linkmem.identity.spec);
              let res = client.get(reqwest::Url::parse(&format!("https://api.guildwars2.com/v2/specializations/{}", &linkmem.identity.spec)).unwrap()).send().unwrap();
              let spec_data_text = res.text().unwrap();
              let spec_data_json: ApiSpecData = serde_json::from_str(&spec_data_text).unwrap();
              let spec_name = if spec_data_json.elite == true { spec_data_json.name } else { spec_data_json.profession };
              log::info!("Got spec details for {} as {}", linkmem.identity.spec, spec_name);
              current_spec_hash.insert(linkmem.identity.spec, spec_name);
            }
            let race_with_spec = vec![ /*race_map.get(&linkmem.identity.race).unwrap_or(&"".to_owned()).to_string(),*/ current_spec_hash.get(&linkmem.identity.spec).unwrap_or(&"Unknown Profession".to_owned()).to_string()];

            let mut large_image_key = format!("{}", current_locations_hash.get(&linkmem.context.map_id).unwrap_or(&"gw2".to_owned())).to_string();
            if large_image_key != "gw2" {
              large_image_key = str::replace(&str::replace(&format!("loading-screen--{}", &large_image_key.to_lowercase()), "'", "_"), " ", "_").to_lowercase();
            }

            let small_image_key = current_spec_hash.get(&linkmem.identity.spec).unwrap_or(&"gw2".to_owned()).to_string().to_lowercase();

            let presence = RichPresenceBuilder::new()
              .details(&format!("{} ({})", linkmem.identity.name, race_with_spec.join(" ")).to_string())
              .state(&format!("{}", current_locations_hash.get(&linkmem.context.map_id).unwrap_or(&"Unknown Location".to_owned())).to_string())
              .large_image_key(&large_image_key)
              .large_image_text(&format!("Exploring {} in Guild Wars 2", current_locations_hash.get(&linkmem.context.map_id).unwrap_or(&"Unknown Location".to_owned())).to_string())
              .small_image_key(&small_image_key)
              .small_image_text(&current_spec_hash.get(&linkmem.identity.spec).unwrap_or(&"Unknown Profession".to_owned()).to_string())
              .start_time(init_time)
              .build();

            match discord.update_presence(presence) {
              Ok(_) => discord.run_callbacks(),
              Err(e) => log::debug!("Error updating presence: {}", e)
            }
          },
          Err(e) => log::error!("Couldn't deserialize json: {}", e)
        };
      });
  });
}

pub fn teardown() {}
