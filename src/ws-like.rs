extern crate websocket;
use std::thread;
use websocket::Message;
use websocket::sync::Server;

fn ws_like() {
  let server = Server::bind("127.0.0.1:5555").unwrap();

  for connection in server.filter_map(Result::ok) {
      // Spawn a new thread for each connection.
      thread::spawn(move || {
            let mut client = connection.accept().unwrap();
  
            let message = Message::text("Hello, client!");
            let _ = client.send_message(&message);
  
            // ...
     });
  }
}