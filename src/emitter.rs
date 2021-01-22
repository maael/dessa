use std::sync::Mutex;
use crate::event_emitter;

// Use lazy_static! because the size of EventEmitter is not known at compile time
lazy_static! {
    // Export the emitter with `pub` keyword
    pub static ref EVENT_EMITTER: Mutex<event_emitter::EventEmitter> = Mutex::new(event_emitter::EventEmitter::new());
}