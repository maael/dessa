use crate::link::GW2;
use std::{thread};

pub fn setup() {
  thread::spawn(|| {
  let mut gw2 = GW2::new().expect("Unable to link to Guild Wars 2");
    
  loop {
      if let Some(link) = gw2.tick() {
          // Do something with the current `LinkedMem` instance
          // This gets spammed a lot
          // Strings don't seem to come out right
          if link.ui_tick() % 100 == 0 {
            log::info!("Mumble Link: {:?} {:?} {:?} {:?}", link.ui_tick(), link.name(), link.identity(), link.context().map_id);
          }
      }
  }
  });
}