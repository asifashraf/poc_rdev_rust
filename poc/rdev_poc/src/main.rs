use rdev::{listen, Event};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;
use std::sync::{Arc, Mutex};

type WebSocketSink = tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>;

fn main() {
    println!("--- Listening to Input ---");

    let clients: Arc<Mutex<Vec<WebSocketSink>>> = Arc::new(Mutex::new(Vec::new()));

    // Spawn a new thread for the WebSocket server.
    // Spawn a new thread for the WebSocket server.
    std::thread::spawn({
        let clients = clients.clone();
        move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            println!("created runtime");
            runtime.block_on(start_server(clients));
            println!("run time blocked");
        }
    });

    // Give the WebSocket server a moment to start
    //std::thread::sleep(std::time::Duration::from_secs(1));

    // This will block the main thread.
    if let Err(error) = listen(move |event| callback(event, &clients)) {

        println!("Error: {:?}", error)
    }
}

async fn start_server(clients: Arc<Mutex<Vec<WebSocketSink>>>) {
    println!("About to bind to the port...");
    let listener = TcpListener::bind("0.0.0.0:14705").await.unwrap_or_else(|e| {
        eprintln!("Error binding to address: {:?}", e);
        panic!("Can't listen");
    });
    println!("Bound to the port successfully...");

    while let Ok((stream, _)) = listener.accept().await {
        let clients = clients.clone();
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                let mut locked_clients = clients.lock().unwrap();
                locked_clients.push(ws_stream);
            }
        });
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
                        Message::Text(_txt) => {
                            if let Err(e) = write.send(Message::Text("Hello from keys WebSocket!".to_string())).await {
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


fn callback(event: Event, clients: &Arc<Mutex<Vec<WebSocketSink>>>) {

    println!("My callback {:?}", event.name);
    match event.name {
        Some(string) => {
            println!("input= {:?}", string);

            // Send the key event to all connected WebSocket clients.
            let clients_copy = clients.clone();
            std::thread::spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                runtime.block_on(async {
                    let mut locked_clients = clients_copy.lock().unwrap();
                    for client in locked_clients.iter_mut() {
                        if let Err(e) = client.send(Message::Text(string.clone())).await {
                            eprintln!("Failed to send message: {:?}", e);
                        }
                    }
                });
            });
        },

        None => (),
    }
}
