mod logging;
mod websockets;
mod arcdps;
#[cfg(windows)]
mod link;
mod mumblelink;
mod executor;
mod exports;

use winapi::shared::minwindef::LPVOID;

fn main() -> LPVOID {
    executor::setup();
    logging::setup();
    websockets::setup();
    mumblelink::setup();
    arcdps::gen_arcdps() // There is no semi colon here on purpose
}

fn release() {
    // Release/teardown here
    executor::teardown();
}
