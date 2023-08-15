#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[tauri::command]
async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:14703").await.unwrap();
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            socket.write_all(&buf[0..n]).await.unwrap();
        });
    }
}

fn main() {
    // Create Tokio runtime
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Use the runtime to run the Tauri application
    runtime.block_on(async {
        tauri::Builder::default()
            .setup(|_app| {
                // Start the server when the Tauri app starts
                tokio::spawn(start_server());
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![greet, start_server])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}
