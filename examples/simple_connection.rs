extern crate phoenix;

use std::{thread, time};

use phoenix::socket::SocketBuilder;

fn main() {
    let socket = SocketBuilder::new(String::from("ws://127.0.0.1:4000/socket/websocket"))
        .add_on_open(|| println!("We're open for business!"))
        .add_on_close(|message| println!("Closed with reason: {:?}", message))
        .add_on_message(hello)
        .finish();

    match socket.connect() {
        Ok(()) => (),
        Err(e) => println!("Error: {:?}", e),
    }

    // let chan = socket.channel_builder("lobby", {})
    //     .on("new_message", )


    println!("We made it here!");
    socket.send(String::from("{ \"topic\": \"lobby\", \"event\": \"phx_join\", \"payload\": {}, \"ref\": \"1\" }"));
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        //socket.send(String::from("{ \"topic\": \"lobby\", \"event\": \"phx_join\", \"payload\": {}, \"ref\": 1 }"));
    }
}