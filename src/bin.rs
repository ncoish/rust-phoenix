extern crate phoenix;

use std::{thread, time};

use phoenix::socket::SocketBuilder;

fn hello(message: String) {
    println!("{}", message);
}

fn main() {
    let socket_builder = SocketBuilder::new(String::from("ws://127.0.0.1:4000/socket/websocket"))
        .add_on_open(|| println!("We're open for business!"))
        .add_on_close(|message| println!("Closed with reason: {:?}", message))
        .add_on_message(hello);

    let mut socket = socket_builder.finish();
    match socket.connect() {
        Ok(()) => (),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("We made it here!");
    socket.send(String::from("{ \"topic\": \"lobby\", \"event\": \"phx_join\", \"payload\": {}, \"ref\": \"1\" }"));
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        //socket.send(String::from("{ \"topic\": \"lobby\", \"event\": \"phx_join\", \"payload\": {}, \"ref\": 1 }"));
    }
}