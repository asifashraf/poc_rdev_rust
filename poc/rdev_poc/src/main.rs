// Importing required modules from the rdev crate to listen for and manage input events.
use rdev::{listen, Event};

// Importing the TcpListener from tokio to listen for incoming TCP connections.
use tokio::net::TcpListener;

// Importing necessary components for managing WebSockets.
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;

// Importing stream and sink utilities from the futures crate.
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;

// Importing synchronization primitives from the standard library.
use std::sync::{Arc, Mutex};

// Type alias for a WebSocket stream connected over a TCP stream.
type WebSocketSink = tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>;

fn main() {
    // Printing a message indicating that the program is listening to input.
    println!("--- Listening to Input ---");

    /* Creating a thread-safe, reference-counted vector of WebSocket clients.
       Arc allows multiple threads to share ownership, and Mutex provides mutual exclusion to ensure safe concurrent access. */
    let clients: Arc<Mutex<Vec<WebSocketSink>>> = Arc::new(Mutex::new(Vec::new()));

    // Spawning a new thread to run the WebSocket server.
    std::thread::spawn({
        let clients = clients.clone();  // Cloning the clients for use within the new thread.
        move || {
            // Creating a new asynchronous runtime for executing asynchronous tasks within this thread.
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            println!("created runtime");
            // Running the start_server function within the async runtime.
            runtime.block_on(start_server(clients));
            println!("run time blocked");
        }
    });

    // Commented out: This line makes the main thread sleep, giving the WebSocket server some time to start.
    //std::thread::sleep(std::time::Duration::from_secs(1));

    // Blocking the main thread to listen for input events. If an error occurs, it is printed.
    if let Err(error) = listen(move |event| callback(event, &clients)) {
        println!("Error: {:?}", error)
    }
}

async fn start_server(clients: Arc<Mutex<Vec<WebSocketSink>>>) {
    // Printing a message indicating the start of the binding process.
    println!("About to bind to the port...");

    /* Binding a TCP listener to a specific address and port. If binding fails, an error message is printed
       and the program panics. */
    let listener = TcpListener::bind("0.0.0.0:14705").await.unwrap_or_else(|e| {
        eprintln!("Error binding to address: {:?}", e);
        panic!("Can't listen");
    });
    println!("Bound to the port successfully...");

    // Continuously listening for incoming connections.
    while let Ok((stream, _)) = listener.accept().await {
        let clients = clients.clone();  // Cloning the clients for use within the async block.

        // Spawning an asynchronous task to handle each incoming connection.
        tokio::spawn(async move {
            // Accepting a WebSocket connection from the incoming TCP stream.
            if let Ok(ws_stream) = accept_async(stream).await {
                let mut locked_clients = clients.lock().unwrap();  // Locking the client list for exclusive access.
                locked_clients.push(ws_stream);  // Adding the new WebSocket client to the list.
            }
        });
    }
    println!("post starting socket server");
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    println!("handle_connection: ");

    // Attempting a WebSocket handshake for the incoming stream.
    match accept_async(stream).await {
        Ok(ws_stream) => {
            // Splitting the WebSocket stream into separate reader and writer halves.
            let (mut write, mut read) = ws_stream.split();

            // Continuously reading messages from the WebSocket connection.
            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => match msg {
                        Message::Text(_txt) => {
                            // If a text message is received, sending a response back to the client.
                            if let Err(e) = write.send(Message::Text("Hello from keys WebSocket!".to_string())).await {
                                eprintln!("Failed to send message: {:?}", e);
                            }
                        },
                        Message::Close(_) => {
                            // If a close message is received, sending a close response and breaking out of the loop.
                            if let Err(e) = write.send(Message::Close(None)).await {
                                eprintln!("Failed to send close message: {:?}", e);
                            }
                            break;
                        },
                        _ => {}  // Ignoring other types of messages.
                    },
                    Err(e) => {
                        // Printing any errors encountered while reading messages.
                        eprintln!("Error reading message: {:?}", e);
                    }
                }
            }
        },
        Err(e) => {
            // Printing any errors encountered during the WebSocket handshake.
            eprintln!("Error during WebSocket handshake: {:?}", e);
        }
    }
}

fn callback(event: Event, clients: &Arc<Mutex<Vec<WebSocketSink>>>) {
    // Printing the name of the received input event.
    println!("My callback {:?}", event.name);

    // Checking if the event has a name.
    match event.name {
        Some(string) => {
            println!("input= {:?}", string);

            // Sending the received key event to all connected WebSocket clients.
            let clients_copy = clients.clone();  // Cloning the clients for use within the new thread.

            // Spawning a new thread to send the message.
            std::thread::spawn(move || {
                // Creating a new asynchronous runtime for the thread.
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                runtime.block_on(async {
                    let mut locked_clients = clients_copy.lock().unwrap();  // Locking the client list for exclusive access.
                    // Iterating over each client and sending the message.
                    for client in locked_clients.iter_mut() {
                        if let Err(e) = client.send(Message::Text(string.clone())).await {
                            eprintln!("Failed to send message: {:?}", e);
                        }
                    }
                });
            });
        },
        None => (),  // Doing nothing if the event has no name.
    }
}
