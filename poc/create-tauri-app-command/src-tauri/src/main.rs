#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;
use rdev::{listen, Event};
use std::thread;
use std::time::Duration;
#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

fn listen_callback(event: Event) {
    println!("My callback {:?}", event.name);
    // match event.name {
    //     Some(string) => println!("input= {:?}", string),
    //     None => (),
    // }
}

#[tauri::command]
async fn start_listen_keys(){
    println!("KEYS: start_listen_keys");
    if let Err(error) = listen(listen_callback) {
        println!("Error: {:?}", error)
    }
}



#[tauri::command]
async fn start_server() {
    println!("About to bind to the port...");
    let listener = TcpListener::bind("0.0.0.0:14703").await.unwrap_or_else(|e| {
        eprintln!("Error binding to address: {:?}", e);
        panic!("Can't listen");
    });
    println!("Bound to the port successfully...");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
    println!("post starting socket server");
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    println!("handle_connection: ");
    match accept_async(stream).await {
        Ok(ws_stream) => {
            let (mut write, mut read) = ws_stream.split();
            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => match msg {
                        Message::Text(txt) => {
                            println!("msg");
                            if let Err(e) = write.send(Message::Text("Hello from Tauri WebSocket!".to_string())).await {
                                eprintln!("Failed to send message: {:?}", e);
                            }
                        },
                        Message::Close(_) => {
                            if let Err(e) = write.send(Message::Close(None)).await {
                                eprintln!("Failed to send close message: {:?}", e);
                            }
                            break;
                        },
                        _ => {}
                    },
                    Err(e) => {
                        eprintln!("Error reading message: {:?}", e);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error during WebSocket handshake: {:?}", e);
        }
    }
}

fn main_working_websocket() {
    println!("-------------------- Program startup --------------------");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Runtime creation failed: {:?}", e);
            panic!("Error creating runtime");
        });

    println!("created runtime");
    runtime.block_on(start_server());
    println!("run time blocked");
}
fn callback(event: Event) {
    println!("My callback {:?}", event.name);
    match event.name {
        Some(string) => println!("input= {:?}", string),
        None => (),
    }
}
fn main() {
    // Spawn a new thread for rdev's listen function.
    thread::spawn(|| {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }
    });

    // Set up the Tokio runtime and run the Tauri application on the main thread.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Runtime creation failed: {:?}", e);
            panic!("Error creating runtime");
        });

    println!("created runtime");
    runtime.block_on(async {
        println!("blocking async function");
        tauri::Builder::default()
            .setup(|_app| {
                println!("Spawning server task...");
                tokio::spawn(start_server());
                println!("pre ok() function in setup");
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![greet, start_server])
            .run(tauri::generate_context!())
            .unwrap_or_else(|e| {
                eprintln!("Error while running tauri application: {:?}", e);
                panic!("Tauri app run failed");
            });
        println!("blocking async function ends.");
    });
}
