use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
mod foo;

/// A WebSocket echo server
fn main () {
    let url = "localhost:9001";
    let server = TcpListener::bind(url).unwrap();
    println!("websocket echo on :: {}", url);
    foo::hot_reload();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                
                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}