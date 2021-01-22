extern crate ws;

use log;
use std::cell::Cell;
use std::rc::Rc;
use std::{thread};
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};
use std::sync::mpsc::{Receiver};
use std::sync::{Arc, Mutex};
use crate::emitter::EVENT_EMITTER;

// Our Handler struct.
// Here we explicitly indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
    clients: Rc<Cell<u32>>,
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.clients.set(self.clients.get() + 1);
        return self.out.send("{\"start\": 1}");
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        log::info!("Got message: {}", msg);
        // self.out.close(CloseCode::Normal)
        return self.out.send(msg);
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => log::info!(
                "The client {} is done with the connection.",
                self.out.connection_id()
            ),
            CloseCode::Away => log::info!(
                "The client {} is leaving the site.",
                self.out.connection_id()
            ),
            CloseCode::Abnormal => log::error!(
                "Closing handshake failed! Unable to obtain closing status from client {}.",
                self.out.connection_id()
            ),
            _ => log::info!(
                "The client {} encountered an error: {}",
                self.out.connection_id(),
                reason
            ),
        }
        self.clients.set(self.clients.get() - 1);
    }
}

pub fn setup(ws_rx: Receiver<String>) {
    let connections: Arc<Mutex<Vec<Sender>>> = Arc::new(Mutex::new(Vec::new()));
    let conn1 = connections.clone();
    let conn2 = connections.clone();
    EVENT_EMITTER.lock().unwrap().on("arc", move |data: String| {
        let current_conns = conn1.lock().unwrap();
        log::info!("Sending {} to {} clients", data, current_conns.len());
        for client in current_conns.iter() {
            client.send(data.to_string()).unwrap();
        }
    });
    thread::spawn(move || {
        let held_clients = Rc::new(Cell::new(0));
        thread::spawn(move || {
            loop {
                let new_message = ws_rx.recv().unwrap();
                let current_conns = conn2.lock().unwrap();
                log::info!("Sending {} to {} clients", new_message, current_conns.len());
                for client in current_conns.iter() {
                    client.send(new_message.to_string()).unwrap();
                }
            }
        });

        // TODO: Client is called per incoming, so shouldn't need to do Rc/Cell
        // TODO: Can just handle DC inside each client per client
        if let Err(error) = listen("127.0.0.1:3012", |out| {
                connections.lock().unwrap().push(out.clone());
                log::info!("Sending new client {}", out.connection_id());
                Client {
                    out: out,
                    clients: held_clients.clone(),
                }
            }
        ) {
            log::error!("Failed to create WebSocket due to {:?}", error);
        }
    });
}
