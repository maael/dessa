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

#[macro_use]
extern crate lazy_static;

use std::sync::mpsc::{channel};
use winapi::shared::minwindef::LPVOID;

fn main() -> LPVOID {
    let (tx, rx) = channel::<String>();
    executor::setup();
    logging::setup();
    websockets::setup(rx);
    mumblelink::setup(tx.clone());
    arcdps::gen_arcdps(tx.clone()) // There is no semi colon here on purpose
}

fn release() {
    // Release/teardown here
    executor::teardown();
}
