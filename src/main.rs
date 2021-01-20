extern crate env_logger;
/// Simple WebSocket server with error handling. It is not necessary to setup logging, but doing
/// so will allow you to see more details about the connection by using the RUST_LOG env variable.
extern crate ws;

use std::cell::Cell;
use std::rc::Rc;
use std::{thread, time::Duration};
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};
use serde_json::json;

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
        let out = self.out.clone();

        self.clients.set(self.clients.get() + 1);

        thread::spawn(move || loop {
            let data = json!({
                "name": "Matt",
                "age": 26
            });

            out.send(data.to_string()).unwrap();
            thread::sleep(Duration::from_millis(1000));
            // TODO: Figure out how to stop this thread when websocket closes
        });

        return self.out.send("{\"start\": 1}");
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Got message: {}", msg);
        // self.out.close(CloseCode::Normal)
        return self.out.send(msg);
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!(
                "The client {} is done with the connection.",
                self.out.connection_id()
            ),
            CloseCode::Away => println!(
                "The client {} is leaving the site.",
                self.out.connection_id()
            ),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client {}.",
                self.out.connection_id()
            ),
            _ => println!(
                "The client {} encountered an error: {}",
                self.out.connection_id(),
                reason
            ),
        }
        self.clients.set(self.clients.get() - 1);
    }
}

fn main() {
    env_logger::init();
    let held_clients = Rc::new(Cell::new(0));

    // TODO: Client is called per incoming, so shouldn't need to do Rc/Cell
    // TODO: Can just handle DC inside each client per client
    if let Err(error) = listen("127.0.0.1:3012", |out| Client {
        out: out,
        clients: held_clients.clone(),
    }) {
        println!("Failed to create WebSocket due to {:?}", error);
    }
}
