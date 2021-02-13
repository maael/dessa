use std::thread;
use serde_json::{json};
use ws::{connect, Sender, Handler, Handshake, Result};
use std::sync::{Arc, Mutex};
use crate::emitter::EVENT_EMITTER;
use crate::mumblelink;

#[allow(dead_code)]
struct Client {
  out: Sender,
}

impl Handler for Client {
  fn on_open(&mut self, _: Handshake) -> Result<()> {
    return Ok(());
  }
}

pub fn setup() {
  let connections: Arc<Mutex<Vec<Sender>>> = Arc::new(Mutex::new(Vec::new()));
  let conn1 = connections.clone();
  EVENT_EMITTER.lock().unwrap().on("link", move |data: String| {
    match serde_json::from_str(&data) {
      Ok(result) => {
        let linkmem: mumblelink::LinkedMem = result;
        let msg = json!({
          "source": "dessa",
          "type": "character",
          "data": {
            "charName": linkmem.identity.name,
            "accountName": "Unknown",
            "spec": linkmem.identity.spec,
            "race": linkmem.identity.race,
            "commander": linkmem.identity.commander,
            "player": {
              "x": linkmem.context.player_x,
              "y": linkmem.context.player_y
            },
          }
        }).to_string();
        let current_conns = conn1.lock().unwrap();
        for client in current_conns.iter() {
          client.send(msg.to_string()).unwrap();
        }
      }, Err(e) => {
        log::error!("Error parsing link data for observatory: {}", e);
      }
    }
  });
  thread::spawn(move || {
    connect("ws://dessa-observatory.herokuapp.com/", |out| {
      connections.lock().unwrap().push(out.clone());
      Client { out: out }
    }).unwrap();
  });
}
