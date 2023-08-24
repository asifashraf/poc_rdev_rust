use chrono::{Local, Utc};
use cli_clipboard;
use enigo::{Enigo, Key as EnigoKey, KeyboardControllable};
use rdev::{listen, Event};
use rdev::{simulate, EventType, Key, SimulateError};
use rdev::{Keyboard, KeyboardState};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use simple_websockets::{Event as WsEvent, Message as WsMessage, Responder};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::{thread, time};
use uuid::Uuid;

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

fn main() {
    let mut keyboard = Keyboard::new().unwrap();

    let keys_map = Arc::new(Mutex::new(get_keys_map()));
    // Clone the Arc to share with threads
    let keys_map_thread = Arc::clone(&keys_map);

    println!("--- Listening to Input ---");

    let event_hub = simple_websockets::launch(14705).expect("Failed to listen on port 14705");

    let clients: Arc<Mutex<HashMap<u64, Responder>>> = Arc::new(Mutex::new(HashMap::new()));
    let validated_clients: Arc<Mutex<HashSet<u64>>> = Arc::new(Mutex::new(HashSet::new()));

    let clients_for_thread = clients.clone();
    let validated_clients_for_thread = validated_clients.clone();

    // hash map by character, store Key and bool
    let mut keys_map = get_keys_map();

    std::thread::spawn(move || {
        loop {
            match event_hub.poll_event() {
                WsEvent::Connect(client_id, responder) => {
                    clients_for_thread
                        .lock()
                        .unwrap()
                        .insert(client_id, responder);
                }
                WsEvent::Disconnect(client_id) => {
                    println!("Client #{} disconnected.", client_id);
                    clients_for_thread.lock().unwrap().remove(&client_id);
                    validated_clients_for_thread
                        .lock()
                        .unwrap()
                        .remove(&client_id);
                }
                WsEvent::Message(in_client_id, message) => {
                    const AUTH_CODE: &str = "EdbKsUzjFHYNRmTAWqGClcBXgrZivLQhJoMItSbwEPaDnxOpfVuyXerHPksLOhBvXeUfzaCwIyRGtQJmNVblMnsjZdYKFrcPoAigXuhZWq";
                    //println!("--- Listening to Input --- {:?}", message);
                    if let WsMessage::Text(text_message) = message {
                        match serde_json::from_str::<HashMap<String, Value>>(&text_message) {
                            Ok(map) => {
                                if let Some(msg_type) = map.get("type") {
                                    if let Some(serde_json::Value::String(data_string)) =
                                        map.get("data")
                                    {
                                        if msg_type == "auth" {
                                            if data_string == AUTH_CODE {
                                                println!("Client #{} connected", in_client_id);
                                                validated_clients_for_thread
                                                    .lock()
                                                    .unwrap()
                                                    .insert(in_client_id);
                                            }
                                        }

                                        if msg_type == "write_sequence" {
                                            let mut enigo = Enigo::new();
                                            enigo.key_sequence(data_string);
                                        }

                                        if msg_type == "paste_text_via_clipboard" {
                                            cli_clipboard::set_contents(data_string.clone())
                                                .unwrap();
                                            // Paste
                                            // Simulate the key combination for "Paste"
                                            #[cfg(target_os = "macos")]
                                            {
                                                // macOS uses the Command key (⌘) for shortcuts
                                                send(&EventType::KeyPress(Key::MetaLeft));
                                                send(&EventType::KeyPress(Key::KeyV));
                                                send(&EventType::KeyRelease(Key::KeyV));
                                                send(&EventType::KeyRelease(Key::MetaLeft));
                                                //-- with delays
                                                // send(&EventType::KeyPress(Key::MetaLeft));
                                                // sleep(Duration::from_millis(10));

                                                // send(&EventType::KeyPress(Key::KeyV));
                                                // sleep(Duration::from_millis(10));

                                                // send(&EventType::KeyRelease(Key::KeyV));
                                                // sleep(Duration::from_millis(10));

                                                // send(&EventType::KeyRelease(Key::MetaLeft));
                                            }

                                            #[cfg(all(
                                                not(target_os = "macos"),
                                                any(target_os = "linux", target_os = "windows")
                                            ))]
                                            {
                                                // Windows and Linux use the Ctrl key for shortcuts
                                                send(&EventType::KeyPress(Key::ControlLeft));
                                                send(&EventType::KeyPress(Key::KeyV));
                                                send(&EventType::KeyRelease(Key::KeyV));
                                                send(&EventType::KeyRelease(Key::ControlLeft));
                                            }
                                        }

                                        if msg_type == "set_text_in_clipboard" {
                                            cli_clipboard::set_contents(data_string.clone())
                                                .unwrap();
                                        }

                                        if msg_type == "type_characters_one_by_one" {
                                            let mut enigo = Enigo::new();
                                            let keys_map = keys_map_thread.lock().unwrap();

                                            // Loop over the characters from data_string
                                            for c in data_string.chars() {
                                                let key_from_map = keys_map.get(&c);
                                                if let Some((key, is_shifted)) = key_from_map {
                                                    if key == &Key::Unknown(0) {
                                                        println!(
                                                            "Key not found: {:?}, is_shifted: {}",
                                                            key, is_shifted
                                                        );
                                                        enigo.key_click(EnigoKey::Layout(c));
                                                    } else {
                                                        // Key was found in the map

                                                        if *is_shifted {
                                                            send(&EventType::KeyPress(
                                                                Key::ShiftLeft,
                                                            )); // Simulating Shift key press
                                                        }
                                                        send(&EventType::KeyPress(key.clone())); // Simulating the key press
                                                        if *is_shifted {
                                                            send(&EventType::KeyRelease(
                                                                Key::ShiftLeft,
                                                            )); // Simulating Shift key release
                                                        }
                                                    }
                                                } else {
                                                    println!(
                                                        "2. Key not found: {:?}", c
                                                    );
                                                    //enigo.key_down(EnigoKey::Layout(c));
                                                    //enigo.key_up(EnigoKey::Layout(c));
                                                    enigo.key_sequence(&c.to_string());
                                                }
                                            }

                                            // Loop over the characters from data_string
                                            // for c in data_string.chars() {
                                            //     // print character
                                            //     println!("typing char: {}", c);
                                            //     if let Some(key) = char_to_key(c) {
                                            //         // Type each character using rdev
                                            //         send(&EventType::KeyPress(key));
                                            //     }
                                            // }

                                            // const sentence: &str =
                                            //     "abcABC123!@#$%^&*()_+-={}[]\";\\;':\",./<>?`~";
                                            // for c in data_string.chars() {
                                            //     let keys_map = keys_map_thread.lock().unwrap();
                                            //     let key_from_map = keys_map.get(&c);
                                            //     if let Some((key, is_shifted)) = key_from_map {
                                            //         println!("Character => {:?}", key);
                                            //         if key == &Key::Unknown(0) {
                                            //             // Key not found in the map
                                            //             println!("Key not found");
                                            //         } else {
                                            //             // Key was found in the map
                                            //             println!(
                                            //                 "Key found: {:?}, is_shifted: {}",
                                            //                 key, is_shifted
                                            //             );
                                            //             if *is_shifted {
                                            //                 send(&EventType::KeyPress(
                                            //                     Key::ShiftLeft,
                                            //                 )); // Simulating Shift key press
                                            //             }
                                            //             send(&EventType::KeyPress(key.clone())); // Simulating the key press
                                            //             if *is_shifted {
                                            //                 send(&EventType::KeyRelease(
                                            //                     Key::ShiftLeft,
                                            //                 )); // Simulating Shift key release
                                            //             }
                                            //         }
                                            //     }
                                            // }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error deserializing data: {:?}", e);
                            }
                        }
                    }
                }
            }
        }
    });

    // Note: I'm assuming you have a function called `listen` and `callback`.
    if let Err(error) = listen(move |event| callback(event, &clients, &validated_clients)) {
        println!("Error: {:?}", error);
    }
}

fn callback(
    event: Event,
    clients: &Arc<Mutex<HashMap<u64, Responder>>>,
    validated_clients: &Arc<Mutex<HashSet<u64>>>,
) {
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

fn get_keys_map() -> HashMap<char, (Key, bool)> {
    let mut my_hashmap = HashMap::new();
    
    my_hashmap.insert('a', (Key::KeyA, false));
    my_hashmap.insert('b', (Key::KeyB, false));
    my_hashmap.insert('c', (Key::KeyC, false));
    my_hashmap.insert('d', (Key::KeyD, false));
    my_hashmap.insert('e', (Key::KeyE, false));
    my_hashmap.insert('f', (Key::KeyF, false));
    my_hashmap.insert('g', (Key::KeyG, false));
    my_hashmap.insert('h', (Key::KeyH, false));
    my_hashmap.insert('i', (Key::KeyI, false));
    my_hashmap.insert('j', (Key::KeyJ, false));
    my_hashmap.insert('k', (Key::KeyK, false));
    my_hashmap.insert('l', (Key::KeyL, false));
    my_hashmap.insert('m', (Key::KeyM, false));
    my_hashmap.insert('n', (Key::KeyN, false));
    my_hashmap.insert('o', (Key::KeyO, false));
    my_hashmap.insert('p', (Key::KeyP, false));
    my_hashmap.insert('q', (Key::KeyQ, false));
    my_hashmap.insert('r', (Key::KeyR, false));
    my_hashmap.insert('s', (Key::KeyS, false));
    my_hashmap.insert('t', (Key::KeyT, false));
    my_hashmap.insert('u', (Key::KeyU, false));
    my_hashmap.insert('v', (Key::KeyV, false));
    my_hashmap.insert('w', (Key::KeyW, false));
    my_hashmap.insert('x', (Key::KeyX, false));
    my_hashmap.insert('y', (Key::KeyY, false));
    my_hashmap.insert('z', (Key::KeyZ, false));

    my_hashmap.insert('A', (Key::KeyA, true));
    my_hashmap.insert('B', (Key::KeyB, true));
    my_hashmap.insert('C', (Key::KeyC, true));
    my_hashmap.insert('D', (Key::KeyD, true));
    my_hashmap.insert('E', (Key::KeyE, true));
    my_hashmap.insert('F', (Key::KeyF, true));
    my_hashmap.insert('G', (Key::KeyG, true));
    my_hashmap.insert('H', (Key::KeyH, true));
    my_hashmap.insert('I', (Key::KeyI, true));
    my_hashmap.insert('J', (Key::KeyJ, true));
    my_hashmap.insert('K', (Key::KeyK, true));
    my_hashmap.insert('L', (Key::KeyL, true));
    my_hashmap.insert('M', (Key::KeyM, true));
    my_hashmap.insert('N', (Key::KeyN, true));
    my_hashmap.insert('O', (Key::KeyO, true));
    my_hashmap.insert('P', (Key::KeyP, true));
    my_hashmap.insert('Q', (Key::KeyQ, true));
    my_hashmap.insert('R', (Key::KeyR, true));
    my_hashmap.insert('S', (Key::KeyS, true));
    my_hashmap.insert('T', (Key::KeyT, true));
    my_hashmap.insert('U', (Key::KeyU, true));
    my_hashmap.insert('V', (Key::KeyV, true));
    my_hashmap.insert('W', (Key::KeyW, true));
    my_hashmap.insert('X', (Key::KeyX, true));
    my_hashmap.insert('Y', (Key::KeyY, true));
    my_hashmap.insert('Z', (Key::KeyZ, true));

    my_hashmap.insert('0', (Key::Num0, false));
    my_hashmap.insert('1', (Key::Num1, false));
    my_hashmap.insert('2', (Key::Num2, false));
    my_hashmap.insert('3', (Key::Num3, false));
    my_hashmap.insert('4', (Key::Num4, false));
    my_hashmap.insert('5', (Key::Num5, false));
    my_hashmap.insert('6', (Key::Num6, false));
    my_hashmap.insert('7', (Key::Num7, false));
    my_hashmap.insert('8', (Key::Num8, false));
    my_hashmap.insert('9', (Key::Num9, false));

    // my_hashmap.insert(')', (Key::Num0, true));
    // my_hashmap.insert('!', (Key::Num1, true));
    // my_hashmap.insert('@', (Key::Num2, true));
    // my_hashmap.insert('#', (Key::Num3, true));
    // my_hashmap.insert('$', (Key::Num4, true));
    // my_hashmap.insert('%', (Key::Num5, true));
    // my_hashmap.insert('^', (Key::Num6, true));
    // my_hashmap.insert('&', (Key::Num7, true));
    // my_hashmap.insert('*', (Key::Num8, true));
    // my_hashmap.insert('(', (Key::Num9, true));

    my_hashmap.insert('\n', (Key::Return, false));
    my_hashmap.insert(' ', (Key::Space, false));
    my_hashmap.insert('\t', (Key::Tab, false));

    //BackQuote
    my_hashmap.insert('`', (Key::BackQuote, false));
    //my_hashmap.insert('~', (Key::BackQuote, true));

    my_hashmap.insert('-', (Key::Minus, false));
    //my_hashmap.insert('_', (Key::Minus, true));
    my_hashmap.insert('=', (Key::Equal, false));
    //my_hashmap.insert('+', (Key::Equal, true));


    // my_hashmap.insert('[', (Key::LeftBracket, false));
    // my_hashmap.insert('{', (Key::LeftBracket, true));
    // my_hashmap.insert(']', (Key::RightBracket, false));
    // my_hashmap.insert('}', (Key::RightBracket, true));
    // my_hashmap.insert(';', (Key::SemiColon, false));
    // my_hashmap.insert(':', (Key::SemiColon, true));
    // my_hashmap.insert('\'', (Key::Quote, false));
    // my_hashmap.insert('"', (Key::Quote, true));

    /**  
    
    BackSlash \ |

    IntlBackslash / ?

    Comma , <

    Dot . >

    Slash / ?

     */
    return my_hashmap;
}

fn mapped_keys(c: char) -> (Key, bool) {
    match c {
        'a' => (Key::KeyA, false),
        'b' => (Key::KeyB, false),
        'c' => (Key::KeyC, false),
        'd' => (Key::KeyD, false),
        'e' => (Key::KeyE, false),
        'f' => (Key::KeyF, false),
        'g' => (Key::KeyG, false),
        'h' => (Key::KeyH, false),
        'i' => (Key::KeyI, false),
        'j' => (Key::KeyJ, false),
        'k' => (Key::KeyK, false),
        'l' => (Key::KeyL, false),
        'm' => (Key::KeyM, false),
        'n' => (Key::KeyN, false),
        'o' => (Key::KeyO, false),
        'p' => (Key::KeyP, false),
        'q' => (Key::KeyQ, false),
        'r' => (Key::KeyR, false),
        's' => (Key::KeyS, false),
        't' => (Key::KeyT, false),
        'u' => (Key::KeyU, false),
        'v' => (Key::KeyV, false),
        'w' => (Key::KeyW, false),
        'x' => (Key::KeyX, false),
        'y' => (Key::KeyY, false),
        'z' => (Key::KeyZ, false),
        _ => (Key::Unknown(0), false),
    }
}

fn char_to_key(c: char) -> Option<Key> {
    match c {
        'a' => Some(Key::KeyA),
        'b' => Some(Key::KeyB),
        'c' => Some(Key::KeyC),
        'd' => Some(Key::KeyD),
        'e' => Some(Key::KeyE),
        'f' => Some(Key::KeyF),
        'g' => Some(Key::KeyG),
        'h' => Some(Key::KeyH),
        'i' => Some(Key::KeyI),
        'j' => Some(Key::KeyJ),
        'k' => Some(Key::KeyK),
        'l' => Some(Key::KeyL),
        'm' => Some(Key::KeyM),
        'n' => Some(Key::KeyN),
        'o' => Some(Key::KeyO),
        'p' => Some(Key::KeyP),
        'q' => Some(Key::KeyQ),
        'r' => Some(Key::KeyR),
        's' => Some(Key::KeyS),
        't' => Some(Key::KeyT),
        'u' => Some(Key::KeyU),
        'v' => Some(Key::KeyV),
        'w' => Some(Key::KeyW),
        'x' => Some(Key::KeyX),
        'y' => Some(Key::KeyY),
        'z' => Some(Key::KeyZ),

        '0' => Some(Key::Num0),
        '1' => Some(Key::Num1),
        '2' => Some(Key::Num2),
        '3' => Some(Key::Num3),
        '4' => Some(Key::Num4),
        '5' => Some(Key::Num5),
        '6' => Some(Key::Num6),
        '7' => Some(Key::Num7),
        '8' => Some(Key::Num8),
        '9' => Some(Key::Num9),
        ' ' => Some(Key::Space),
        '\n' => Some(Key::Return),
        ',' => Some(Key::Comma),
        '.' => Some(Key::Dot),
        '/' => Some(Key::Slash),
        ';' => Some(Key::SemiColon),
        '\'' => Some(Key::Quote),
        '[' => Some(Key::LeftBracket),
        ']' => Some(Key::RightBracket),
        '\\' => Some(Key::BackSlash),
        '`' => Some(Key::BackQuote),
        '-' => Some(Key::Minus),
        '=' => Some(Key::Equal),
        _ => None, // Return None for characters not covered above
    }
}
