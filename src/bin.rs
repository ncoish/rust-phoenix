extern crate phoenix;

use std::{thread, time};
use std::io::{self, BufRead};

use phoenix::socket::SocketBuilder;

fn hello(message: String) {
    println!("{}", message);
}

fn main() {
    let socket_builder = SocketBuilder::new(String::from("ws://127.0.0.1:4000/socket/websocket"))
        .add_on_open(|| println!("We're open for business!"))
        .add_on_close(|message| println!("Closed with reason: {:?}", message))
        .add_on_message(hello);

    let mut socket_handler = socket_builder.finish();
    match socket_handler.connect() {
        Ok(()) => (),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("We made it here!");
    socket_handler.send(String::from("{ \"topic\": \"lobby\", \"event\": \"phx_join\", \"payload\": {}, \"ref\": \"1\" }"));
    let stdin = io::stdin();
    let io_thread = thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            stdin.lock().read_line(&mut buffer);
            let str_buf = buffer.trim();
            println!("{:?}", str_buf);

            match str_buf {
            "1" => socket_handler.send(String::from("{ \"topic\": \"lobby\", \"event\": \"new_message\", \"payload\": {\"message\": \"hello\", \"name\": \"nick\"}, \"ref\": 1 }")),
            _ => println!("Not recognized as a command"),
           }
        }
    });

    io_thread.join();
    // loop {
    //     thread::sleep(time::Duration::from_millis(5000));
    //     socket_handler.send(String::from("{ \"topic\": \"lobby\", \"event\": \"new_message\", \"payload\": {\"message\": \"hello\", \"name\": \"nick\"}, \"ref\": 1 }"));
    // }
}