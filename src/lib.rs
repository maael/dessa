mod logging;
mod websockets;
mod arcdps;
#[cfg(windows)]
mod link;
mod mumblelink;
mod executor;
mod exports;
mod emitter;
mod event_emitter;
mod fractal_level;
mod discord_presence;
mod observatory_agent;

#[macro_use]
extern crate lazy_static;

use std::sync::mpsc::{channel};
use winapi::shared::minwindef::LPVOID;

fn main() -> LPVOID {
    let (tx, rx) = channel::<String>();
    executor::setup();
    logging::setup();
    websockets::setup(rx);
    fractal_level::setup();
    mumblelink::setup(tx.clone());
    discord_presence::setup();
    observatory_agent::setup();
    arcdps::gen_arcdps(tx.clone()) // There is no semi colon here on purpose
}

fn release() {
    // Release/teardown here
    discord_presence::teardown();
    executor::teardown();
}
