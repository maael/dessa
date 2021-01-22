use crate::link::GW2;
use std::{thread};
use std::sync::mpsc::{Sender};
use serde_json::json;

pub fn setup(tx: Sender<String>) {
  thread::spawn(move || {
  let mut gw2 = GW2::new().expect("Unable to link to Guild Wars 2");
    
  loop {
      if let Some(link) = gw2.tick() {
          // Do something with the current `LinkedMem` instance
          // This gets spammed a lot
          // Strings don't seem to come out right
          if link.ui_tick() % 100 == 0 {
            tx.send(json!({
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
              "identity": link.identity(),
              "context_len": link.context_len(),
              "context": {
                "map_id": link.context().map_id
              },
              "description": link.description(),
            }).to_string()).unwrap();
            log::debug!("Mumble Link: {:?} {:?} {:?} {:?}", link.ui_tick(), link.name(), link.identity(), link.context().map_id);
          }
      }
  }
  });
}