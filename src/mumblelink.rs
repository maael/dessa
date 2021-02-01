use crate::link::{GW2, GW2Identity};
use std::{thread};
use std::sync::mpsc::{Sender};
use serde_json::json;
use crate::emitter::EVENT_EMITTER;

pub fn setup(tx: Sender<String>) {
  thread::spawn(move || {
  let mut gw2 = GW2::new().expect("Unable to link to Guild Wars 2");
    
  loop {
      if let Some(link) = gw2.tick() {
          // Do something with the current `LinkedMem` instance
          // This gets spammed a lot
          // Strings don't seem to come out right
          if link.ui_tick() % 100 == 0 {
            let identity: GW2Identity = serde_json::from_str(&link.identity().to_string()).unwrap();
            let json_s = json!({
              "type": "link",
              "ui_version": link.ui_version(),
              "ui_tick": link.ui_tick(),
              "avatar": {
                "front": link.avatar().front,
                "position": link.avatar().position,
                "top": link.avatar().top
              },
              "name": link.name(),
              "camera": {
                "front": link.camera().front,
                "position": link.camera().position,
                "top": link.camera().top
              },
              "identity": identity,
              "context_len": link.context_len(),
              "context": {
                "server_address": link.context().server_address,
                "map_id": link.context().map_id,
                "map_type": link.context().map_type,
                "shard_id": link.context().shard_id,
                "instance": link.context().instance,
                "build_id": link.context().build_id,
                "ui_state": link.context().ui_state,
                "compass_width": link.context().compass_width,
                "compass_height": link.context().compass_height,
                "compass_rotation": link.context().compass_rotation,
                "player_x": link.context().player_x,
                "player_y": link.context().player_y,
                "map_center_x": link.context().map_center_x,
                "map_center_y": link.context().map_center_y,
                "map_scale": link.context().map_scale,
                "process_id": link.context().process_id,
                "mount_index": link.context().mount_index,
              },
              "description": link.description(),
            });
            EVENT_EMITTER.lock().unwrap().emit("link", json_s.to_string());
            tx.send(json_s.to_string()).unwrap();
            log::debug!("Mumble Link: {:?} {:?} {:?} {:?}", link.ui_tick(), link.name(), link.identity(), link.context().map_id);
          }
      }
  }
  });
}