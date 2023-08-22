use rdev::{listen, Event};
use simple_websockets::{Event as WsEvent, Message as WsMessage, Responder};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() {
    println!("--- Listening to Input ---");

    // Launching the simple_websockets server on port 14705
    let event_hub = simple_websockets::launch(14705)
        .expect("Failed to listen on port 14705");

    let clients: Arc<Mutex<HashMap<u64, Responder>>> = Arc::new(Mutex::new(HashMap::new()));

    // Separate thread to manage WebSocket connections
    let clients_for_thread = clients.clone();
    std::thread::spawn(move || {
        loop {
            match event_hub.poll_event() {
                WsEvent::Connect(client_id, responder) => {
                    println!("A client connected with id #{}", client_id);
                    clients_for_thread.lock().unwrap().insert(client_id, responder);
                }
                WsEvent::Disconnect(client_id) => {
                    println!("Client #{} disconnected.", client_id);
                    clients_for_thread.lock().unwrap().remove(&client_id);
                }
                WsEvent::Message(client_id, message) => {
                    println!("Received a message from client #{}: {:?}", client_id, message);
                    // If you want to echo the message back or process it, you can do so here.
                }
            }
        }
    });

    // Listen for input events and handle them
    if let Err(error) = listen(move |event| callback(event, &clients)) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event, clients: &Arc<Mutex<HashMap<u64, Responder>>>) {
    // Checking if the event has a name.
    if let Some(string) = event.name {
        // Lock the clients hashmap just once to iterate over it.
        let locked_clients = clients.lock().unwrap();

        // Sending the received key event to all connected WebSocket clients.
        for (client_id, responder) in locked_clients.iter() {
            if !responder.send(WsMessage::Text(string.clone())) {
                eprintln!("Failed to send message to client #{}", client_id);
            }
        }
    }
}
