extern crate phoenix;

use std::{thread, time};

use phoenix::socket::SocketBuilder;

fn main() {
    let socket_builder = SocketBuilder::new(String::from("ws://127.0.0.1:2794"))
        .add_on_open(|| println!("We're open for business!"))
        .add_on_message(|message| println!("{:?}", message));

    let mut socket = socket_builder.finish();
    //socket.connect();
    println!("We made it here!");
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        socket.send(String::from("Hello there!"));
    }
}