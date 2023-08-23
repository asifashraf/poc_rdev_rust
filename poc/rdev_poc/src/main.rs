use rdev::{listen, Event};
use simple_websockets::{Event as WsEvent, Message as WsMessage, Responder};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use chrono::{Local, Utc};
use uuid::Uuid;
use serde_json::json;

fn main() {
    println!("--- Listening to Input ---");

    let event_hub = simple_websockets::launch(14705)
        .expect("Failed to listen on port 14705");

    let clients: Arc<Mutex<HashMap<u64, Responder>>> = Arc::new(Mutex::new(HashMap::new()));
    let validated_clients: Arc<Mutex<HashSet<u64>>> = Arc::new(Mutex::new(HashSet::new()));

    let clients_for_thread = clients.clone();
    let validated_clients_for_thread = validated_clients.clone();

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
                    validated_clients_for_thread.lock().unwrap().remove(&client_id);
                }
                WsEvent::Message(in_client_id, message) => {
                    println!("Received a message from client #{}: {:?}", in_client_id, message);
                    const AUTH_CODE: &str = "ABC12356753422DFREDSJJHHY";

                    match message {
                        WsMessage::Text(text_message) => {
                            if AUTH_CODE == text_message {
                                println!("Client #{} is validated: {:?}", in_client_id, text_message);
                                validated_clients_for_thread.lock().unwrap().insert(in_client_id);
                            }
                        }
                        _ => {
                            // Handle other message types or ignore them
                        }
                    }
                }
            }
        }
    });

    if let Err(error) = listen(move |event| callback(event, &clients, &validated_clients)) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event, clients: &Arc<Mutex<HashMap<u64, Responder>>>, validated_clients: &Arc<Mutex<HashSet<u64>>>) {
    if event.name.is_some() {
        let local_timestamp = Local::now().to_rfc3339();
        let utc_timestamp = Utc::now().to_rfc3339();
        let id = Uuid::new_v4();
        let event_type_str = format!("{:?}", event.event_type);

        let data = json!({
            "event": event.name,
            "event_time": event.time,
            "event_type": event_type_str,
            "timestamp_local": local_timestamp,
            "timestamp_utc": utc_timestamp,
            "uuid": id.to_string(),
        });

        let message = data.to_string();

        let locked_clients = clients.lock().unwrap();
        let locked_validated_clients = validated_clients.lock().unwrap();

        for (client_id, responder) in locked_clients.iter() {
            if locked_validated_clients.contains(client_id) {
                if !responder.send(WsMessage::Text(message.clone())) {
                    eprintln!("Failed to send message to client #{}", client_id);
                }
            }
        }
    }
}
